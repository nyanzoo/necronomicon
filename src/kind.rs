use std::{
    fmt::Debug,
    io::{Read, Write},
};

use crate::{deque_codec, kv_store_codec, system_codec, Decode, Encode, Error};

// TODO: need to map these to packet types, also need to do partial
// decodes of header to get packet type and then decode the rest.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    // deque messages
    // make sure to keep these in sync with the ones in
    // necronomicon/src/deque_codec/mod.rs
    Enqueue = deque_codec::START as isize,
    EnqueueAck,
    Deque,
    DequeAck,
    Peek,
    PeekAck,
    Len,
    LenAck,
    CreateQueue,
    CreateQueueAck,
    DeleteQueue,
    DeleteQueueAck = deque_codec::END as isize,

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
            // deque messages
            Self::Enqueue => write!(f, "Enqueue"),
            Self::EnqueueAck => write!(f, "EnqueueAck"),
            Self::Deque => write!(f, "Deque"),
            Self::DequeAck => write!(f, "DequeAck"),
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
            // deque messages
            deque_codec::ENQUEUE => Kind::Enqueue,
            deque_codec::ENQUEUE_ACK => Kind::EnqueueAck,
            deque_codec::DEQUEUE => Kind::Deque,
            deque_codec::DEQUEUE_ACK => Kind::DequeAck,
            deque_codec::PEEK => Kind::Peek,
            deque_codec::PEEK_ACK => Kind::PeekAck,
            deque_codec::LEN => Kind::Len,
            deque_codec::LEN_ACK => Kind::LenAck,
            deque_codec::CREATE => Kind::CreateQueue,
            deque_codec::CREATE_ACK => Kind::CreateQueueAck,
            deque_codec::DELETE => Kind::DeleteQueue,
            deque_codec::DELETE_ACK => Kind::DeleteQueueAck,

            // kv store messages
            kv_store_codec::PUT => Kind::Put,
            kv_store_codec::PUT_ACK => Kind::PutAck,
            kv_store_codec::GET => Kind::Get,
            kv_store_codec::GET_ACK => Kind::GetAck,
            kv_store_codec::DELETE => Kind::Delete,
            kv_store_codec::DELETE_ACK => Kind::DeleteAck,

            // internal system messages
            system_codec::REPORT => Kind::Report,
            system_codec::REPORT_ACK => Kind::ReportAck,
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
            // deque messages
            Kind::Enqueue => deque_codec::ENQUEUE,
            Kind::EnqueueAck => deque_codec::ENQUEUE_ACK,
            Kind::Deque => deque_codec::DEQUEUE,
            Kind::DequeAck => deque_codec::DEQUEUE_ACK,
            Kind::Peek => deque_codec::PEEK,
            Kind::PeekAck => deque_codec::PEEK_ACK,
            Kind::Len => deque_codec::LEN,
            Kind::LenAck => deque_codec::LEN_ACK,
            Kind::CreateQueue => deque_codec::CREATE,
            Kind::CreateQueueAck => deque_codec::CREATE_ACK,
            Kind::DeleteQueue => deque_codec::DELETE,
            Kind::DeleteQueueAck => deque_codec::DELETE_ACK,

            // kv store messages
            Kind::Put => kv_store_codec::PUT,
            Kind::PutAck => kv_store_codec::PUT_ACK,
            Kind::Get => kv_store_codec::GET,
            Kind::GetAck => kv_store_codec::GET_ACK,
            Kind::Delete => kv_store_codec::DELETE,
            Kind::DeleteAck => kv_store_codec::DELETE_ACK,

            // internal system messages
            Kind::Report => system_codec::REPORT,
            Kind::ReportAck => system_codec::REPORT_ACK,
            Kind::Join => system_codec::JOIN,
            Kind::JoinAck => system_codec::JOIN_ACK,
            Kind::Transfer => system_codec::TRANSFER,
            Kind::TransferAck => system_codec::TRANSFER_ACK,
            Kind::Ping => system_codec::PING,
            Kind::PingAck => system_codec::PING_ACK,
        }
    }
}
