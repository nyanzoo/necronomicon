use std::{
    fmt::Debug,
    io::{Read, Write},
};

use crate::{dequeue_codec, error::Error, kv_store_codec, system_codec, Decode, Encode};

pub struct VersionAndUuid {
    pub version: u8,
    pub uuid: u128,
}

impl VersionAndUuid {
    pub fn new(version: u8, uuid: u128) -> Self {
        Self { version, uuid }
    }

    pub fn into_header(self, kind: impl Into<Kind>) -> Header {
        Header::new(kind, self.version, self.uuid)
    }
}

impl From<(u8, u128)> for VersionAndUuid {
    fn from((version, uuid): (u8, u128)) -> Self {
        Self { version, uuid }
    }
}

// TODO: need to map these to packet types, also need to do partial
// decodes of header to get packet type and then decode the rest.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    // dequeue messages
    // make sure to keep these in sync with the ones in
    // necronomicon/src/dequeue_codec/mod.rs
    Enqueue = dequeue_codec::START as isize,
    EnqueueAck,
    Dequeue,
    DequeueAck,
    Peek,
    PeekAck,
    Len,
    LenAck,
    CreateQueue,
    CreateQueueAck,
    DeleteQueue,
    DeleteQueueAck = dequeue_codec::END as isize,

    // kv store messages
    // make sure to keep these in sync with the ones in
    // necronomicon/src/kv_store_codec/mod.rs
    Put = kv_store_codec::START as isize,
    PutAck,
    Get,
    GetAck,
    Delete,
    DeleteAck = kv_store_codec::END as isize,

    // internal system messages
    Report = system_codec::START as isize,
    ReportAck,
    Join,
    JoinAck,
    Transfer,
    TransferAck,
    Ping,
    PingAck = system_codec::END as isize,
}

impl Debug for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            // dequeue messages
            Self::Enqueue => write!(f, "Enqueue"),
            Self::EnqueueAck => write!(f, "EnqueueAck"),
            Self::Dequeue => write!(f, "Dequeue"),
            Self::DequeueAck => write!(f, "DequeueAck"),
            Self::Peek => write!(f, "Peek"),
            Self::PeekAck => write!(f, "PeekAck"),
            Self::Len => write!(f, "Len"),
            Self::LenAck => write!(f, "LenAck"),
            Self::CreateQueue => write!(f, "CreateQueue"),
            Self::CreateQueueAck => write!(f, "CreateQueueAck"),
            Self::DeleteQueue => write!(f, "DeleteQueue"),
            Self::DeleteQueueAck => write!(f, "DeleteQueueAck"),

            // kv store messages
            Self::Put => write!(f, "Put"),
            Self::PutAck => write!(f, "PutAck"),
            Self::Get => write!(f, "Get"),
            Self::GetAck => write!(f, "GetAck"),
            Self::Delete => write!(f, "Delete"),
            Self::DeleteAck => write!(f, "DeleteAck"),

            // internal system messages
            Self::Report => write!(f, "Report"),
            Self::ReportAck => write!(f, "ReportAck"),
            Self::Join => write!(f, "Join"),
            Self::JoinAck => write!(f, "JoinAck"),
            Self::Transfer => write!(f, "Transfer"),
            Self::TransferAck => write!(f, "TransferAck"),
            Self::Ping => write!(f, "Ping"),
            Self::PingAck => write!(f, "PingAck"),
        };

        res?;

        write!(f, "({})", *self as u8)
    }
}

impl<R> Decode<R> for Kind
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error> {
        u8::decode(reader).map(Self::from)
    }
}

impl<W> Encode<W> for Kind
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        u8::from(*self).encode(writer)
    }
}

impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        match value {
            // dequeue messages
            dequeue_codec::ENQUEUE => Kind::Enqueue,
            dequeue_codec::ENQUEUE_ACK => Kind::EnqueueAck,
            dequeue_codec::DEQUEUE => Kind::Dequeue,
            dequeue_codec::DEQUEUE_ACK => Kind::DequeueAck,
            dequeue_codec::PEEK => Kind::Peek,
            dequeue_codec::PEEK_ACK => Kind::PeekAck,
            dequeue_codec::LEN => Kind::Len,
            dequeue_codec::LEN_ACK => Kind::LenAck,
            dequeue_codec::CREATE => Kind::CreateQueue,
            dequeue_codec::CREATE_ACK => Kind::CreateQueueAck,
            dequeue_codec::DELETE => Kind::DeleteQueue,
            dequeue_codec::DELETE_ACK => Kind::DeleteQueueAck,

            // kv store messages
            kv_store_codec::PUT => Kind::Put,
            kv_store_codec::PUT_ACK => Kind::PutAck,
            kv_store_codec::GET => Kind::Get,
            kv_store_codec::GET_ACK => Kind::GetAck,
            kv_store_codec::DELETE => Kind::Delete,
            kv_store_codec::DELETE_ACK => Kind::DeleteAck,

            // internal system messages
            system_codec::CHAIN => Kind::Report,
            system_codec::CHAIN_ACK => Kind::ReportAck,
            system_codec::JOIN => Kind::Join,
            system_codec::JOIN_ACK => Kind::JoinAck,
            system_codec::TRANSFER => Kind::Transfer,
            system_codec::TRANSFER_ACK => Kind::TransferAck,
            system_codec::PING => Kind::Ping,
            system_codec::PING_ACK => Kind::PingAck,

            _ => panic!("invalid kind: {}", value),
        }
    }
}

impl From<Kind> for u8 {
    fn from(value: Kind) -> Self {
        match value {
            // dequeue messages
            Kind::Enqueue => dequeue_codec::ENQUEUE,
            Kind::EnqueueAck => dequeue_codec::ENQUEUE_ACK,
            Kind::Dequeue => dequeue_codec::DEQUEUE,
            Kind::DequeueAck => dequeue_codec::DEQUEUE_ACK,
            Kind::Peek => dequeue_codec::PEEK,
            Kind::PeekAck => dequeue_codec::PEEK_ACK,
            Kind::Len => dequeue_codec::LEN,
            Kind::LenAck => dequeue_codec::LEN_ACK,
            Kind::CreateQueue => dequeue_codec::CREATE,
            Kind::CreateQueueAck => dequeue_codec::CREATE_ACK,
            Kind::DeleteQueue => dequeue_codec::DELETE,
            Kind::DeleteQueueAck => dequeue_codec::DELETE_ACK,

            // kv store messages
            Kind::Put => kv_store_codec::PUT,
            Kind::PutAck => kv_store_codec::PUT_ACK,
            Kind::Get => kv_store_codec::GET,
            Kind::GetAck => kv_store_codec::GET_ACK,
            Kind::Delete => kv_store_codec::DELETE,
            Kind::DeleteAck => kv_store_codec::DELETE_ACK,

            // internal system messages
            Kind::Report => system_codec::CHAIN,
            Kind::ReportAck => system_codec::CHAIN_ACK,
            Kind::Join => system_codec::JOIN,
            Kind::JoinAck => system_codec::JOIN_ACK,
            Kind::Transfer => system_codec::TRANSFER,
            Kind::TransferAck => system_codec::TRANSFER_ACK,
            Kind::Ping => system_codec::PING,
            Kind::PingAck => system_codec::PING_ACK,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    kind: Kind,
    version: u8,
    uuid: u128,
}

impl Header {
    pub fn new(kind: impl Into<Kind>, version: u8, uuid: u128) -> Self {
        Self {
            kind: kind.into(),
            version,
            uuid,
        }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn uuid(&self) -> u128 {
        self.uuid
    }
}

impl<R> Decode<R> for Header
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error> {
        let kind = Kind::decode(reader)?;
        let version = u8::decode(reader)?;
        let uuid = u128::decode(reader)?;

        Ok(Header {
            kind,
            version,
            uuid,
        })
    }
}

impl<W> Encode<W> for Header
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.kind.encode(writer)?;
        self.version.encode(writer)?;
        self.uuid.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use test_case::test_case;

    use crate::{Decode, Encode};

    use super::Header;

    #[cfg_attr(nightly, no_coverage)]
    #[test_case(0, 0, 0; "zero")]
    #[test_case(1, 1, 1; "one")]
    #[test_case(2, 2, 2; "two")]
    fn test_header_encode_decode(kind: u8, version: u8, uuid: u128) {
        let mut buf = Vec::new();
        let header = Header::new(kind, version, uuid);
        header.encode(&mut buf).expect("encode");

        let mut buf = Cursor::new(buf);
        let actual = Header::decode(&mut buf).expect("decode");
        assert_eq!(actual, header);
    }
}
