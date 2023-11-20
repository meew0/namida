use crate::extc;
use ::libc;

#[no_mangle]
pub unsafe extern "C" fn xscript_close_server(
    mut session: *mut super::ttp_session_t,
    mut delta: u64,
) {
    let mut xfer: *mut super::ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut super::ttp_parameter_t = (*session).parameter;
    extc::fprintf(
        (*xfer).transcript,
        b"mb_transmitted = %0.2f\n\0" as *const u8 as *const libc::c_char,
        (*param).file_size as libc::c_double / (1024.0f64 * 1024.0f64),
    );
    extc::fprintf(
        (*xfer).transcript,
        b"duration = %0.2f\n\0" as *const u8 as *const libc::c_char,
        delta as libc::c_double / 1000000.0f64,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"throughput = %0.2f\n\0" as *const u8 as *const libc::c_char,
        (*param).file_size as libc::c_double * 8.0f64
            / (delta as libc::c_double
                * 1e-6f64
                * 1024 as libc::c_int as libc::c_double
                * 1024 as libc::c_int as libc::c_double),
    );
    extc::fclose((*xfer).transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_data_log_server(
    mut session: *mut super::ttp_session_t,
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
pub unsafe extern "C" fn xscript_data_start_server(
    mut session: *mut super::ttp_session_t,
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
pub unsafe extern "C" fn xscript_data_stop_server(
    mut session: *mut super::ttp_session_t,
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
pub unsafe extern "C" fn xscript_open_server(mut session: *mut super::ttp_session_t) {
    let mut xfer: *mut super::ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut super::ttp_parameter_t = (*session).parameter;
    let mut filename: [libc::c_char; 64] = [0; 64];
    crate::common::common::make_transcript_filename(
        filename.as_mut_ptr(),
        (*param).epoch,
        b"tsus\0" as *const u8 as *const libc::c_char,
    );
    (*xfer).transcript = extc::fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if ((*xfer).transcript).is_null() {
        crate::common::error::error_handler(
            b"transcript.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            b"Could not create transcript file\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        return;
    }
    extc::fprintf(
        (*xfer).transcript,
        b"filename = %s\n\0" as *const u8 as *const libc::c_char,
        (*xfer).filename,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"file_size = %llu\n\0" as *const u8 as *const libc::c_char,
        (*param).file_size as u64,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"block_count = %llu\n\0" as *const u8 as *const libc::c_char,
        (*param).block_count as u64,
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
        b"target_rate = %llu\n\0" as *const u8 as *const libc::c_char,
        (*param).target_rate as u64,
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
        b"ipd_time = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).ipd_time,
    );
    extc::fprintf(
        (*xfer).transcript,
        b"ipd_current = %u\n\0" as *const u8 as *const libc::c_char,
        (*xfer).ipd_current as u32,
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
