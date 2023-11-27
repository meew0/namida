use std::{io::Write, path::PathBuf, time::Instant};

use ::libc;
use anyhow::bail;

use super::{OutputMode, Parameter, Session, Transfer};
use crate::{
    message::{ClientToServer, ServerToClient, TransmissionControl},
    types::{BlockIndex, ErrorRate},
};

pub fn ttp_authenticate_client(session: &mut Session, mut secret: String) -> anyhow::Result<()> {
    let ServerToClient::AuthenticationChallenge(mut random) = session.read()? else {
        bail!("Expected authentication challenge");
    };

    let digest: [u8; 16] = crate::common::prepare_proof(&mut random, secret.as_bytes()).into();

    session.write(ClientToServer::AuthenticationResponse(digest))?;

    let ServerToClient::AuthenticationStatus(success) = session.read()? else {
        bail!("Expected authentication status");
    };

    if !success {
        bail!("Authentication failed");
    }

    Ok(())
}

pub fn ttp_negotiate_client(session: &mut Session) -> anyhow::Result<()> {
    let client_revision = crate::common::PROTOCOL_REVISION;
    session.write(client_revision)?;
    let server_revision: u32 = session.read()?;

    if client_revision != server_revision {
        bail!(
            "Protocol negotiation failed: client_revision = {}, server_revision = {}",
            client_revision,
            server_revision
        );
    }

    Ok(())
}

pub fn ttp_open_transfer_client(
    session: &mut Session,
    parameter: &Parameter,
    remote_filename: PathBuf,
    local_filename: PathBuf,
) -> anyhow::Result<()> {
    session.write(ClientToServer::FileRequest(remote_filename.clone()))?;
    let ServerToClient::FileResponseOne(result) = session.read()? else {
        bail!("Expected file response");
    };

    if let Err(err) = result {
        bail!(
            "Server: File does not exist or cannot be transmitted: {:?}",
            err
        );
    }

    // Send our desired parameters...

    session.write(ClientToServer::BlockSize(parameter.block_size))?;
    session.write(ClientToServer::TargetRate(parameter.target_rate))?;
    session.write(ClientToServer::ErrorRate(parameter.error_rate))?;
    session.flush()?;

    session.write(ClientToServer::Slowdown(parameter.slower))?;
    session.write(ClientToServer::Speedup(parameter.faster))?;
    session.flush()?;

    // and get the server's parameters

    session.transfer = Transfer::default();
    session.transfer.remote_filename = Some(remote_filename);
    session.transfer.local_filename = Some(local_filename);

    let ServerToClient::FileSize(file_size) = session.read()? else {
        bail!("Expected file size");
    };
    session.transfer.file_size = file_size;

    let ServerToClient::BlockSize(block_size) = session.read()? else {
        bail!("Expected block size");
    };
    if block_size != parameter.block_size {
        bail!("Block size disagreement");
    }

    let ServerToClient::BlockCount(block_count) = session.read()? else {
        bail!("Expected block count");
    };
    session.transfer.block_count = block_count;

    let ServerToClient::Epoch(epoch) = session.read()? else {
        bail!("Expected epoch");
    };
    session.transfer.epoch = epoch;

    session.transfer.blocks_left = session.transfer.block_count;

    let local_path = session.transfer.local_filename.as_ref().unwrap().as_path();
    if local_path.exists() {
        println!(
            "Warning: overwriting existing file '{}'",
            local_path.display()
        );
    }
    session.transfer.file = Some(
        std::fs::File::options()
            .write(true)
            .create(true)
            .open(local_path)?,
    );

    session.transfer.on_wire_estimate = BlockIndex(
        (0.5f64 * parameter.target_rate.0 as f64 / (parameter.block_size.0 * 8) as f64) as u32,
    );
    session.transfer.on_wire_estimate =
        if session.transfer.block_count < session.transfer.on_wire_estimate {
            session.transfer.block_count
        } else {
            session.transfer.on_wire_estimate
        };

    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::xscript_open_client(
            session, parameter,
        ));
    }

    Ok(())
}

pub fn ttp_open_port_client(
    session: &mut Session,
    parameter: &mut Parameter,
) -> anyhow::Result<()> {
    let udp_socket = session
        .transfer
        .udp_socket
        .insert(super::network::create_udp_socket_client(
            parameter,
            session.server.local_addr()?.is_ipv6(),
        )?);

    let port = udp_socket.local_addr()?.port();

    session.write(ClientToServer::UdpPort(port))?;
    session.flush()?;

    Ok(())
}

pub fn ttp_repeat_retransmit(session: &mut Session) -> anyhow::Result<()> {
    session.transfer.stats.this_retransmits = BlockIndex(0);

    session.transfer.retransmit.next_table.clear();
    for block in &session.transfer.retransmit.previous_table {
        if session.transfer.retransmit.next_table.len() >= 2048 {
            break;
        }

        if !block.is_zero() && !super::command::got_block(session, *block) {
            session.transfer.retransmit.next_table.push(*block);
        }
    }

    let count = BlockIndex(
        session
            .transfer
            .retransmit
            .next_table
            .len()
            .try_into()
            .unwrap(),
    );

    if count >= BlockIndex(2048) {
        let block =
            if session.transfer.block_count < session.transfer.gapless_to_block + BlockIndex(1) {
                session.transfer.block_count
            } else {
                session.transfer.gapless_to_block + BlockIndex(1)
            };

        session.write(TransmissionControl::RestartAt(block))?;

        session.transfer.restart_pending = true;
        session.transfer.restart_lastidx = session
            .transfer
            .retransmit
            .previous_table
            .last()
            .copied()
            .unwrap_or(BlockIndex(0));
        session.transfer.restart_wireclearidx = if session.transfer.block_count
            < (session.transfer.restart_lastidx + session.transfer.on_wire_estimate)
        {
            session.transfer.block_count
        } else {
            session.transfer.restart_lastidx + session.transfer.on_wire_estimate
        };

        session.transfer.retransmit.previous_table.clear();
        session.transfer.retransmit.next_table.clear();

        session.transfer.next_block = block;
        session.transfer.stats.this_retransmits = BlockIndex(2048);
    } else {
        session.transfer.stats.this_retransmits = count;
        session.transfer.stats.total_retransmits = session.transfer.stats.total_retransmits + count;

        if count > BlockIndex(0) {
            let mut retransmits = Vec::with_capacity(session.transfer.retransmit.next_table.len());
            for block_index in &session.transfer.retransmit.next_table {
                retransmits.push(TransmissionControl::Retransmit(*block_index));
            }
            for retransmit in retransmits {
                session.write(retransmit)?;
            }
        }

        session.transfer.retransmit.previous_table.clear();
        session.transfer.retransmit.swap_tables();
    }
    session.flush()?;
    Ok(())
}

pub fn ttp_request_retransmit(session: &mut Session, block: BlockIndex) {
    if super::command::got_block(session, block) {
        return;
    }

    if session.transfer.retransmit.previous_table.len() > 32 * 2048 {
        return;
    }

    session.transfer.retransmit.previous_table.push(block);
}

pub fn ttp_request_stop(session: &mut Session) -> anyhow::Result<()> {
    session.write(TransmissionControl::EndTransmission(0))?;
    Ok(())
}

pub fn ttp_update_stats(
    session: &mut Session,
    parameter: &Parameter,
    iteration: &mut u64,
) -> anyhow::Result<()> {
    let u_mega: f64 = 1_000_000.0;
    let u_giga: f64 = 1_000_000_000.0;

    let delta = Instant::now() - session.transfer.stats.this_time.unwrap();
    let delta_total = Instant::now() - session.transfer.stats.start_time.unwrap();

    let milliseconds = delta_total.subsec_millis();

    let mut temp = delta_total.as_secs();
    let seconds = temp % 60;
    temp /= 60;
    let minutes = temp % 60;
    temp /= 60;
    let hours = temp;

    let d_seconds = delta.as_secs_f64();
    let d_seconds_total = delta_total.as_secs_f64();

    let data_total = parameter.block_size.0 as f64 * session.transfer.stats.total_blocks.0 as f64;
    let data_this = parameter.block_size.0 as f64
        * (session.transfer.stats.total_blocks - session.transfer.stats.this_blocks).0 as f64;
    let data_this_rexmit =
        parameter.block_size.0 as f64 * session.transfer.stats.this_flow_retransmitteds.0 as f64;
    let data_this_goodpt =
        parameter.block_size.0 as f64 * session.transfer.stats.this_flow_originals.0 as f64;

    session.transfer.stats.udp_errors.update();

    let retransmits_fraction = session.transfer.stats.this_retransmits.0 as f64
        / (1.0f64
            + session.transfer.stats.this_retransmits.0 as f64
            + session.transfer.stats.total_blocks.0 as f64
            - session.transfer.stats.this_blocks.0 as f64);
    let ringfill_fraction = session
        .transfer
        .ring_buffer
        .as_ref()
        .map_or(0, |ring| ring.count()) as f64
        / 4096_f64;
    let total_retransmits_fraction = session.transfer.stats.total_retransmits.0 as f64
        / (session.transfer.stats.total_retransmits + session.transfer.stats.total_blocks).0 as f64;

    session.transfer.stats.this_transmit_rate = 8.0f64 * data_this / (d_seconds * u_mega);
    session.transfer.stats.this_retransmit_rate = 8.0f64 * data_this_rexmit / (d_seconds * u_mega);

    let data_total_rate = 8.0f64 * data_total / (d_seconds_total * u_mega);
    let fb = parameter.history as f64 / 100.0f64;
    let ff = 1.0f64 - fb;

    session.transfer.stats.transmit_rate =
        fb * session.transfer.stats.transmit_rate + ff * session.transfer.stats.this_transmit_rate;
    session.transfer.stats.error_rate = fb * session.transfer.stats.error_rate
        + ff * 500 as libc::c_int as f64
            * 100 as libc::c_int as f64
            * (retransmits_fraction + ringfill_fraction);

    session.write(TransmissionControl::SubmitErrorRate(ErrorRate(
        session.transfer.stats.error_rate as u32,
    )))?;

    let stats_flags = format!(
        "{}{}",
        if session.transfer.restart_pending {
            'R' as i32
        } else {
            '-' as i32
        },
        if session
            .transfer
            .ring_buffer
            .as_ref()
            .map_or(false, |ring| ring.is_full())
        {
            'F' as i32
        } else {
            '-' as i32
        },
    );

    let stats_line = format!(
        "{:02}:{:02}:{:02}.{:03} {:4} {:6.2}M {:6.1}Mbps {:5.1}% {:7} {:6.1}G {:6.1}Mbps {:5.1}% {:5} {:5} {:7} {:8} {:8} {}\n",
        hours,
        minutes,
        seconds,
        milliseconds,
        (session.transfer.stats.total_blocks - session.transfer.stats.this_blocks).0,
        session.transfer.stats.this_retransmit_rate,
        session.transfer.stats.this_transmit_rate,
        100.0f64 * retransmits_fraction,
        session.transfer.stats.total_blocks.0,
        data_total / u_giga,
        data_total_rate,
        100.0f64 * total_retransmits_fraction,
        session.transfer.retransmit.previous_table.len(),
        session.transfer.ring_buffer
        .as_ref().map_or(0, |ring| ring.count()),
        session.transfer.blocks_left.0,
        session.transfer.stats.this_retransmits.0,
        session.transfer.stats.udp_errors,
        stats_flags,
    );

    if parameter.verbose_yn {
        if parameter.output_mode == OutputMode::Screen {
            print!("\x1B[2J\x1B[H");
            println!("Current time:   {}", 0); // TODO
            println!(
                "Elapsed time:   {:02}:{:02}:{:02}.{:03}",
                hours, minutes, seconds, milliseconds,
            );
            println!();
            println!("Last interval");
            println!("--------------------------------------------------");
            println!(
                "Blocks count:     {}",
                (session.transfer.stats.total_blocks - session.transfer.stats.this_blocks).0,
            );
            println!("Data transferred: {:02} GB", data_this / u_giga);
            println!(
                "Transfer rate:    {:02} Mbps",
                session.transfer.stats.this_transmit_rate,
            );
            println!(
                "Retransmissions:  {} ({:02}%)",
                session.transfer.stats.this_retransmits.0,
                100.0f64 * retransmits_fraction,
            );
            println!();
            println!("Cumulative");
            println!("--------------------------------------------------");
            println!(
                "Blocks count:     {}",
                session.transfer.stats.total_blocks.0,
            );
            println!("Data transferred: {:02} GB", data_total / u_giga,);
            println!("Transfer rate:    {:02} Mbps", data_total_rate,);
            println!(
                "Retransmissions:  {} ({:02}%)",
                session.transfer.stats.total_retransmits.0,
                100.0f64 * total_retransmits_fraction,
            );
            println!("Flags          :  {}", stats_flags);
            println!();
            println!("OS UDP rx errors: {}", session.transfer.stats.udp_errors);
        } else {
            if *iteration % 23 == 0 {
                println!(
                    "             last_interval                   transfer_total                   buffers      transfer_remaining  OS UDP"
                );
                println!(
                    "time          blk    data       rate rexmit     blk    data       rate rexmit queue  ring     blk   rt_len      err "
                );
            }
            *iteration = iteration.wrapping_add(1);
            print!("{}", stats_line);
        }
        std::io::stdout().flush()?;
    }

    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::xscript_data_log_client(
            session,
            parameter,
            stats_line.as_str(),
        ));
    }

    session.transfer.stats.this_blocks = session.transfer.stats.total_blocks;
    session.transfer.stats.this_retransmits = BlockIndex(0);
    session.transfer.stats.this_flow_originals = BlockIndex(0);
    session.transfer.stats.this_flow_retransmitteds = BlockIndex(0);
    session.transfer.stats.this_time = Some(Instant::now());

    Ok(())
}
