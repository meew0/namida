use std::path::PathBuf;

use crate::message;
use anyhow::bail;

#[derive(Clone, clap::Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct Parameter {
    /// The server to connect to. May be specified as IP address or hostname. A remote TCP port may
    /// also be specified using the `host:port` notation. If no port is specified, the default port
    /// will be used (51038).
    #[arg(long = "server", short = 's')]
    pub server: String,

    /// If this flag is present, the client will not encrypt the connection. The same flag must also
    /// be specified on the server.
    #[arg(long = "unencrypted", action = clap::ArgAction::SetFalse)]
    pub encrypted: bool,

    /// Specifies the path to a file from which the pre-shared key will be loaded. Only the first 32
    /// bytes of the file will be used as the PSK. If not specified, a hard-coded key will be used;
    /// this is not recommended.
    #[arg(long = "secret")]
    pub secret_file: Option<PathBuf>,

    #[arg(skip = *crate::common::DEFAULT_SECRET)]
    pub secret: [u8; 32],
}

#[allow(clippy::missing_errors_doc)]
pub fn run(mut parameter: Parameter) -> anyhow::Result<()> {
    crate::common::load_secret(&parameter.secret_file, &mut parameter.secret);
    super::print_intro(parameter.encrypted);

    // Connect to the server
    let mut session =
        super::protocol::connect(&parameter.server, parameter.encrypted, &parameter.secret)?;

    // send request and parse the resulting response
    session
        .server
        .write(message::ClientToServer::FileListRequest)?;
    let message::ServerToClient::FileCount(num_files) = session.server.read()? else {
        bail!("Expected file count");
    };

    eprintln!("Remote file list:");
    for i in 0..num_files {
        let message::ServerToClient::FileListEntry(file_metadata) = session.server.read()? else {
            bail!("Expected file list entry");
        };

        eprintln!(
            " {:2}) {:<64} {:10}",
            i,
            file_metadata.path.display(),
            file_metadata.size.0
        );
    }
    eprintln!();
    Ok(())
}
