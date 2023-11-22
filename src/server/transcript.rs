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

    writeln!(
        transcript,
        "mb_transmitted = {:0>.2}",
        parameter.file_size as f64 / 1000000.0,
    )?;
    writeln!(transcript, "duration = {:0>.2}", delta as f64 / 1000000.0)?;

    // Bits per microsecond = megabits per second
    writeln!(
        transcript,
        "throughput = {:0>.2}",
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
    writeln!(transcript, "START {}.{:06}", epoch.tv_sec, epoch.tv_usec)?;
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

    writeln!(
        transcript,
        "filename = {}",
        session.transfer.filename.as_ref().unwrap()
    )?;
    writeln!(transcript, "file_size = {}", parameter.file_size)?;
    writeln!(transcript, "block_count = {}", parameter.block_count)?;
    writeln!(transcript, "udp_buffer = {}", parameter.udp_buffer)?;
    writeln!(transcript, "block_size = {}", parameter.block_size)?;
    writeln!(transcript, "target_rate = {}", parameter.target_rate)?;
    writeln!(transcript, "error_rate = {}", parameter.error_rate)?;
    writeln!(transcript, "slower_num = {}", parameter.slower_num)?;
    writeln!(transcript, "slower_den = {}", parameter.slower_den)?;
    writeln!(transcript, "faster_num = {}", parameter.faster_num)?;
    writeln!(transcript, "faster_den = {}", parameter.faster_den)?;
    writeln!(transcript, "ipd_time = {}", parameter.ipd_time)?;
    writeln!(transcript, "ipd_current = {}", session.transfer.ipd_current,)?;
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
