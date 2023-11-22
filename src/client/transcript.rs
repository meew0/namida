use ::libc;

use crate::extc;

use super::{Parameter, Session};

pub unsafe fn xscript_close_client(session: &mut Session, parameter: &Parameter, mut delta: u64) {
    let mut mb_thru: libc::c_double = 0.;
    let mut mb_good: libc::c_double = 0.;
    let mut mb_file: libc::c_double = 0.;
    let mut secs: libc::c_double = 0.;
    mb_thru = (session.transfer.stats.total_blocks * parameter.block_size) as libc::c_double;
    mb_good = mb_thru
        - (session.transfer.stats.total_recvd_retransmits * parameter.block_size) as libc::c_double;
    mb_file = session.transfer.file_size as libc::c_double;
    mb_thru /= 1024.0f64 * 1024.0f64;
    mb_good /= 1024.0f64 * 1024.0f64;
    mb_file /= 1024.0f64 * 1024.0f64;
    secs = delta as libc::c_double / 1e6f64;
    extc::fprintf(
        session.transfer.transcript,
        b"mbyte_transmitted = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_thru,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"mbyte_usable = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_good,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"mbyte_file = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_file,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"duration = %0.2f\n\0" as *const u8 as *const libc::c_char,
        secs,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"throughput = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_thru / secs,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"goodput_with_restarts = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_good / secs,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"file_rate = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_file / secs,
    );
    extc::fclose(session.transfer.transcript);
}
pub unsafe fn xscript_data_log_client(
    session: &mut Session,
    _parameter: &Parameter,
    mut logline: *const libc::c_char,
) {
    extc::fprintf(
        session.transfer.transcript,
        b"%s\0" as *const u8 as *const libc::c_char,
        logline,
    );
    extc::fflush(session.transfer.transcript);
}
pub unsafe fn xscript_data_start_client(
    session: &mut Session,
    _parameter: &Parameter,
    mut epoch: extc::timeval,
) {
    extc::fprintf(
        session.transfer.transcript,
        b"START %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        epoch.tv_sec as libc::c_ulong,
        epoch.tv_usec as libc::c_ulong,
    );
    extc::fflush(session.transfer.transcript);
}
pub unsafe fn xscript_data_stop_client(
    session: &mut Session,
    _parameter: &Parameter,
    mut epoch: extc::timeval,
) {
    extc::fprintf(
        session.transfer.transcript,
        b"STOP %lu.%06lu\n\n\0" as *const u8 as *const libc::c_char,
        epoch.tv_sec as libc::c_ulong,
        epoch.tv_usec as libc::c_ulong,
    );
    extc::fflush(session.transfer.transcript);
}
pub unsafe fn xscript_open_client(session: &mut Session, parameter: &Parameter) {
    let mut filename: [libc::c_char; 64] = [0; 64];
    crate::common::make_transcript_filename(
        filename.as_mut_ptr(),
        session.transfer.epoch,
        b"tsuc\0" as *const u8 as *const libc::c_char,
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
        b"remote_filename = %s\n\0" as *const u8 as *const libc::c_char,
        session.transfer.remote_filename,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"local_filename = %s\n\0" as *const u8 as *const libc::c_char,
        session.transfer.local_filename,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"file_size = %llu\n\0" as *const u8 as *const libc::c_char,
        session.transfer.file_size,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"block_count = %u\n\0" as *const u8 as *const libc::c_char,
        session.transfer.block_count,
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
        b"target_rate = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.target_rate,
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
        b"history = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.history as libc::c_int,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"lossless = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.lossless as libc::c_int,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"losswindow = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.losswindow_ms,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"blockdump = %u\n\0" as *const u8 as *const libc::c_char,
        parameter.blockdump as libc::c_int,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"update_period = %llu\n\0" as *const u8 as *const libc::c_char,
        350000 as libc::c_longlong,
    );
    extc::fprintf(
        session.transfer.transcript,
        b"rexmit_period = %llu\n\0" as *const u8 as *const libc::c_char,
        350000 as libc::c_longlong,
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
