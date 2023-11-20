use ::libc;

use crate::extc;

use super::{ttp_parameter_t, ttp_session_t, ttp_transfer_t};

#[no_mangle]
pub unsafe extern "C" fn xscript_close_client(mut session: *mut ttp_session_t, mut delta: u64) {
    let mut mb_thru: libc::c_double = 0.;
    let mut mb_good: libc::c_double = 0.;
    let mut mb_file: libc::c_double = 0.;
    let mut secs: libc::c_double = 0.;
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    mb_thru = ((*xfer).stats.total_blocks * (*(*session).parameter).block_size) as libc::c_double;
    mb_good = mb_thru
        - ((*xfer).stats.total_recvd_retransmits * (*(*session).parameter).block_size)
            as libc::c_double;
    mb_file = (*xfer).file_size as libc::c_double;
    mb_thru /= 1024.0f64 * 1024.0f64;
    mb_good /= 1024.0f64 * 1024.0f64;
    mb_file /= 1024.0f64 * 1024.0f64;
    secs = delta as libc::c_double / 1e6f64;
    extc::fprintf(
        (*xfer).transcript,
        b"mbyte_transmitted = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_thru,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"mbyte_usable = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_good,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"mbyte_file = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_file,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"duration = %0.2f\n\0" as *const u8 as *const libc::c_char,
        secs,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"throughput = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_thru / secs,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"goodput_with_restarts = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_good / secs,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"file_rate = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_file / secs,
    );
    extc::fclose((*xfer).transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_data_log_client(
    mut session: *mut ttp_session_t,
    mut logline: *const libc::c_char,
) {
    extc::fprintf(
        (*session).transfer.transcript,
        b"%s\0" as *const u8 as *const libc::c_char,
        logline,
    );
    extc::fflush((*session).transfer.transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_data_start_client(
    mut session: *mut ttp_session_t,
    mut epoch: *const extc::timeval,
) {
    extc::fprintf(
        (*session).transfer.transcript,
        b"START %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        (*epoch).tv_sec as libc::c_ulong,
        (*epoch).tv_usec as libc::c_ulong,
    );
    extc::fflush((*session).transfer.transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_data_stop_client(
    mut session: *mut ttp_session_t,
    mut epoch: *const extc::timeval,
) {
    extc::fprintf(
        (*session).transfer.transcript,
        b"STOP %lu.%06lu\n\n\0" as *const u8 as *const libc::c_char,
        (*epoch).tv_sec as libc::c_ulong,
        (*epoch).tv_usec as libc::c_ulong,
    );
    extc::fflush((*session).transfer.transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_open_client(mut session: *mut ttp_session_t) {
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut ttp_parameter_t = (*session).parameter;
    let mut filename: [libc::c_char; 64] = [0; 64];
    crate::common::common::make_transcript_filename(
        filename.as_mut_ptr(),
        (*xfer).epoch,
        b"tsuc\0" as *const u8 as *const libc::c_char,
    );
    (*xfer).transcript = extc::fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if ((*xfer).transcript).is_null() {
        crate::common::error::error_handler(
            b"transcript.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            b"Could not create transcript file\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        return;
    }
    extc::fprintf(
        (*xfer).transcript,
        b"remote_filename = %s\n\0" as *const u8 as *const libc::c_char,
        (*xfer).remote_filename,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"local_filename = %s\n\0" as *const u8 as *const libc::c_char,
        (*xfer).local_filename,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"file_size = %llu\n\0" as *const u8 as *const libc::c_char,
        (*xfer).file_size as u64,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"block_count = %u\n\0" as *const u8 as *const libc::c_char,
        (*xfer).block_count,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"udp_buffer = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).udp_buffer,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"block_size = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).block_size,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"target_rate = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).target_rate,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"error_rate = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).error_rate,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"slower_num = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).slower_num as libc::c_int,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"slower_den = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).slower_den as libc::c_int,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"faster_num = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).faster_num as libc::c_int,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"faster_den = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).faster_den as libc::c_int,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"history = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).history as libc::c_int,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"lossless = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).lossless as libc::c_int,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"losswindow = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).losswindow_ms,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"blockdump = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).blockdump as libc::c_int,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"update_period = %llu\n\0" as *const u8 as *const libc::c_char,
        350000 as libc::c_longlong,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"rexmit_period = %llu\n\0" as *const u8 as *const libc::c_char,
        350000 as libc::c_longlong,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"protocol_version = 0x%x\n\0" as *const u8 as *const libc::c_char,
        crate::common::common::PROTOCOL_REVISION,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"software_version = %s\n\0" as *const u8 as *const libc::c_char,
        b"v1.1 devel cvsbuild 43\0" as *const u8 as *const libc::c_char,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"ipv6 = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).ipv6_yn as libc::c_int,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    extc::fflush((*session).transfer.transcript);
}
