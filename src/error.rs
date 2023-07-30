use thiserror::Error;

use crate::Header;

#[derive(Debug, Error)]
pub enum Error {
    #[error("packet size {size} > capacity {capacity}")]
    BufferTooSmallForPacketDecode {
        header: Header,
        size: usize,
        capacity: usize,
    },

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

    #[error("io err: {0}")]
    Io(#[from] std::io::Error),

    #[error("bad position: {0}")]
    SystemBadPosition(u8),

    #[error("bad role: {0}")]
    SystemBadRole(u8),
}
