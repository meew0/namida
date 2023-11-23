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

    writeln!(transcript, "mbyte_transmitted = {:0>.2}", mb_thru)?;
    writeln!(transcript, "mbyte_usable = {:0>.2}", mb_good)?;
    writeln!(transcript, "mbyte_file = {:0>.2}", mb_file)?;
    writeln!(transcript, "duration = {:0>.2}", secs)?;
    writeln!(transcript, "throughput = {:0>.2}", 8.0f64 * mb_thru / secs,)?;
    writeln!(
        transcript,
        "goodput_with_restarts = {:0>.2}",
        8.0f64 * mb_good / secs,
    )?;
    writeln!(transcript, "file_rate = {:0>.2}", 8.0f64 * mb_file / secs)?;

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
    writeln!(transcript, "START {}.{:06}", epoch.tv_sec, epoch.tv_usec)?;
    transcript.flush()?;
    Ok(())
}

pub unsafe fn xscript_data_stop_client(
    session: &mut Session,
    _parameter: &Parameter,
    epoch: extc::timeval,
) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();
    writeln!(transcript, "STOP {}.{:06}", epoch.tv_sec, epoch.tv_usec)?;
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

    writeln!(
        transcript,
        "remote_filename = {}",
        session.transfer.remote_filename.as_ref().unwrap().as_str()
    )?;
    writeln!(
        transcript,
        "local_filename = {}",
        session.transfer.local_filename.as_ref().unwrap().as_str()
    )?;
    writeln!(transcript, "file_size = {}", session.transfer.file_size)?;
    writeln!(transcript, "block_count = {}", session.transfer.block_count,)?;
    writeln!(transcript, "udp_buffer = {}", parameter.udp_buffer)?;
    writeln!(transcript, "block_size = {}", parameter.block_size)?;
    writeln!(transcript, "target_rate = {}", parameter.target_rate)?;
    writeln!(transcript, "error_rate = {}", parameter.error_rate)?;
    writeln!(transcript, "slower_num = {}", parameter.slower.numerator)?;
    writeln!(transcript, "slower_den = {}", parameter.slower.denominator)?;
    writeln!(transcript, "faster_num = {}", parameter.faster.numerator)?;
    writeln!(transcript, "faster_den = {}", parameter.faster.denominator)?;
    writeln!(transcript, "history = {}", parameter.history)?;
    writeln!(transcript, "lossless = {}", parameter.lossless)?;
    writeln!(transcript, "losswindow = {}", parameter.losswindow_ms)?;
    writeln!(transcript, "blockdump = {}", parameter.blockdump)?;
    writeln!(transcript, "update_period = {}", 350000)?;
    writeln!(transcript, "rexmit_period = {}", 350000)?;
    writeln!(
        transcript,
        "protocol_version = 0x{:x}",
        crate::common::PROTOCOL_REVISION,
    )?;
    writeln!(
        transcript,
        "software_version = {}",
        crate::common::NAMIDA_VERSION,
    )?;
    writeln!(transcript, "ipv6 = {}", parameter.ipv6_yn)?;
    writeln!(transcript)?;
    transcript.flush()?;
    Ok(())
}
