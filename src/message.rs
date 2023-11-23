use crate::types::{
    BlockIndex, BlockSize, Epoch, ErrorRate, FileMetadata, FileSize, Fraction, TargetRate,
};

#[derive(bincode::Encode, bincode::Decode)]
pub enum ClientToServer {
    ProtocolRevision(u32),
    AuthenticationResponse([u8; 16]),
    FileRequest(String),
    MultiRequest,
    MultiEnd,
    BlockSize(BlockSize),
    TargetRate(TargetRate),
    ErrorRate(ErrorRate),
    Slowdown(Fraction),
    Speedup(Fraction),
    UdpPort(u16),
    RestartAt(BlockIndex),
    Retransmit(Vec<BlockIndex>),
    EndTransmission,
    DirList,
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
    Epoch(Epoch),
    DirListStatus(DirListStatus),
    DirListNumFiles(u32),
    DirListFile(FileMetadata),
    MultiHeader { array_size: u32, total: u32 },
    MultiFile(FileMetadata),
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
enum FileRequestError {
    Nonexistent,
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
enum DirListStatus {
    Ok,
    Unsupported,
}
