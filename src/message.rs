use std::{path::PathBuf, time::Duration};

use crate::types::{
    BlockIndex, BlockSize, ErrorRate, FileMetadata, FileSize, Fraction, TargetRate,
};

#[derive(bincode::Encode, bincode::Decode)]
pub enum ClientToServer {
    ProtocolRevision(u32),
    AuthenticationResponse([u8; 16]),
    FileRequest(PathBuf),
    MultiRequest,
    MultiAcknowledgeCount,
    MultiEnd,
    BlockSize(BlockSize),
    TargetRate(TargetRate),
    ErrorRate(ErrorRate),
    Slowdown(Fraction),
    Speedup(Fraction),
    UdpPort(u16),
    RetransmitMany(Vec<BlockIndex>),
    DirList,
    DirListEnd,
}

#[derive(bincode::Encode, bincode::Decode)]
pub enum ServerToClient {
    ProtocolRevision(u32),
    AuthenticationChallenge([u8; 64]),
    AuthenticationStatus(bool),
    FileResponseOne(Result<(), FileRequestError>),
    FileSize(FileSize),
    BlockSize(BlockSize),
    BlockCount(BlockIndex),
    Epoch(Duration),
    DirListHeader {
        status: DirListStatus,
        num_files: u32,
    },
    DirListFile(FileMetadata),
    MultiFileCount(u32),
    MultiFile(FileMetadata),
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub enum TransmissionControl {
    RestartAt(BlockIndex),
    Retransmit(BlockIndex),
    SubmitErrorRate(ErrorRate),

    /// Dummy value to ensure all enum variants have the same length
    EndTransmission(u32),
}

impl TransmissionControl {
    pub const SIZE: usize = 8;
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub enum FileRequestError {
    Nonexistent,
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub enum DirListStatus {
    Ok,
    Unsupported,
}

#[cfg(test)]
mod tests {
    use crate::types::{BlockIndex, ErrorRate};

    use super::TransmissionControl;

    #[test]
    fn transmission_control_sizes() -> anyhow::Result<()> {
        let mut slice = [0_u8; 8];

        assert_eq!(
            bincode::encode_into_slice(
                TransmissionControl::RestartAt(BlockIndex(0)),
                &mut slice,
                crate::common::BINCODE_CONFIG,
            )?,
            TransmissionControl::SIZE
        );
        assert_eq!(
            bincode::encode_into_slice(
                TransmissionControl::Retransmit(BlockIndex(0)),
                &mut slice,
                crate::common::BINCODE_CONFIG,
            )?,
            TransmissionControl::SIZE
        );
        assert_eq!(
            bincode::encode_into_slice(
                TransmissionControl::SubmitErrorRate(ErrorRate(0)),
                &mut slice,
                crate::common::BINCODE_CONFIG,
            )?,
            TransmissionControl::SIZE
        );
        assert_eq!(
            bincode::encode_into_slice(
                TransmissionControl::EndTransmission(0),
                &mut slice,
                crate::common::BINCODE_CONFIG,
            )?,
            TransmissionControl::SIZE
        );

        Ok(())
    }
}
