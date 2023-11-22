use crate::extc;
use ::libc;

use super::{Parameter, Session};

pub unsafe fn xscript_close_server(session: &mut Session, parameter: &Parameter, mut delta: u64) {
    extc::fprintf(
        session.transfer.transcript,
        b"mb_transmitted = %0.2f\n\0" as *const u8 as *const libc::c_char,
        parameter.file_size as libc::c_double / (1024.0f64 * 1024.0f64),
    );
    extc::fprintf(
        session.transfer.transcript,
        b"duration = %0.2f\n\0" as *const u8 as *const libc::c_char,
        delta as libc::c_double / 1000000.0f64,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"throughput = %0.2f\n\0" as *const u8 as *const libc::c_char,
        parameter.file_size as libc::c_double * 8.0f64
            / (delta as libc::c_double
                * 1e-6f64
                * 1024 as libc::c_int as libc::c_double
                * 1024 as libc::c_int as libc::c_double),
    );
    extc::fclose(session.transfer.transcript);
}
pub unsafe fn xscript_data_log_server(session: &mut Session, mut logline: *const libc::c_char) {
    extc::fprintf(
        session.transfer.transcript,
        b"%s\0" as *const u8 as *const libc::c_char,
        logline,
    );
    extc::fflush(session.transfer.transcript);
}
pub unsafe fn xscript_data_start_server(session: &mut Session, mut epoch: *const extc::timeval) {
    extc::fprintf(
        session.transfer.transcript,
        b"START %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        (*epoch).tv_sec as libc::c_ulong,
        (*epoch).tv_usec as libc::c_ulong,
    );
    extc::fflush(session.transfer.transcript);
}
pub unsafe fn xscript_data_stop_server(session: &mut Session, mut epoch: *const extc::timeval) {
    extc::fprintf(
        session.transfer.transcript,
        b"STOP %lu.%06lu\n\n\0" as *const u8 as *const libc::c_char,
        (*epoch).tv_sec as libc::c_ulong,
        (*epoch).tv_usec as libc::c_ulong,
    );
    extc::fflush(session.transfer.transcript);
}
pub unsafe fn xscript_open_server(session: &mut Session, parameter: &Parameter) {
    let mut filename: [libc::c_char; 64] = [0; 64];
    crate::common::make_transcript_filename(
        filename.as_mut_ptr(),
        parameter.epoch,
        b"tsus\0" as *const u8 as *const libc::c_char,
    );
    session.transfer.transcript = extc::fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if (session.transfer.transcript).is_null() {
        println!("WARNING: Could not create transcript file");
        return;
    }
    extc::fprintf(
        session.transfer.transcript,
        b"filename = %s\n\0" as *const u8 as *const libc::c_char,
        session.transfer.filename,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"file_size = %llu\n\0" as *const u8 as *const libc::c_char,
        parameter.file_size,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"block_count = %llu\n\0" as *const u8 as *const libc::c_char,
        parameter.block_count as u64,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"udp_buffer = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.udp_buffer,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"block_size = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.block_size,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"target_rate = %llu\n\0" as *const u8 as *const libc::c_char,
        parameter.target_rate as u64,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"error_rate = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.error_rate,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"slower_num = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.slower_num as libc::c_int,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"slower_den = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.slower_den as libc::c_int,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"faster_num = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.faster_num as libc::c_int,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"faster_den = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.faster_den as libc::c_int,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"ipd_time = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.ipd_time,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"ipd_current = %u\n\0" as *const u8 as *const libc::c_char,
        session.transfer.ipd_current as u32,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"protocol_version = 0x%x\n\0" as *const u8 as *const libc::c_char,
        crate::common::PROTOCOL_REVISION,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"software_version = %s\n\0" as *const u8 as *const libc::c_char,
        b"v1.1 devel cvsbuild 43\0" as *const u8 as *const libc::c_char,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"ipv6 = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.ipv6_yn as libc::c_int,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    extc::fflush(session.transfer.transcript);
}
