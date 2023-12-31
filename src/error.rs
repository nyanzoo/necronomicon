use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("decode err: {0}")]
    Decode(#[source] std::io::Error),

    #[error("decode string err: {0}")]
    DecodeString(#[source] std::string::FromUtf8Error),

    #[error("encode err: {0}")]
    Encode(#[source] std::io::Error),

    #[error("incomplete header: {0}")]
    IncompleteHeader(#[source] std::io::Error),

    #[error("invalid header kind: {0}")]
    InvalidHeaderKind(u8),

    #[error("invalid header version: {0}")]
    InvalidHeaderVersion(u8),

    #[error("invalid key len from key: {0}")]
    InvalidKeyLength(String),

    #[error("io err: {0}")]
    Io(#[from] std::io::Error),

    #[error("bad position: {0}")]
    SystemBadPosition(u8),

    #[error("bad role: {0}")]
    SystemBadRole(u8),

    #[error("trailing bytes: {0}")]
    TrailingBytes(usize),
}
