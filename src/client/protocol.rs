use std::{
    ffi::{CStr, CString},
    path::Path,
};

use ::libc;
use anyhow::bail;

use super::{Parameter, Session, Transfer};
use crate::{
    extc,
    message::{ClientToServer, ServerToClient},
    types::{BlockIndex, ErrorRate, Retransmission},
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
    remote_filename: String,
    local_filename: String,
) -> anyhow::Result<()> {
    session.write(ClientToServer::FileRequest(remote_filename))?;
    let ServerToClient::FileResponseOne(result) = session.read()?;

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
    let local_filename = session.transfer.local_filename.insert(local_filename);

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

    let local_path = Path::new(local_filename);
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

pub unsafe fn ttp_open_port_client(
    session: &mut Session,
    parameter: &mut Parameter,
) -> anyhow::Result<()> {
    let mut udp_address: extc::sockaddr = extc::sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
    };
    let mut udp_length: libc::c_uint =
        ::core::mem::size_of::<extc::sockaddr>() as libc::c_ulong as libc::c_uint;
    let mut status: libc::c_int = 0;
    let mut port: *mut u16 = std::ptr::null_mut::<u16>();
    session.transfer.udp_fd = super::network::create_udp_socket_client(parameter)?;
    extc::memset(
        &mut udp_address as *mut extc::sockaddr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<extc::sockaddr>() as libc::c_ulong,
    );
    extc::getsockname(
        session.transfer.udp_fd,
        extc::__SOCKADDR_ARG {
            __sockaddr__: &mut udp_address as *mut extc::sockaddr,
        },
        &mut udp_length,
    );
    port = if parameter.ipv6_yn as libc::c_int != 0 {
        &mut (*(&mut udp_address as *mut extc::sockaddr as *mut extc::sockaddr_in6)).sin6_port
    } else {
        &mut (*(&mut udp_address as *mut extc::sockaddr as *mut extc::sockaddr_in)).sin_port
    };

    session.write(ClientToServer::UdpPort(port))?;
    session.flush()?;

    Ok(())
}
pub unsafe fn ttp_repeat_retransmit(session: &mut Session) -> anyhow::Result<()> {
    let mut status: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    session.transfer.stats.this_retransmits = 0 as libc::c_int as u32;

    session.transfer.retransmit.next_table.clear();
    for block in &session.transfer.retransmit.previous_table {
        if session.transfer.retransmit.next_table.len() >= 2048 {
            break;
        }

        if !block.is_zero() && !super::command::got_block(session, *block) {
            session.transfer.retransmit.next_table.push(*block);
        }
    }

    if count >= 2048 {
        let block =
            if session.transfer.block_count < session.transfer.gapless_to_block + BlockIndex(1) {
                session.transfer.block_count
            } else {
                session.transfer.gapless_to_block + BlockIndex(1)
            };

        session.write(ClientToServer::RestartAt(block))?;

        session.transfer.restart_pending = 1 as libc::c_int as u8;
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
        session.transfer.retransmit.swap_tables();

        session.transfer.next_block = block;
        session.transfer.stats.this_retransmits = 2048;
    } else {
        session.transfer.retransmit.index_max = count as u32;
        session.transfer.stats.this_retransmits = count as u32;
        session.transfer.stats.total_retransmits =
            (session.transfer.stats.total_retransmits).wrapping_add(count as u32);
        if count > 0 as libc::c_int {
            status = extc::fwrite(
                retransmission.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<Retransmission>() as libc::c_ulong,
                count as libc::c_ulong,
                session.server,
            ) as libc::c_int;
            if status <= 0 as libc::c_int {
                bail!("Could not send retransmit requests");
            }
        }
    }
    if extc::fflush(session.server) != 0 {
        bail!("Could not flush retransmit requests");
    }
    Ok(())
}

pub unsafe fn ttp_request_retransmit(session: &mut Session, mut block: u32) -> anyhow::Result<()> {
    let mut ptr: *mut u32 = std::ptr::null_mut::<u32>();
    if !super::command::got_block(session, block) {
        return Ok(());
    }
    if session.transfer.retransmit.index_max >= session.transfer.retransmit.table_size {
        if session.transfer.retransmit.index_max >= (32 as libc::c_int * 2048 as libc::c_int) as u32
        {
            return Ok(());
        }
        ptr = extc::realloc(
            session.transfer.retransmit.table as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<u32>() as libc::c_ulong)
                .wrapping_mul(session.transfer.retransmit.table_size as libc::c_ulong),
        ) as *mut u32;
        if ptr.is_null() {
            bail!("Could not grow retransmission table");
        }
        session.transfer.retransmit.table = ptr;
        extc::memset(
            (session.transfer.retransmit.table)
                .offset(session.transfer.retransmit.table_size as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<u32>() as libc::c_ulong)
                .wrapping_mul(session.transfer.retransmit.table_size as libc::c_ulong),
        );
        session.transfer.retransmit.table_size *= 2 as libc::c_int as u32;
    }
    *(session.transfer.retransmit.table).offset(session.transfer.retransmit.index_max as isize) =
        block;
    session.transfer.retransmit.index_max = (session.transfer.retransmit.index_max).wrapping_add(1);
    Ok(())
}

pub unsafe fn ttp_request_stop(session: &mut Session) -> anyhow::Result<()> {
    let mut retransmission: Retransmission = Retransmission {
        request_type: 0,
        block: 0,
        error_rate: 0,
    };
    let mut status: libc::c_int = 0;
    retransmission.request_type = extc::__bswap_16(crate::common::REQUEST_STOP);
    status = extc::fwrite(
        &mut retransmission as *mut Retransmission as *const libc::c_void,
        ::core::mem::size_of::<Retransmission>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status <= 0 as libc::c_int || extc::fflush(session.server) != 0 {
        bail!("Could not request end of transmission");
    }
    Ok(())
}
pub unsafe fn ttp_update_stats(session: &mut Session, parameter: &Parameter) -> anyhow::Result<()> {
    let mut now_epoch: extc::time_t = extc::time(std::ptr::null_mut::<extc::time_t>());
    let mut delta: u64 = 0;
    let mut d_seconds: libc::c_double = 0.;
    let mut delta_total: u64 = 0;
    let mut d_seconds_total: libc::c_double = 0.;
    let mut temp: u64 = 0;
    let mut hours: libc::c_int = 0;
    let mut minutes: libc::c_int = 0;
    let mut seconds: libc::c_int = 0;
    let mut milliseconds: libc::c_int = 0;
    let mut data_total: libc::c_double = 0.;
    let mut data_total_rate: libc::c_double = 0.;
    let mut data_this: libc::c_double = 0.;
    let mut data_this_rexmit: libc::c_double = 0.;
    let mut data_this_goodpt: libc::c_double = 0.;
    let mut retransmits_fraction: libc::c_double = 0.;
    let mut total_retransmits_fraction: libc::c_double = 0.;
    let mut ringfill_fraction: libc::c_double = 0.;
    let mut retransmission: Retransmission = Retransmission {
        request_type: 0,
        block: 0,
        error_rate: 0,
    };
    let mut status: libc::c_int = 0;
    static mut iteration: u32 = 0 as libc::c_int as u32;
    static mut stats_line: [libc::c_char; 128] = [0; 128];
    static mut stats_flags: [libc::c_char; 8] = [0; 8];
    let mut ff: libc::c_double = 0.;
    let mut fb: libc::c_double = 0.;
    let u_mega: libc::c_double = (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double;
    let u_giga: libc::c_double =
        (1024 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double;
    delta = crate::common::get_usec_since(&mut session.transfer.stats.this_time);
    temp = crate::common::get_usec_since(&mut session.transfer.stats.start_time);
    delta_total = temp;
    milliseconds =
        (temp % 1000000 as libc::c_int as u64 / 1000 as libc::c_int as u64) as libc::c_int;
    temp /= 1000000 as libc::c_int as u64;
    seconds = (temp % 60 as libc::c_int as u64) as libc::c_int;
    temp /= 60 as libc::c_int as u64;
    minutes = (temp % 60 as libc::c_int as u64) as libc::c_int;
    temp /= 60 as libc::c_int as u64;
    hours = temp as libc::c_int;
    d_seconds = delta as libc::c_double / 1e6f64;
    d_seconds_total = delta_total as libc::c_double / 1e6f64;
    data_total = parameter.block_size as libc::c_double
        * session.transfer.stats.total_blocks as libc::c_double;
    data_this = parameter.block_size as libc::c_double
        * (session.transfer.stats.total_blocks).wrapping_sub(session.transfer.stats.this_blocks)
            as libc::c_double;
    data_this_rexmit = parameter.block_size as libc::c_double
        * session.transfer.stats.this_flow_retransmitteds as libc::c_double;
    data_this_goodpt = parameter.block_size as libc::c_double
        * session.transfer.stats.this_flow_originals as libc::c_double;
    session.transfer.stats.this_udp_errors = crate::common::get_udp_in_errors();
    retransmits_fraction = session.transfer.stats.this_retransmits as libc::c_double
        / (1.0f64
            + session.transfer.stats.this_retransmits as libc::c_double
            + session.transfer.stats.total_blocks as libc::c_double
            - session.transfer.stats.this_blocks as libc::c_double);
    ringfill_fraction = session
        .transfer
        .ring_buffer
        .as_ref()
        .map_or(0, |ring| ring.count()) as f64
        / 4096_f64;
    total_retransmits_fraction = (session.transfer.stats.total_retransmits
        / (session.transfer.stats.total_retransmits)
            .wrapping_add(session.transfer.stats.total_blocks))
        as libc::c_double;
    session.transfer.stats.this_transmit_rate = 8.0f64 * data_this / (d_seconds * u_mega);
    session.transfer.stats.this_retransmit_rate = 8.0f64 * data_this_rexmit / (d_seconds * u_mega);
    data_total_rate = 8.0f64 * data_total / (d_seconds_total * u_mega);
    fb = parameter.history as libc::c_int as libc::c_double / 100.0f64;
    ff = 1.0f64 - fb;
    session.transfer.stats.transmit_rate =
        fb * session.transfer.stats.transmit_rate + ff * session.transfer.stats.this_transmit_rate;
    session.transfer.stats.error_rate = fb * session.transfer.stats.error_rate
        + ff * 500 as libc::c_int as libc::c_double
            * 100 as libc::c_int as libc::c_double
            * (retransmits_fraction + ringfill_fraction);
    retransmission.request_type = extc::__bswap_16(crate::common::REQUEST_ERROR_RATE);
    retransmission.error_rate = extc::__bswap_32(session.transfer.stats.error_rate as u64 as u32);
    status = extc::fwrite(
        &mut retransmission as *mut Retransmission as *const libc::c_void,
        ::core::mem::size_of::<Retransmission>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status <= 0 as libc::c_int || extc::fflush(session.server) != 0 {
        bail!("Could not send error rate information");
    }
    extc::printf(
        stats_flags.as_mut_ptr(),
        b"%c%c\0" as *const u8 as *const libc::c_char,
        if session.transfer.restart_pending as libc::c_int != 0 {
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
    extc::printf(
        stats_line.as_mut_ptr(),
        b"%02d:%02d:%02d.%03d %4u %6.2fM %6.1fMbps %5.1f%% %7u %6.1fG %6.1fMbps %5.1f%% %5d %5d %7u %8u %8Lu %s\n\0"
            as *const u8 as *const libc::c_char,
        hours,
        minutes,
        seconds,
        milliseconds,
        (session.transfer.stats.total_blocks).wrapping_sub(session.transfer.stats.this_blocks),
        session.transfer.stats.this_retransmit_rate,
        session.transfer.stats.this_transmit_rate,
        100.0f64 * retransmits_fraction,
        session.transfer.stats.total_blocks,
        data_total / u_giga,
        data_total_rate,
        100.0f64 * total_retransmits_fraction,
        session.transfer.retransmit.index_max,
        session.transfer.ring_buffer
        .as_ref().map_or(0, |ring| ring.count()),
        session.transfer.blocks_left,
        session.transfer.stats.this_retransmits,
        (session.transfer.stats.this_udp_errors).wrapping_sub(session.transfer.stats.start_udp_errors),
        stats_flags.as_mut_ptr(),
    );
    if parameter.verbose_yn {
        if parameter.output_mode as libc::c_int == 0 as libc::c_int {
            extc::printf(b"\x1B[2J\x1B[H\0" as *const u8 as *const libc::c_char);
            extc::printf(
                b"Current time:   %s\n\0" as *const u8 as *const libc::c_char,
                extc::ctime(&now_epoch),
            );
            extc::printf(
                b"Elapsed time:   %02d:%02d:%02d.%03d\n\n\0" as *const u8 as *const libc::c_char,
                hours,
                minutes,
                seconds,
                milliseconds,
            );
            extc::printf(
                b"Last interval\n--------------------------------------------------\n\0"
                    as *const u8 as *const libc::c_char,
            );
            extc::printf(
                b"Blocks count:     %u\n\0" as *const u8 as *const libc::c_char,
                (session.transfer.stats.total_blocks)
                    .wrapping_sub(session.transfer.stats.this_blocks),
            );
            extc::printf(
                b"Data transferred: %0.2f GB\n\0" as *const u8 as *const libc::c_char,
                data_this / u_giga,
            );
            extc::printf(
                b"Transfer rate:    %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                session.transfer.stats.this_transmit_rate,
            );
            extc::printf(
                b"Retransmissions:  %u (%0.2f%%)\n\n\0" as *const u8 as *const libc::c_char,
                session.transfer.stats.this_retransmits,
                100.0f64 * retransmits_fraction,
            );
            extc::printf(
                b"Cumulative\n--------------------------------------------------\n\0" as *const u8
                    as *const libc::c_char,
            );
            extc::printf(
                b"Blocks count:     %u\n\0" as *const u8 as *const libc::c_char,
                session.transfer.stats.total_blocks,
            );
            extc::printf(
                b"Data transferred: %0.2f GB\n\0" as *const u8 as *const libc::c_char,
                data_total / u_giga,
            );
            extc::printf(
                b"Transfer rate:    %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                data_total_rate,
            );
            extc::printf(
                b"Retransmissions:  %u (%0.2f%%)\n\0" as *const u8 as *const libc::c_char,
                session.transfer.stats.total_retransmits,
                100.0f64 * total_retransmits_fraction,
            );
            extc::printf(
                b"Flags          :  %s\n\n\0" as *const u8 as *const libc::c_char,
                stats_flags.as_mut_ptr(),
            );
            extc::printf(
                b"OS UDP rx errors: %llu\n\0" as *const u8 as *const libc::c_char,
                (session.transfer.stats.this_udp_errors)
                    .wrapping_sub(session.transfer.stats.start_udp_errors),
            );
        } else {
            let fresh1 = iteration;
            iteration = iteration.wrapping_add(1);
            if fresh1 % 23 as libc::c_int as u32 == 0 {
                extc::printf(
                    b"             last_interval                   transfer_total                   buffers      transfer_remaining  OS UDP\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::printf(
                    b"time          blk    data       rate rexmit     blk    data       rate rexmit queue  ring     blk   rt_len      err \n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            extc::printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                stats_line.as_mut_ptr(),
            );
        }
        extc::fflush(extc::stdout);
    }
    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::xscript_data_log_client(
            session,
            parameter,
            CStr::from_ptr(stats_line.as_mut_ptr()).to_str().unwrap(),
        ));
    }
    session.transfer.stats.this_blocks = session.transfer.stats.total_blocks;
    session.transfer.stats.this_retransmits = 0 as libc::c_int as u32;
    session.transfer.stats.this_flow_originals = 0 as libc::c_int as u32;
    session.transfer.stats.this_flow_retransmitteds = 0 as libc::c_int as u32;
    extc::gettimeofday(
        &mut session.transfer.stats.this_time,
        std::ptr::null_mut::<libc::c_void>(),
    );
    Ok(())
}
