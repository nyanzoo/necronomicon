use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("owned acquire {acquire} > capacity {capacity}")]
    OwnedRemaining { acquire: usize, capacity: usize },

    #[error("expected buffer size {expected} < read bytes {read}")]
    BinaryDataSizeMismatch { expected: usize, read: usize },

    #[error("decode err: {0}")]
    Decode(#[source] std::io::Error),

    #[error("encode err: {0}")]
    Encode(#[source] std::io::Error),

    #[error("invalid header kind: {0}")]
    InvalidHeaderKind(u8),

    #[error("invalid header version: {0}")]
    InvalidHeaderVersion(u8),

    #[error("io err: {0}")]
    Io(#[from] std::io::Error),

    #[error("bad position: {0}")]
    SystemBadPosition(u8),

    #[error("bad role: {0}")]
    SystemBadRole(u8),
}
