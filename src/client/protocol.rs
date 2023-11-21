use std::{ffi::CStr, path::Path};

use ::libc;
use anyhow::bail;

use super::{Parameter, Retransmission, Retransmit, Session, Statistics, Transfer};
use crate::extc;

pub unsafe fn ttp_authenticate_client(
    session: &mut Session,
    mut secret: String,
) -> anyhow::Result<()> {
    let mut random: [u8; 64] = [0; 64];
    let mut result: u8 = 0;
    let mut status: libc::c_int = 0;
    status = extc::fread(
        random.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        64 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status < 64 as libc::c_int {
        bail!("Could not read authentication challenge from server");
    }
    let mut digest: [u8; 16] =
        crate::common::common::prepare_proof(&mut random, secret.as_bytes()).into();

    status = extc::fwrite(
        digest.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        16 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status < 16 as libc::c_int || extc::fflush(session.server) != 0 {
        bail!("Could not send authentication response");
    }
    status = extc::fread(
        &mut result as *mut u8 as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status < 1 as libc::c_int {
        bail!("Could not read authentication status");
    }
    if result as libc::c_int != 0 as libc::c_int {
        bail!("Authentication failed");
    }
    Ok(())
}

pub unsafe fn ttp_negotiate_client(session: &mut Session) -> anyhow::Result<()> {
    let mut server_revision: u32 = 0;
    let mut client_revision: u32 = extc::__bswap_32(crate::common::common::PROTOCOL_REVISION);
    let mut status: libc::c_int = 0;
    status = extc::fwrite(
        &mut client_revision as *mut u32 as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status < 1 as libc::c_int || extc::fflush(session.server) != 0 {
        bail!("Could not send protocol revision number");
    }
    status = extc::fread(
        &mut server_revision as *mut u32 as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status < 1 as libc::c_int {
        bail!("Could not read protocol revision number");
    }
    if client_revision != server_revision {
        bail!("Protocol negotiation failed");
    }

    Ok(())
}
pub unsafe fn ttp_open_transfer_client(
    session: &mut Session,
    parameter: &Parameter,
    mut remote_filename: *const libc::c_char,
    mut local_filename: *const libc::c_char,
) -> anyhow::Result<()> {
    dbg!();
    let mut result: u8 = 0;
    let mut temp: u32 = 0;
    let mut temp16: u16 = 0;
    let mut status: libc::c_int = 0;
    let mut xfer: *mut Transfer = &mut session.transfer;
    status = extc::fprintf(
        session.server,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        remote_filename,
    );
    if status <= 0 as libc::c_int || extc::fflush(session.server) != 0 {
        bail!("Could not request file");
    }
    status = extc::fread(
        &mut result as *mut u8 as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status < 1 as libc::c_int {
        bail!("Could not read response to file request");
    }
    if result as libc::c_int != 0 as libc::c_int {
        bail!("Server: File does not exist or cannot be transmitted");
    }
    temp = extc::__bswap_32(parameter.block_size);
    if extc::fwrite(
        &mut temp as *mut u32 as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not submit block size");
    }
    temp = extc::__bswap_32(parameter.target_rate);
    if extc::fwrite(
        &mut temp as *mut u32 as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not submit target rate");
    }
    temp = extc::__bswap_32(parameter.error_rate);
    if extc::fwrite(
        &mut temp as *mut u32 as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not submit error rate");
    }
    if extc::fflush(session.server) != 0 {
        bail!("Could not flush control channel");
    }
    temp16 = extc::__bswap_16(parameter.slower_num);
    if extc::fwrite(
        &mut temp16 as *mut u16 as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not submit slowdown numerator");
    }
    temp16 = extc::__bswap_16(parameter.slower_den);
    if extc::fwrite(
        &mut temp16 as *mut u16 as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not submit slowdown denominator");
    }
    temp16 = extc::__bswap_16(parameter.faster_num);
    if extc::fwrite(
        &mut temp16 as *mut u16 as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not submit speedup numerator");
    }
    temp16 = extc::__bswap_16(parameter.faster_den);
    if extc::fwrite(
        &mut temp16 as *mut u16 as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not submit speedup denominator");
    }
    if extc::fflush(session.server) != 0 {
        bail!("Could not flush control channel");
    }
    extc::memset(
        xfer as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Transfer>() as libc::c_ulong,
    );
    (*xfer).remote_filename = remote_filename;
    (*xfer).local_filename = local_filename;
    if extc::fread(
        &mut (*xfer).file_size as *mut u64 as *mut libc::c_void,
        8 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not read file size");
    }
    (*xfer).file_size = crate::common::common::ntohll((*xfer).file_size);
    if extc::fread(
        &mut temp as *mut u32 as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not read block size");
    }
    if extc::__bswap_32(temp) != parameter.block_size {
        bail!("Block size disagreement");
    }
    if extc::fread(
        &mut (*xfer).block_count as *mut u32 as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not read number of blocks");
    }
    (*xfer).block_count = extc::__bswap_32((*xfer).block_count);
    if extc::fread(
        &mut (*xfer).epoch as *mut extc::time_t as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        bail!("Could not read run epoch");
    }
    (*xfer).epoch = extc::__bswap_32((*xfer).epoch as u32) as extc::time_t;
    (*xfer).blocks_left = (*xfer).block_count;
    if extc::access((*xfer).local_filename, 0 as libc::c_int) == 0 {
        extc::printf(
            b"Warning: overwriting existing file '%s'\n\0" as *const u8 as *const libc::c_char,
            local_filename,
        );
    }

    let local_path = Path::new(CStr::from_ptr((*xfer).local_filename).to_str()?);
    (*xfer).file = Some(
        std::fs::File::options()
            .write(true)
            .create(true)
            .open(local_path)?,
    );

    (*xfer).on_wire_estimate = (0.5f64 * parameter.target_rate as libc::c_double
        / (8 as libc::c_int as u32 * parameter.block_size) as libc::c_double)
        as u32;
    (*xfer).on_wire_estimate = if (*xfer).block_count < (*xfer).on_wire_estimate {
        (*xfer).block_count
    } else {
        (*xfer).on_wire_estimate
    };
    if parameter.transcript_yn != 0 {
        super::transcript::xscript_open_client(session, parameter);
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
    status = extc::fwrite(
        port as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) as libc::c_int;
    if status < 1 as libc::c_int || extc::fflush(session.server) != 0 {
        extc::close(session.transfer.udp_fd);
        bail!("Could not send UDP port number");
    }
    Ok(())
}
pub unsafe fn ttp_repeat_retransmit(session: &mut Session) -> anyhow::Result<()> {
    let mut retransmission: [Retransmission; 2048] = [Retransmission {
        request_type: 0,
        block: 0,
        error_rate: 0,
    }; 2048];
    let mut entry: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut block: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut rexmit: *mut Retransmit = &mut session.transfer.retransmit;
    let mut xfer: *mut Transfer = &mut session.transfer;
    extc::memset(
        retransmission.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[Retransmission; 2048]>() as libc::c_ulong,
    );
    (*xfer).stats.this_retransmits = 0 as libc::c_int as u32;
    count = 0 as libc::c_int;
    entry = 0 as libc::c_int;
    while (entry as u32) < (*rexmit).index_max && count < 2048 as libc::c_int {
        block = *((*rexmit).table).offset(entry as isize) as libc::c_int;
        if block != 0 && super::command::got_block(session, block as u32) == 0 {
            *((*rexmit).table).offset(count as isize) = block as u32;
            retransmission[count as usize].request_type =
                extc::__bswap_16(crate::common::common::REQUEST_RETRANSMIT);
            retransmission[count as usize].block = extc::__bswap_32(block as u32);
            count += 1;
            count;
        }
        entry += 1;
        entry;
    }
    if count >= 2048 as libc::c_int {
        block = (if (*xfer).block_count
            < ((*xfer).gapless_to_block).wrapping_add(1 as libc::c_int as u32)
        {
            (*xfer).block_count
        } else {
            ((*xfer).gapless_to_block).wrapping_add(1 as libc::c_int as u32)
        }) as libc::c_int;
        retransmission[0 as libc::c_int as usize].request_type =
            extc::__bswap_16(crate::common::common::REQUEST_RESTART);
        retransmission[0 as libc::c_int as usize].block = extc::__bswap_32(block as u32);
        status = extc::fwrite(
            &mut *retransmission
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut Retransmission
                as *const libc::c_void,
            ::core::mem::size_of::<Retransmission>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            session.server,
        ) as libc::c_int;
        if status <= 0 as libc::c_int {
            bail!("Could not send restart-at request");
        }
        (*xfer).restart_pending = 1 as libc::c_int as u8;
        (*xfer).restart_lastidx = *((*rexmit).table)
            .offset(((*rexmit).index_max).wrapping_sub(1 as libc::c_int as u32) as isize);
        (*xfer).restart_wireclearidx = if (*xfer).block_count
            < ((*xfer).restart_lastidx).wrapping_add((*xfer).on_wire_estimate)
        {
            (*xfer).block_count
        } else {
            ((*xfer).restart_lastidx).wrapping_add((*xfer).on_wire_estimate)
        };
        (*rexmit).index_max = 0 as libc::c_int as u32;
        (*xfer).next_block = block as u32;
        (*xfer).stats.this_retransmits = 2048 as libc::c_int as u32;
    } else {
        (*rexmit).index_max = count as u32;
        (*xfer).stats.this_retransmits = count as u32;
        (*xfer).stats.total_retransmits =
            ((*xfer).stats.total_retransmits).wrapping_add(count as u32);
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
    let mut rexmit: *mut Retransmit = &mut session.transfer.retransmit;
    if super::command::got_block(session, block) != 0 {
        return Ok(());
    }
    if (*rexmit).index_max >= (*rexmit).table_size {
        if (*rexmit).index_max >= (32 as libc::c_int * 2048 as libc::c_int) as u32 {
            return Ok(());
        }
        ptr = extc::realloc(
            (*rexmit).table as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<u32>() as libc::c_ulong)
                .wrapping_mul((*rexmit).table_size as libc::c_ulong),
        ) as *mut u32;
        if ptr.is_null() {
            bail!("Could not grow retransmission table");
        }
        (*rexmit).table = ptr;
        extc::memset(
            ((*rexmit).table).offset((*rexmit).table_size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<u32>() as libc::c_ulong)
                .wrapping_mul((*rexmit).table_size as libc::c_ulong),
        );
        (*rexmit).table_size *= 2 as libc::c_int as u32;
    }
    *((*rexmit).table).offset((*rexmit).index_max as isize) = block;
    (*rexmit).index_max = ((*rexmit).index_max).wrapping_add(1);
    (*rexmit).index_max;
    Ok(())
}
pub unsafe fn ttp_request_stop(session: &mut Session) -> anyhow::Result<()> {
    let mut retransmission: Retransmission = {
        Retransmission {
            request_type: 0 as libc::c_int as u16,
            block: 0 as libc::c_int as u32,
            error_rate: 0 as libc::c_int as u32,
        }
    };
    let mut status: libc::c_int = 0;
    retransmission.request_type = extc::__bswap_16(crate::common::common::REQUEST_STOP);
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
    let mut stats: *mut Statistics = &mut session.transfer.stats;
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
    delta = crate::common::common::get_usec_since(&mut (*stats).this_time);
    temp = crate::common::common::get_usec_since(&mut (*stats).start_time);
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
    data_total = parameter.block_size as libc::c_double * (*stats).total_blocks as libc::c_double;
    data_this = parameter.block_size as libc::c_double
        * ((*stats).total_blocks).wrapping_sub((*stats).this_blocks) as libc::c_double;
    data_this_rexmit = parameter.block_size as libc::c_double
        * (*stats).this_flow_retransmitteds as libc::c_double;
    data_this_goodpt =
        parameter.block_size as libc::c_double * (*stats).this_flow_originals as libc::c_double;
    (*stats).this_udp_errors = crate::common::common::get_udp_in_errors();
    retransmits_fraction = (*stats).this_retransmits as libc::c_double
        / (1.0f64
            + (*stats).this_retransmits as libc::c_double
            + (*stats).total_blocks as libc::c_double
            - (*stats).this_blocks as libc::c_double);
    ringfill_fraction = session
        .transfer
        .ring_buffer
        .as_ref()
        .map_or(0, |ring| ring.count()) as f64
        / 4096_f64;
    total_retransmits_fraction = ((*stats).total_retransmits
        / ((*stats).total_retransmits).wrapping_add((*stats).total_blocks))
        as libc::c_double;
    (*stats).this_transmit_rate = 8.0f64 * data_this / (d_seconds * u_mega);
    (*stats).this_retransmit_rate = 8.0f64 * data_this_rexmit / (d_seconds * u_mega);
    data_total_rate = 8.0f64 * data_total / (d_seconds_total * u_mega);
    fb = parameter.history as libc::c_int as libc::c_double / 100.0f64;
    ff = 1.0f64 - fb;
    (*stats).transmit_rate = fb * (*stats).transmit_rate + ff * (*stats).this_transmit_rate;
    (*stats).error_rate = fb * (*stats).error_rate
        + ff * 500 as libc::c_int as libc::c_double
            * 100 as libc::c_int as libc::c_double
            * (retransmits_fraction + ringfill_fraction);
    extc::memset(
        &mut retransmission as *mut Retransmission as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Retransmission>() as libc::c_ulong,
    );
    retransmission.request_type = extc::__bswap_16(crate::common::common::REQUEST_ERROR_RATE);
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
        ((*stats).total_blocks).wrapping_sub((*stats).this_blocks),
        (*stats).this_retransmit_rate,
        (*stats).this_transmit_rate,
        100.0f64 * retransmits_fraction,
        session.transfer.stats.total_blocks,
        data_total / u_giga,
        data_total_rate,
        100.0f64 * total_retransmits_fraction,
        session.transfer.retransmit.index_max,
        session.transfer.ring_buffer
        .as_ref().map_or(0, |ring| ring.count()),
        session.transfer.blocks_left,
        (*stats).this_retransmits,
        ((*stats).this_udp_errors).wrapping_sub((*stats).start_udp_errors),
        stats_flags.as_mut_ptr(),
    );
    if parameter.verbose_yn != 0 {
        if parameter.output_mode as libc::c_int == 0 as libc::c_int {
            extc::printf(b"\x1B[2J\x1B[H\0" as *const u8 as *const libc::c_char);
            extc::printf(
                b"Current time:   %s\n\0" as *const u8 as *const libc::c_char,
                extc::ctime(&mut now_epoch),
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
                ((*stats).total_blocks).wrapping_sub((*stats).this_blocks),
            );
            extc::printf(
                b"Data transferred: %0.2f GB\n\0" as *const u8 as *const libc::c_char,
                data_this / u_giga,
            );
            extc::printf(
                b"Transfer rate:    %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                (*stats).this_transmit_rate,
            );
            extc::printf(
                b"Retransmissions:  %u (%0.2f%%)\n\n\0" as *const u8 as *const libc::c_char,
                (*stats).this_retransmits,
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
                (*stats).total_retransmits,
                100.0f64 * total_retransmits_fraction,
            );
            extc::printf(
                b"Flags          :  %s\n\n\0" as *const u8 as *const libc::c_char,
                stats_flags.as_mut_ptr(),
            );
            extc::printf(
                b"OS UDP rx errors: %llu\n\0" as *const u8 as *const libc::c_char,
                ((*stats).this_udp_errors).wrapping_sub((*stats).start_udp_errors),
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
    if parameter.transcript_yn != 0 {
        super::transcript::xscript_data_log_client(session, parameter, stats_line.as_mut_ptr());
    }
    (*stats).this_blocks = (*stats).total_blocks;
    (*stats).this_retransmits = 0 as libc::c_int as u32;
    (*stats).this_flow_originals = 0 as libc::c_int as u32;
    (*stats).this_flow_retransmitteds = 0 as libc::c_int as u32;
    extc::gettimeofday(
        &mut (*stats).this_time,
        std::ptr::null_mut::<libc::c_void>(),
    );
    Ok(())
}
