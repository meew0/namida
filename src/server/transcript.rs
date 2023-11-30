use std::io::Write;
use std::path::Path;

use super::{Parameter, Session};

/// Closes the transcript file for the given session after writing out the final transfer
/// statistics.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if no transcript file is opened.
pub fn close(session: &mut Session, delta: u64) -> anyhow::Result<()> {
    let transcript = session
        .transfer
        .transcript
        .as_mut()
        .expect("transcript should have been opened");

    #[allow(clippy::cast_precision_loss)]
    writeln!(
        transcript,
        "mb_transmitted = {:0>.2}",
        session.properties.file_size.0 as f64 / 1_000_000.0,
    )?;
    #[allow(clippy::cast_precision_loss)]
    writeln!(transcript, "duration = {:0>.2}", delta as f64 / 1_000_000.0)?;

    // Bits per microsecond = megabits per second
    #[allow(clippy::cast_precision_loss)]
    writeln!(
        transcript,
        "throughput = {:0>.2}",
        session.properties.file_size.0 as f64 * 8.0_f64 / delta as f64,
    )?;

    session.transfer.transcript.take();
    Ok(())
}

/// Logs the given line to the transcript.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if no transcript file is opened.
pub fn data_log(session: &mut Session, logline: &str) -> anyhow::Result<()> {
    let transcript = session
        .transfer
        .transcript
        .as_mut()
        .expect("transcript should have been opened");
    write!(transcript, "{logline}")?;
    transcript.flush()?;
    Ok(())
}

/// Begins the data section of the transcript with a "START" line containing the current epoch.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if no transcript file is opened.
pub fn data_start(session: &mut Session) -> anyhow::Result<()> {
    let start_time = crate::common::epoch();

    let transcript = session
        .transfer
        .transcript
        .as_mut()
        .expect("transcript should have been opened");
    writeln!(
        transcript,
        "START {}.{:06}",
        start_time.as_secs(),
        start_time.subsec_micros()
    )?;
    transcript.flush()?;
    Ok(())
}

/// Terminates the data section of the transcript with a "STOP" line containing the current epoch.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if no transcript file is opened.
pub fn data_stop(session: &mut Session) -> anyhow::Result<()> {
    let end_time = crate::common::epoch();

    let transcript = session
        .transfer
        .transcript
        .as_mut()
        .expect("transcript should have been opened");
    writeln!(
        transcript,
        "STOP {}.{:06}",
        end_time.as_secs(),
        end_time.subsec_micros()
    )?;
    transcript.flush()?;
    Ok(())
}

/// Opens a new transcript file for the given session and writes the initial transcript information
/// to the file.
///
/// # Errors
/// Returns an error on I/O failure.
pub fn open(session: &mut Session, parameter: &Parameter) -> anyhow::Result<()> {
    let transcript_filename = crate::common::make_transcript_filename("nams");
    let transcript = session.transfer.transcript.insert(
        std::fs::File::options()
            .write(true)
            .create(true)
            .open(Path::new(&transcript_filename))?,
    );

    if let Some(filename) = &session.transfer.filename {
        writeln!(transcript, "filename = {}", filename.display())?;
    } else {
        writeln!(transcript, "filename = <not set>")?;
    }
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
        crate::version::PROTOCOL_REVISION,
    )?;
    writeln!(
        transcript,
        "namida_protocol_revision = {}",
        crate::version::NAMIDA_PROTOCOL_REVISION,
    )?;
    writeln!(
        transcript,
        "software_version = {}",
        crate::version::NAMIDA_VERSION,
    )?;
    writeln!(transcript, "bind = {}", parameter.bind)?;
    writeln!(transcript)?;
    transcript.flush()?;
    Ok(())
}
