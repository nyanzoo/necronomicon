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

    #[error("decode err: {kind}-{buffer:?}-{source}")]
    Decode {
        kind: &'static str,
        buffer: Option<&'static str>,
        source: Box<dyn std::error::Error>,
    },

    #[error("encode err: {kind}-{source}")]
    Encode {
        kind: &'static str,
        source: Box<dyn std::error::Error>,
    },

    #[error("invalid header kind: {0}")]
    InvalidHeaderKind(u8),

    #[error("invalid header version: {0}")]
    InvalidHeaderVersion(u8),

    #[error("io err: {0}")]
    Io(#[from] std::io::Error),

    #[error("owned acquire {acquire} > capacity {capacity}")]
    OwnedRemaining { acquire: usize, capacity: usize },

    #[error("bad position: {0}")]
    SystemBadPosition(u8),

    #[error("bad role: {0}")]
    SystemBadRole(u8),
}
