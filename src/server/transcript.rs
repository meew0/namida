use std::io::Write;
use std::path::Path;

use super::{Parameter, Session};

pub fn xscript_close_server(session: &mut Session, mut delta: u64) -> anyhow::Result<()> {
    let transcript = session.transfer.transcript.as_mut().unwrap();

    writeln!(
        transcript,
        "mb_transmitted = {:0>.2}",
        session.properties.file_size.0 as f64 / 1000000.0,
    )?;
    writeln!(transcript, "duration = {:0>.2}", delta as f64 / 1000000.0)?;

    // Bits per microsecond = megabits per second
    writeln!(
        transcript,
        "throughput = {:0>.2}",
        session.properties.file_size.0 as f64 * 8.0f64 / delta as f64,
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

pub fn xscript_data_start_server(session: &mut Session) -> anyhow::Result<()> {
    let start_time = crate::common::epoch();

    let transcript = session.transfer.transcript.as_mut().unwrap();
    writeln!(
        transcript,
        "START {}.{:06}",
        start_time.as_secs(),
        start_time.subsec_micros()
    )?;
    transcript.flush()?;
    Ok(())
}

pub fn xscript_data_stop_server(session: &mut Session) -> anyhow::Result<()> {
    let end_time = crate::common::epoch();

    let transcript = session.transfer.transcript.as_mut().unwrap();
    writeln!(
        transcript,
        "STOP {}.{:06}",
        end_time.as_secs(),
        end_time.subsec_micros()
    )?;
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
        session.transfer.filename.as_ref().unwrap().display()
    )?;
    writeln!(transcript, "file_size = {}", session.properties.file_size.0)?;
    writeln!(
        transcript,
        "block_count = {}",
        session.properties.block_count.0
    )?;
    writeln!(transcript, "udp_buffer = {}", parameter.udp_buffer)?;
    writeln!(
        transcript,
        "block_size = {}",
        session.properties.block_size.0
    )?;
    writeln!(
        transcript,
        "target_rate = {}",
        session.properties.target_rate.0
    )?;
    writeln!(
        transcript,
        "error_rate = {}",
        session.properties.error_rate.0
    )?;
    writeln!(transcript, "slower = {}", session.properties.slower)?;
    writeln!(transcript, "faster = {}", session.properties.faster)?;
    writeln!(transcript, "ipd_time = {}", session.properties.ipd_time)?;
    writeln!(transcript, "ipd_current = {}", session.transfer.ipd_current)?;
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
    writeln!(transcript, "bind = {}", parameter.bind)?;
    writeln!(transcript)?;
    transcript.flush()?;
    Ok(())
}
