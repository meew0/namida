use std::io::Write;
use std::path::Path;

use crate::extc;

use super::{Parameter, Session};

pub fn xscript_close_server(
    session: &mut Session,
    parameter: &Parameter,
    mut delta: u64,
) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();

    write!(
        transcript,
        "mb_transmitted = {:0>.2}\n",
        parameter.file_size as f64 / 1000000.0,
    )?;
    write!(transcript, "duration = {:0>.2}\n", delta as f64 / 1000000.0)?;

    // Bits per microsecond = megabits per second
    write!(
        transcript,
        "throughput = {:0>.2}\n",
        parameter.file_size as f64 * 8.0f64 / delta as f64,
    )?;

    session.transfer.transcript.take();
    Ok(())
}

pub fn xscript_data_log_server(session: &mut Session, mut logline: &str) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();
    write!(transcript, "{}", logline)?;
    transcript.flush()?;
    Ok(())
}

pub fn xscript_data_start_server(
    session: &mut Session,
    epoch: extc::timeval,
) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();
    write!(transcript, "START {}.{:06}\n", epoch.tv_sec, epoch.tv_usec)?;
    transcript.flush()?;
    Ok(())
}

pub fn xscript_data_stop_server(session: &mut Session, epoch: extc::timeval) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();
    write!(transcript, "STOP {}.{:06}\n\n", epoch.tv_sec, epoch.tv_usec)?;
    transcript.flush()?;
    Ok(())
}

pub fn xscript_open_server(session: &mut Session, parameter: &Parameter) -> anyhow::Result<()> {
    let transcript_filename = crate::common::make_transcript_filename("nams");
    let transcript = session.transfer.transcript.insert(
        std::fs::File::options()
            .write(true)
            .create(true)
            .open(Path::new(&transcript_filename))?,
    );

    write!(
        transcript,
        "filename = {}\n",
        session.transfer.filename.as_ref().unwrap()
    )?;
    write!(transcript, "file_size = {}\n", parameter.file_size)?;
    write!(transcript, "block_count = {}\n", parameter.block_count)?;
    write!(transcript, "udp_buffer = {}\n", parameter.udp_buffer)?;
    write!(transcript, "block_size = {}\n", parameter.block_size)?;
    write!(transcript, "target_rate = {}\n", parameter.target_rate)?;
    write!(transcript, "error_rate = {}\n", parameter.error_rate)?;
    write!(transcript, "slower_num = {}\n", parameter.slower_num)?;
    write!(transcript, "slower_den = {}\n", parameter.slower_den)?;
    write!(transcript, "faster_num = {}\n", parameter.faster_num)?;
    write!(transcript, "faster_den = {}\n", parameter.faster_den)?;
    write!(transcript, "ipd_time = {}\n", parameter.ipd_time)?;
    write!(
        transcript,
        "ipd_current = {}\n",
        session.transfer.ipd_current,
    )?;
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
