#![cfg_attr(nightly, feature(no_coverage))]

use std::io::{Read, Write};

mod codes;
pub use codes::{
    FAILED_TO_PUSH_TO_TRANSACTION_LOG, INTERNAL_ERROR, KEY_ALREADY_EXISTS, KEY_DOES_NOT_EXIST,
    QUEUE_ALREADY_EXISTS, QUEUE_DOES_NOT_EXIST, QUEUE_EMPTY, QUEUE_FULL, SERVER_BUSY, SUCCESS,
};

pub mod dequeue_codec;
use dequeue_codec::{
    Create, CreateAck, Delete as DeleteQueue, DeleteAck as DeleteQueueAck, Dequeue, DequeueAck,
    Enqueue, EnqueueAck, Len, LenAck, Peek, PeekAck,
};

mod error;
pub use error::Error;

mod header;
pub use header::{Header, Kind};

pub mod kv_store_codec;
use kv_store_codec::{Delete, DeleteAck, Get, GetAck, Put, PutAck};

pub mod system_codec;
use system_codec::{Chain, ChainAck, Join, JoinAck, Transfer, TransferAck};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    // dequeue
    Enqueue(Enqueue),
    EnqueueAck(EnqueueAck),
    Dequeue(Dequeue),
    DequeueAck(DequeueAck),
    Peek(Peek),
    PeekAck(PeekAck),
    Len(Len),
    LenAck(LenAck),
    CreateQueue(Create),
    CreateQueueAck(CreateAck),
    DeleteQueue(DeleteQueue),
    DeleteQueueAck(DeleteQueueAck),

    // kv store
    Put(Put),
    PutAck(PutAck),
    Get(Get),
    GetAck(GetAck),
    Delete(Delete),
    DeleteAck(DeleteAck),

    // internal system messages
    Chain(Chain),
    ChainAck(ChainAck),
    Join(Join),
    JoinAck(JoinAck),
    Transfer(Transfer),
    TransferAck(TransferAck),
}

impl Packet {
    pub fn nack(self, response_code: u8) -> Option<Packet> {
        match self {
            // dequeue
            Packet::Enqueue(this) => Some(Packet::EnqueueAck(this.nack(response_code))),
            Packet::Dequeue(this) => Some(Packet::DequeueAck(this.nack(response_code))),
            Packet::Peek(this) => Some(Packet::PeekAck(this.nack(response_code))),
            Packet::Len(this) => Some(Packet::LenAck(this.nack(response_code))),
            Packet::CreateQueue(this) => Some(Packet::CreateQueueAck(this.nack(response_code))),
            Packet::DeleteQueue(this) => Some(Packet::DeleteQueueAck(this.nack(response_code))),

            // kv store
            Packet::Put(this) => Some(Packet::PutAck(this.nack(response_code))),
            Packet::Get(this) => Some(Packet::GetAck(this.nack(response_code))),
            Packet::Delete(this) => Some(Packet::DeleteAck(this.nack(response_code))),

            // internal system messages
            Packet::Chain(this) => Some(Packet::ChainAck(this.nack(response_code))),
            Packet::Join(this) => Some(Packet::JoinAck(this.nack(response_code))),
            Packet::Transfer(this) => Some(Packet::TransferAck(this.nack(response_code))),

            // acks
            _ => None,
        }
    }
}

pub trait Ack {
    fn header(&self) -> &Header;

    fn response_code(&self) -> u8;
}

/// # Description
/// After decoding the `Header`, the `PartialDecode` trait is used to decode the rest of the bytes.
pub trait PartialDecode<R>
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized;
}

pub fn partial_decode<R>(header: Header, reader: &mut R) -> Result<Packet, Error>
where
    R: Read,
{
    let packet = match header.kind() {
        // dequeue messages
        Kind::Enqueue => Packet::Enqueue(Enqueue::decode(header, reader)?),
        Kind::EnqueueAck => Packet::EnqueueAck(EnqueueAck::decode(header, reader)?),
        Kind::Dequeue => Packet::Dequeue(Dequeue::decode(header, reader)?),
        Kind::DequeueAck => Packet::DequeueAck(DequeueAck::decode(header, reader)?),
        Kind::Peek => Packet::Peek(Peek::decode(header, reader)?),
        Kind::PeekAck => Packet::PeekAck(PeekAck::decode(header, reader)?),
        Kind::Len => Packet::Len(Len::decode(header, reader)?),
        Kind::LenAck => Packet::LenAck(LenAck::decode(header, reader)?),
        Kind::CreateQueue => Packet::CreateQueue(Create::decode(header, reader)?),
        Kind::CreateQueueAck => Packet::CreateQueueAck(CreateAck::decode(header, reader)?),
        Kind::DeleteQueue => Packet::DeleteQueue(DeleteQueue::decode(header, reader)?),
        Kind::DeleteQueueAck => Packet::DeleteQueueAck(DeleteQueueAck::decode(header, reader)?),

        // kv store messages
        Kind::Put => Packet::Put(Put::decode(header, reader)?),
        Kind::PutAck => Packet::PutAck(PutAck::decode(header, reader)?),
        Kind::Get => Packet::Get(Get::decode(header, reader)?),
        Kind::GetAck => Packet::GetAck(GetAck::decode(header, reader)?),
        Kind::Delete => Packet::Delete(Delete::decode(header, reader)?),
        Kind::DeleteAck => Packet::DeleteAck(DeleteAck::decode(header, reader)?),

        // internal system messages
        Kind::Chain => Packet::Chain(Chain::decode(header, reader)?),
        Kind::ChainAck => Packet::ChainAck(ChainAck::decode(header, reader)?),
        Kind::Join => Packet::Join(Join::decode(header, reader)?),
        Kind::JoinAck => Packet::JoinAck(JoinAck::decode(header, reader)?),
        Kind::Transfer => Packet::Transfer(Transfer::decode(header, reader)?),
        Kind::TransferAck => Packet::TransferAck(TransferAck::decode(header, reader)?),
    };

    Ok(packet)
}

pub fn full_decode<R>(reader: &mut R) -> Result<Packet, Error>
where
    R: Read,
{
    let header = Header::decode(reader)?;
    partial_decode(header, reader)
}

pub trait Decode<R>
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait Encode<W>
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error>;
}

// impls for Packet
impl<W> Encode<W> for Packet
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            // dequeue
            Packet::Enqueue(packet) => packet.encode(writer),
            Packet::EnqueueAck(packet) => packet.encode(writer),
            Packet::Dequeue(packet) => packet.encode(writer),
            Packet::DequeueAck(packet) => packet.encode(writer),
            Packet::Peek(packet) => packet.encode(writer),
            Packet::PeekAck(packet) => packet.encode(writer),
            Packet::Len(packet) => packet.encode(writer),
            Packet::LenAck(packet) => packet.encode(writer),
            Packet::CreateQueue(packet) => packet.encode(writer),
            Packet::CreateQueueAck(packet) => packet.encode(writer),
            Packet::DeleteQueue(packet) => packet.encode(writer),
            Packet::DeleteQueueAck(packet) => packet.encode(writer),

            // kv store
            Packet::Put(packet) => packet.encode(writer),
            Packet::PutAck(packet) => packet.encode(writer),
            Packet::Get(packet) => packet.encode(writer),
            Packet::GetAck(packet) => packet.encode(writer),
            Packet::Delete(packet) => packet.encode(writer),
            Packet::DeleteAck(packet) => packet.encode(writer),

            // internal system messages
            Packet::Chain(packet) => packet.encode(writer),
            Packet::ChainAck(packet) => packet.encode(writer),
            Packet::Join(packet) => packet.encode(writer),
            Packet::JoinAck(packet) => packet.encode(writer),
            Packet::Transfer(packet) => packet.encode(writer),
            Packet::TransferAck(packet) => packet.encode(writer),
        }
    }
}

//
// String + Vec impls
//

impl<R> Decode<R> for String
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut len = [0; 2];
        reader.read_exact(&mut len).map_err(Error::Decode)?;
        let len = u16::from_be_bytes(len);
        let mut bytes = vec![0; len as usize];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        String::from_utf8(bytes).map_err(Error::DecodeString)
    }
}

impl<W> Encode<W> for String
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        let bytes = self.as_bytes();
        let len = bytes.len() as u16;
        writer
            .write_all(&len.to_be_bytes())
            .map_err(Error::Encode)?;
        writer.write_all(bytes).map_err(Error::Encode)?;
        Ok(())
    }
}

impl<R> Decode<R> for Vec<u8>
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut len = [0; 2];
        reader.read_exact(&mut len).map_err(Error::Decode)?;
        let len = u16::from_be_bytes(len);
        let mut bytes = vec![0; len as usize];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(bytes)
    }
}

impl<W> Encode<W> for Vec<u8>
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        let len = self.len() as u16;
        writer
            .write_all(&len.to_be_bytes())
            .map_err(Error::Encode)?;
        writer.write_all(self).map_err(Error::Encode)?;
        Ok(())
    }
}

//
// integer types
//

impl<R> Decode<R> for u8
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 1];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(bytes[0])
    }
}

impl<W> Encode<W> for u8
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        writer
            .write_all(&self.to_be_bytes())
            .map_err(Error::Encode)?;
        Ok(())
    }
}

impl<R> Decode<R> for u16
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 2];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(u16::from_be_bytes(bytes))
    }
}

impl<W> Encode<W> for u16
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        writer
            .write_all(&self.to_be_bytes())
            .map_err(Error::Encode)?;
        Ok(())
    }
}

impl<R> Decode<R> for u32
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 4];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(u32::from_be_bytes(bytes))
    }
}

impl<W> Encode<W> for u32
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        writer
            .write_all(&self.to_be_bytes())
            .map_err(Error::Encode)?;
        Ok(())
    }
}

impl<R> Decode<R> for u64
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 8];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(u64::from_be_bytes(bytes))
    }
}

impl<W> Encode<W> for u64
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        writer
            .write_all(&self.to_be_bytes())
            .map_err(Error::Encode)?;
        Ok(())
    }
}

impl<R> Decode<R> for usize
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 8];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(usize::from_be_bytes(bytes))
    }
}

impl<W> Encode<W> for usize
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        writer
            .write_all(&self.to_be_bytes())
            .map_err(Error::Encode)?;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::{fmt::Debug, io::Cursor};

    use crate::{kv_store_codec::TEST_KEY, Header, Packet};

    use super::{Decode, Encode};

    // macro for testing acks
    #[cfg(test)]
    macro_rules! test_ack_packet {
        // Multiple fields
        (
            $kind:expr,
            $struct:ty {
                $($i:ident: $v:expr,)+
            }
        ) => {
            use crate::{Ack, Header, SUCCESS};

            type T = $struct;
            let header = Header::new($kind, 123, 456);
            let ack = T { header, $($i: $v),+ };
            assert_eq!(ack.header(), &header);
            assert_eq!(ack.response_code(), SUCCESS);
        };
        // Single field
        (
            $kind:expr,
            $struct:ty {
                $i:ident: $v:expr
            }
        ) => {
            use crate::{Ack, Header, SUCCESS};

            type T = $struct;
            let header = Header::new($kind, 123, 456);
            let ack = T { header, $i: $v };
            assert_eq!(ack.header(), &header);
            assert_eq!(ack.response_code(), SUCCESS);
        };
    }
    pub(crate) use test_ack_packet;

    // macro for testing encode/decode
    #[cfg(test)]
    macro_rules! test_encode_decode_packet {
        // Multiple fields
        (
            $kind:expr,
            $struct:ty {
                $($i:ident: $v:expr,)+
            }
        ) => {
            use crate::{Decode, Encode, Header, PartialDecode};

            type T = $struct;
            let header = Header::new($kind, 123, 456);
            let mut bytes = vec![];
            let packet = T { header, $($i: $v),+ };
            packet.encode(&mut bytes).unwrap();
            let mut slice = bytes.as_slice();
            let header = Header::decode(&mut slice).unwrap();
            let decoded = T::decode(header, &mut slice).unwrap();
            assert_eq!(packet, decoded);
        };
        // Single field
        (
            $kind:expr,
            $struct:ty {
                $i:ident: $v:expr
            }
        ) => {
            use crate::{Decode, Encode, Header, PartialDecode};

            type T = $struct;
            let header = Header::new($kind, 123, 456);
            let mut bytes = vec![];
            let packet = T { header, $i: $v };
            packet.encode(&mut bytes).unwrap();
            let mut slice = bytes.as_slice();
            let header = Header::decode(&mut slice).unwrap();
            let decoded = T::decode(header, &mut slice).unwrap();
            assert_eq!(packet, decoded);
        };
    }
    pub(crate) use test_encode_decode_packet;

    #[test]
    fn test_full_decode_packets() {
        let packets = test_packets();

        for packet in packets {
            let mut bytes = vec![];
            packet.encode(&mut bytes).unwrap();
            let mut cursor = Cursor::new(bytes);
            let decoded = crate::full_decode(&mut cursor).unwrap();
            assert_eq!(packet, decoded);
        }
    }

    #[test_case::test_case(1u8; "u8")]
    #[test_case::test_case(1u16; "u16")]
    #[test_case::test_case(1u32; "u32")]
    #[test_case::test_case(1u64; "u64")]
    #[test_case::test_case(1usize; "usize")]
    #[test_case::test_case("hello".to_string(); "string")]
    #[test_case::test_case(vec![1, 2, 3]; "vec")]
    fn test_encode_decode<T>(val: T)
    where
        T: Decode<Cursor<Vec<u8>>> + Encode<Vec<u8>> + Debug + Eq + PartialEq,
    {
        let mut bytes = vec![];
        val.encode(&mut bytes).unwrap();
        let mut cursor = Cursor::new(bytes);
        let decoded = T::decode(&mut cursor).unwrap();
        assert_eq!(val, decoded);
    }

    fn test_packets() -> Vec<Packet> {
        vec![
            Packet::Enqueue(crate::dequeue_codec::Enqueue {
                header: Header::new(crate::Kind::Enqueue, 123, 456),
                path: "hello".to_string(),
                value: vec![1, 2, 3],
            }),
            Packet::EnqueueAck(crate::dequeue_codec::EnqueueAck {
                header: Header::new(crate::Kind::EnqueueAck, 123, 456),
                response_code: crate::SUCCESS,
            }),
            Packet::Dequeue(crate::dequeue_codec::Dequeue {
                header: Header::new(crate::Kind::Dequeue, 123, 456),
                path: "hello".to_string(),
            }),
            Packet::DequeueAck(crate::dequeue_codec::DequeueAck {
                header: Header::new(crate::Kind::DequeueAck, 123, 456),
                response_code: crate::SUCCESS,
                value: vec![1, 2, 3],
            }),
            Packet::Peek(crate::dequeue_codec::Peek {
                header: Header::new(crate::Kind::Peek, 123, 456),
                path: "hello".to_string(),
            }),
            Packet::PeekAck(crate::dequeue_codec::PeekAck {
                header: Header::new(crate::Kind::PeekAck, 123, 456),
                value: vec![1, 2, 3],
                response_code: crate::SUCCESS,
            }),
            Packet::Len(crate::dequeue_codec::Len {
                header: Header::new(crate::Kind::Len, 123, 456),
                path: "hello".to_string(),
            }),
            Packet::LenAck(crate::dequeue_codec::LenAck {
                header: Header::new(crate::Kind::LenAck, 123, 456),

                len: 1,
                response_code: crate::SUCCESS,
            }),
            Packet::CreateQueue(crate::dequeue_codec::Create {
                header: Header::new(crate::Kind::CreateQueue, 123, 456),
                path: "hello".to_string(),
                node_size: 1,
            }),
            Packet::CreateQueueAck(crate::dequeue_codec::CreateAck {
                header: Header::new(crate::Kind::CreateQueueAck, 123, 456),

                response_code: crate::SUCCESS,
            }),
            Packet::DeleteQueue(crate::dequeue_codec::Delete {
                header: Header::new(crate::Kind::DeleteQueue, 123, 456),
                path: "hello".to_string(),
            }),
            Packet::DeleteQueueAck(crate::dequeue_codec::DeleteAck {
                header: Header::new(crate::Kind::DeleteQueueAck, 123, 456),

                response_code: crate::SUCCESS,
            }),
            Packet::Put(crate::kv_store_codec::Put {
                header: Header::new(crate::Kind::Put, 123, 456),
                key: TEST_KEY,
                value: vec![1, 2, 3],
            }),
            Packet::PutAck(crate::kv_store_codec::PutAck {
                header: Header::new(crate::Kind::PutAck, 123, 456),

                response_code: crate::SUCCESS,
            }),
            Packet::Get(crate::kv_store_codec::Get {
                header: Header::new(crate::Kind::Get, 123, 456),
                key: TEST_KEY,
            }),
            Packet::GetAck(crate::kv_store_codec::GetAck {
                header: Header::new(crate::Kind::GetAck, 123, 456),
                value: vec![1, 2, 3],
                response_code: crate::SUCCESS,
            }),
            Packet::Delete(crate::kv_store_codec::Delete {
                header: Header::new(crate::Kind::Delete, 123, 456),
                key: TEST_KEY,
            }),
            Packet::DeleteAck(crate::kv_store_codec::DeleteAck {
                header: Header::new(crate::Kind::DeleteAck, 123, 456),
                response_code: crate::SUCCESS,
            }),
            Packet::Chain(crate::system_codec::Chain {
                header: Header::new(crate::Kind::Chain, 123, 456),
                position: crate::system_codec::Position::Middle {
                    next: "foo".to_string(),
                },
            }),
            Packet::ChainAck(crate::system_codec::ChainAck {
                header: Header::new(crate::Kind::ChainAck, 123, 456),

                response_code: crate::SUCCESS,
            }),
            Packet::Join(crate::system_codec::Join {
                header: Header::new(crate::Kind::Join, 123, 456),
                position: crate::system_codec::Position::Middle {
                    next: "foo".to_string(),
                },
            }),
            Packet::JoinAck(crate::system_codec::JoinAck {
                header: Header::new(crate::Kind::JoinAck, 123, 456),
                response_code: crate::SUCCESS,
            }),
            Packet::Transfer(crate::system_codec::Transfer {
                header: Header::new(crate::Kind::Transfer, 123, 456),
                candidate: crate::system_codec::Position::Candidate {
                    candidate: "foo".to_string(),
                },
            }),
            Packet::TransferAck(crate::system_codec::TransferAck {
                header: Header::new(crate::Kind::TransferAck, 123, 456),
                response_code: crate::SUCCESS,
            }),
        ]
    }
}
