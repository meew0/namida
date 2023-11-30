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
pub fn close(session: &mut Session, parameter: &Parameter, delta: u64) -> anyhow::Result<()> {
    // File sizes in megabytes, not mibibytes as Tsunami used
    let mb_thru = f64::from(session.transfer.stats.total_blocks.0)
        * f64::from(parameter.block_size.0)
        / 1_000_000.0;
    let mb_good = mb_thru
        - f64::from(session.transfer.stats.total_recvd_retransmits.0)
            * f64::from(parameter.block_size.0)
            / 1_000_000.0;
    #[allow(clippy::cast_precision_loss)]
    let mb_file = session.transfer.file_size.0 as f64 / 1_000_000.0;

    // Microseconds to seconds
    #[allow(clippy::cast_precision_loss)]
    let secs = delta as f64 / 1_000_000.0;

    let transcript = session
        .transfer
        .transcript
        .as_mut()
        .expect("transcript should have been opened");

    writeln!(transcript, "mbyte_transmitted = {mb_thru:0>.2}")?;
    writeln!(transcript, "mbyte_usable = {mb_good:0>.2}")?;
    writeln!(transcript, "mbyte_file = {mb_file:0>.2}")?;
    writeln!(transcript, "duration = {secs:0>.2}")?;
    writeln!(transcript, "throughput = {:0>.2}", 8.0_f64 * mb_thru / secs,)?;
    writeln!(
        transcript,
        "goodput_with_restarts = {:0>.2}",
        8.0_f64 * mb_good / secs,
    )?;
    writeln!(transcript, "file_rate = {:0>.2}", 8.0_f64 * mb_file / secs)?;

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
pub fn data_log(
    session: &mut Session,
    _parameter: &Parameter,
    logline: &str,
) -> anyhow::Result<()> {
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
    let transcript_filename = crate::common::make_transcript_filename("namc");
    let transcript = session.transfer.transcript.insert(
        std::fs::File::options()
            .write(true)
            .create(true)
            .open(Path::new(&transcript_filename))?,
    );

    if let Some(remote_filename) = &session.transfer.remote_filename {
        writeln!(
            transcript,
            "remote_filename = {}",
            remote_filename.display()
        )?;
    } else {
        writeln!(transcript, "remote_filename = <not set>")?;
    }
    if let Some(local_filename) = &session.transfer.local_filename {
        writeln!(transcript, "local_filename = {}", local_filename.display())?;
    } else {
        writeln!(transcript, "local_filename = <not set>")?;
    }
    writeln!(transcript, "file_size = {}", session.transfer.file_size.0)?;
    writeln!(
        transcript,
        "block_count = {}",
        session.transfer.block_count.0
    )?;
    writeln!(transcript, "udp_buffer = {}", parameter.udp_buffer)?;
    writeln!(transcript, "block_size = {}", parameter.block_size)?;
    writeln!(transcript, "target_rate = {}", parameter.target_rate)?;
    writeln!(transcript, "error_rate = {}", parameter.error_rate)?;
    writeln!(transcript, "slower = {}", parameter.slower)?;
    writeln!(transcript, "faster = {}", parameter.faster)?;
    writeln!(transcript, "history = {}", parameter.history)?;
    writeln!(transcript, "lossless = {}", parameter.lossless)?;
    writeln!(transcript, "losswindow = {}", parameter.losswindow_ms)?;
    writeln!(transcript, "blockdump = {}", parameter.blockdump)?;
    writeln!(transcript, "update_period = {}", 350_000)?;
    writeln!(transcript, "rexmit_period = {}", 350_000)?;
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
