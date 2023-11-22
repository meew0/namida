use std::io::Write;
use std::path::Path;

use crate::extc;

use super::{Parameter, Session};

pub fn xscript_close_client(
    session: &mut Session,
    parameter: &Parameter,
    delta: u64,
) -> anyhow::Result<()> {
    // File sizes in megabytes, not mibibytes as Tsunami used
    let mb_thru = (session.transfer.stats.total_blocks * parameter.block_size) as f64 / 1000000.0;
    let mb_good = mb_thru
        - (session.transfer.stats.total_recvd_retransmits * parameter.block_size) as f64
            / 1000000.0;
    let mb_file = session.transfer.file_size as f64 / 1000000.0;

    // Microseconds to seconds
    let secs = delta as f64 / 1000000.0;

    let transcript = session.transfer.transcript.as_mut().unwrap();

    write!(transcript, "mbyte_transmitted = {:0>.2}\n", mb_thru)?;
    write!(transcript, "mbyte_usable = {:0>.2}\n", mb_good)?;
    write!(transcript, "mbyte_file = {:0>.2}\n", mb_file)?;
    write!(transcript, "duration = {:0>.2}\n", secs)?;
    write!(
        transcript,
        "throughput = {:0>.2}\n",
        8.0f64 * mb_thru / secs,
    )?;
    write!(
        transcript,
        "goodput_with_restarts = {:0>.2}\n",
        8.0f64 * mb_good / secs,
    )?;
    write!(transcript, "file_rate = {:0>.2}\n", 8.0f64 * mb_file / secs)?;

    session.transfer.transcript.take();
    Ok(())
}

pub fn xscript_data_log_client(
    session: &mut Session,
    _parameter: &Parameter,
    logline: &str,
) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();
    write!(transcript, "{}", logline)?;
    transcript.flush()?;
    Ok(())
}

pub fn xscript_data_start_client(
    session: &mut Session,
    _parameter: &Parameter,
    epoch: extc::timeval,
) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();
    write!(transcript, "START {}.{:06}\n", epoch.tv_sec, epoch.tv_usec)?;
    transcript.flush()?;
    Ok(())
}

pub unsafe fn xscript_data_stop_client(
    session: &mut Session,
    _parameter: &Parameter,
    epoch: extc::timeval,
) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();
    write!(transcript, "STOP {}.{:06}\n", epoch.tv_sec, epoch.tv_usec)?;
    transcript.flush()?;
    Ok(())
}

pub fn xscript_open_client(session: &mut Session, parameter: &Parameter) -> anyhow::Result<()> {
    let transcript_filename = crate::common::make_transcript_filename("namc");
    let transcript = session.transfer.transcript.insert(
        std::fs::File::options()
            .write(true)
            .create(true)
            .open(Path::new(&transcript_filename))?,
    );

    write!(
        transcript,
        "remote_filename = {}\n",
        session.transfer.remote_filename.as_ref().unwrap().as_str()
    )?;
    write!(
        transcript,
        "local_filename = {}\n",
        session.transfer.local_filename.as_ref().unwrap().as_str()
    )?;
    write!(transcript, "file_size = {}\n", session.transfer.file_size)?;
    write!(
        transcript,
        "block_count = {}\n",
        session.transfer.block_count,
    )?;
    write!(transcript, "udp_buffer = {}\n", parameter.udp_buffer)?;
    write!(transcript, "block_size = {}\n", parameter.block_size)?;
    write!(transcript, "target_rate = {}\n", parameter.target_rate)?;
    write!(transcript, "error_rate = {}\n", parameter.error_rate)?;
    write!(transcript, "slower_num = {}\n", parameter.slower_num)?;
    write!(transcript, "slower_den = {}\n", parameter.slower_den)?;
    write!(transcript, "faster_num = {}\n", parameter.faster_num)?;
    write!(transcript, "faster_den = {}\n", parameter.faster_den)?;
    write!(transcript, "history = {}\n", parameter.history)?;
    write!(transcript, "lossless = {}\n", parameter.lossless)?;
    write!(transcript, "losswindow = {}\n", parameter.losswindow_ms)?;
    write!(transcript, "blockdump = {}\n", parameter.blockdump)?;
    write!(transcript, "update_period = {}\n", 350000)?;
    write!(transcript, "rexmit_period = {}\n", 350000)?;
    write!(
        transcript,
        "protocol_version = 0x{:x}\n",
        crate::common::PROTOCOL_REVISION,
    )?;
    write!(
        transcript,
        "software_version = {}\n",
        crate::common::NAMIDA_VERSION,
    )?;
    write!(transcript, "ipv6 = {}\n", parameter.ipv6_yn)?;
    writeln!(transcript)?;
    transcript.flush()?;
    Ok(())
}
