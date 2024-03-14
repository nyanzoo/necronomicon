#![cfg_attr(nightly, feature(no_coverage))]

use std::io::{Read, Write};

use log::debug;

mod buffer;
#[cfg(any(test, feature = "test"))]
pub use buffer::{binary_data, byte_str};
pub use buffer::{
    fill, BinaryData, Block, BlockMut, Owned, OwnedImpl, Pool, PoolImpl, Shared, SharedImpl,
};

mod codes;
pub use codes::{
    CHAIN_NOT_READY, FAILED_TO_PUSH_TO_TRANSACTION_LOG, INTERNAL_ERROR, KEY_ALREADY_EXISTS,
    KEY_DOES_NOT_EXIST, QUEUE_ALREADY_EXISTS, QUEUE_DOES_NOT_EXIST, QUEUE_EMPTY, QUEUE_FULL,
    SERVER_BUSY, SUCCESS,
};

pub mod dequeue_codec;
use dequeue_codec::{
    Create, CreateAck, Delete as DeleteQueue, DeleteAck as DeleteQueueAck, Dequeue, DequeueAck,
    Enqueue, EnqueueAck, Len, LenAck, Peek, PeekAck,
};

mod error;
pub use error::Error;

mod header;
pub use header::{Header, Uuid, Version};

mod kind;
pub use kind::Kind;

pub mod kv_store_codec;
use kv_store_codec::{Delete, DeleteAck, Get, GetAck, Put, PutAck};

pub mod system_codec;
use system_codec::{Join, JoinAck, Ping, PingAck, Report, ReportAck, Transfer, TransferAck};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet<S>
where
    S: Shared,
{
    // dequeue
    Enqueue(Enqueue<S>),
    EnqueueAck(EnqueueAck),
    Dequeue(Dequeue<S>),
    DequeueAck(DequeueAck<S>),
    Peek(Peek<S>),
    PeekAck(PeekAck<S>),
    Len(Len<S>),
    LenAck(LenAck),
    CreateQueue(Create<S>),
    CreateQueueAck(CreateAck),
    DeleteQueue(DeleteQueue<S>),
    DeleteQueueAck(DeleteQueueAck),

    // kv store
    Put(Put<S>),
    PutAck(PutAck),
    Get(Get<S>),
    GetAck(GetAck<S>),
    Delete(Delete<S>),
    DeleteAck(DeleteAck),

    // internal system messages
    Report(Report<S>),
    ReportAck(ReportAck),
    Join(Join<S>),
    JoinAck(JoinAck),
    Transfer(Transfer<S>),
    TransferAck(TransferAck),
    Ping(Ping),
    PingAck(PingAck),
}

impl<S> Packet<S>
where
    S: Shared,
{
    pub fn header(&self) -> Header {
        match self {
            // dequeue
            Packet::Enqueue(packet) => packet.header,
            Packet::EnqueueAck(packet) => packet.header,
            Packet::Dequeue(packet) => packet.header,
            Packet::DequeueAck(packet) => packet.header,
            Packet::Peek(packet) => packet.header,
            Packet::PeekAck(packet) => packet.header,
            Packet::Len(packet) => packet.header,
            Packet::LenAck(packet) => packet.header,
            Packet::CreateQueue(packet) => packet.header,
            Packet::CreateQueueAck(packet) => packet.header,
            Packet::DeleteQueue(packet) => packet.header,
            Packet::DeleteQueueAck(packet) => packet.header,

            // kv store
            Packet::Put(packet) => packet.header,
            Packet::PutAck(packet) => packet.header,
            Packet::Get(packet) => packet.header,
            Packet::GetAck(packet) => packet.header,
            Packet::Delete(packet) => packet.header,
            Packet::DeleteAck(packet) => packet.header,

            // internal system messages
            Packet::Report(packet) => packet.header,
            Packet::ReportAck(packet) => packet.header,
            Packet::Join(packet) => packet.header,
            Packet::JoinAck(packet) => packet.header,
            Packet::Transfer(packet) => packet.header,
            Packet::TransferAck(packet) => packet.header,
            Packet::Ping(packet) => packet.header,
            Packet::PingAck(packet) => packet.header,
        }
    }

    pub fn nack(self, response_code: u8) -> Option<Self> {
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
            Packet::Report(this) => Some(Packet::ReportAck(this.nack(response_code))),
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
pub trait PartialDecode<R, O>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized;
}

pub fn partial_decode<R, O>(
    header: Header,
    reader: &mut R,
    buffer: &mut O,
) -> Result<Packet<O::Shared>, Error>
where
    R: Read,
    O: Owned,
{
    debug!("partial_decode: {:?}", header);
    let packet = match header.kind {
        // dequeue messages
        Kind::Enqueue => Packet::Enqueue(Enqueue::decode(header, reader, buffer)?),
        Kind::EnqueueAck => Packet::EnqueueAck(EnqueueAck::decode(header, reader, buffer)?),
        Kind::Dequeue => Packet::Dequeue(Dequeue::decode(header, reader, buffer)?),
        Kind::DequeueAck => Packet::DequeueAck(DequeueAck::decode(header, reader, buffer)?),
        Kind::Peek => Packet::Peek(Peek::decode(header, reader, buffer)?),
        Kind::PeekAck => Packet::PeekAck(PeekAck::decode(header, reader, buffer)?),
        Kind::Len => Packet::Len(Len::decode(header, reader, buffer)?),
        Kind::LenAck => Packet::LenAck(LenAck::decode(header, reader, buffer)?),
        Kind::CreateQueue => Packet::CreateQueue(Create::decode(header, reader, buffer)?),
        Kind::CreateQueueAck => Packet::CreateQueueAck(CreateAck::decode(header, reader, buffer)?),
        Kind::DeleteQueue => Packet::DeleteQueue(DeleteQueue::decode(header, reader, buffer)?),
        Kind::DeleteQueueAck => {
            Packet::DeleteQueueAck(DeleteQueueAck::decode(header, reader, buffer)?)
        }

        // kv store messages
        Kind::Put => Packet::Put(Put::decode(header, reader, buffer)?),
        Kind::PutAck => Packet::PutAck(PutAck::decode(header, reader, buffer)?),
        Kind::Get => Packet::Get(Get::decode(header, reader, buffer)?),
        Kind::GetAck => Packet::GetAck(GetAck::decode(header, reader, buffer)?),
        Kind::Delete => Packet::Delete(Delete::decode(header, reader, buffer)?),
        Kind::DeleteAck => Packet::DeleteAck(DeleteAck::decode(header, reader, buffer)?),

        // internal system messages
        Kind::Report => Packet::Report(Report::decode(header, reader, buffer)?),
        Kind::ReportAck => Packet::ReportAck(ReportAck::decode(header, reader, buffer)?),
        Kind::Join => Packet::Join(Join::decode(header, reader, buffer)?),
        Kind::JoinAck => Packet::JoinAck(JoinAck::decode(header, reader, buffer)?),
        Kind::Transfer => Packet::Transfer(Transfer::decode(header, reader, buffer)?),
        Kind::TransferAck => Packet::TransferAck(TransferAck::decode(header, reader, buffer)?),
        Kind::Ping => Packet::Ping(Ping::decode(header, reader, buffer)?),
        Kind::PingAck => Packet::PingAck(PingAck::decode(header, reader, buffer)?),
    };

    Ok(packet)
}

/// Attempts to fully decode a `Packet` from the given reader.
/// We use a buffer to avoid unnecessary allocations, but if the buffer is not large enough, we will
/// error.
pub fn full_decode<R, O>(reader: &mut R, buffer: &mut O) -> Result<Packet<O::Shared>, Error>
where
    R: Read,
    O: Owned,
{
    // decoding the header does not use up buffer space.
    let header = Header::decode(reader, buffer)?;

    if header.len > buffer.unfilled_capacity() {
        return Err(Error::OwnedRemaining {
            acquire: header.len,
            capacity: buffer.unfilled_capacity(),
        });
    }

    partial_decode(header, reader, buffer)
}

//
// Decode
//

pub trait Decode<R, O>
where
    R: Read,
    O: Owned,
{
    fn decode(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized;
}

//
// Encode
//

pub trait Encode<W>
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error>;
}

pub fn write_all<W>(writer: &mut W, bytes: &[u8]) -> Result<usize, Error>
where
    W: Write,
{
    let written = writer.write(bytes).map_err(Error::Encode)?;
    if written != bytes.len() {
        return Err(Error::Encode(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to write all bytes",
        )));
    }
    Ok(written)
}

mod packet {
    use std::io::Write;

    use log::debug;

    use crate::{buffer::Shared, Encode, Error, Packet};

    impl<W, S> Encode<W> for Packet<S>
    where
        W: Write,
        S: Shared,
    {
        fn encode(&self, writer: &mut W) -> Result<(), Error> {
            debug!("encode: {:?}", self);
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
                Packet::Report(packet) => packet.encode(writer),
                Packet::ReportAck(packet) => packet.encode(writer),
                Packet::Join(packet) => packet.encode(writer),
                Packet::JoinAck(packet) => packet.encode(writer),
                Packet::Transfer(packet) => packet.encode(writer),
                Packet::TransferAck(packet) => packet.encode(writer),
                Packet::Ping(packet) => packet.encode(writer),
                Packet::PingAck(packet) => packet.encode(writer),
            }
        }
    }
}

mod integer {
    use std::io::{Read, Write};

    use crate::{buffer::Owned, Decode, Encode, Error};

    macro_rules! impl_integer_decode {
        ($($t:ty),+) => {
            $(
                impl<R, O> Decode<R, O> for $t
                where
                    R: Read,
                    O: Owned,
                {
                    fn decode(reader: &mut R, _: &mut O) -> Result<Self, Error>
                    where
                        Self: Sized,
                    {
                        let mut bytes = [0; std::mem::size_of::<$t>()];
                        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
                        Ok(<$t>::from_be_bytes(bytes))
                    }
                }
            )+
        };
    }

    impl_integer_decode!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

    macro_rules! impl_integer_encode {
        ($($t:ty),+) => {
            $(
                impl<W> Encode<W> for $t
                where
                    W: Write,
                {
                    fn encode(&self, writer: &mut W) -> Result<(), Error> {
                        let data = self.to_be_bytes();
                        let bytes = writer
                            .write(&data)
                            .map_err(Error::Encode)?;
                        if bytes != data.len() {
                            return Err(Error::Encode(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "failed to write all bytes",
                            )));
                        }
                        Ok(())
                    }
                }
            )+
        };
    }

    impl_integer_encode!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
}

mod option {
    use std::io::{Read, Write};

    use crate::{buffer::Owned, Decode, Encode, Error};

    impl<R, T, O> Decode<R, O> for Option<T>
    where
        R: Read,
        T: Decode<R, O>,
        O: Owned,
    {
        fn decode(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let is_some = u8::decode(reader, buffer)? > 0;
            if is_some {
                let value = T::decode(reader, buffer)?;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    impl<R, T, O> DecodeOwned<R, O> for Option<T>
    where
        R: Read,
        T: DecodeOwned<R, O>,
        O: Owned,
    {
        fn decode_owned(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let is_some = u8::decode(reader)? > 0;
            if is_some {
                let value = T::decode_owned(reader, buffer)?;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    impl<W, T> Encode<W> for Option<T>
    where
        W: Write,
        T: Encode<W>,
    {
        fn encode(&self, writer: &mut W) -> Result<(), Error> {
            match self {
                Some(value) => {
                    1u8.encode(writer)?;
                    value.encode(writer)?;
                }
                None => {
                    0u8.encode(writer)?;
                }
            }

            Ok(())
        }
    }
}

mod slice {
    use std::io::Write;

    use crate::{Encode, Error};

    impl<W> Encode<W> for &[u8]
    where
        W: Write,
    {
        fn encode(&self, writer: &mut W) -> Result<(), Error> {
            self.len().encode(writer)?;
            writer.write_all(self).map_err(Error::Encode)?;
            Ok(())
        }
    }
}

mod vector {
    use std::io::{Read, Write};

    use crate::{buffer::Owned, Decode, Encode, Error};

    impl<R, T, O> Decode<R, O> for Vec<T>
    where
        R: Read,
        T: Decode<R, O>,
        O: Owned,
    {
        fn decode(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let len = usize::decode(reader, buffer)?;
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(T::decode(reader, buffer)?);
            }

            Ok(vec)
        }
    }

    impl<W, T> Encode<W> for Vec<T>
    where
        W: Write,
        T: Encode<W>,
    {
        fn encode(&self, writer: &mut W) -> Result<(), Error> {
            self.len().encode(writer)?;
            for value in self {
                value.encode(writer)?;
            }

            Ok(())
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::{fmt::Debug, io::Cursor};

    use crate::{
        buffer::{binary_data, byte_str, OwnedImpl, Pool, PoolImpl, SharedImpl},
        full_decode,
        kv_store_codec::test_key,
        system_codec::*,
        Packet, SUCCESS,
    };

    use super::{Decode, Encode};

    pub fn verify_encode_decode(val: Packet<SharedImpl>) {
        let mut bytes = vec![];
        val.encode(&mut bytes).unwrap();
        let mut cursor = Cursor::new(bytes);

        let pool = PoolImpl::new(1024, 1);
        let mut buffer = pool.acquire().expect("acquire");

        let decoded = full_decode(&mut cursor, &mut buffer).unwrap();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_full_decode_packets() {
        let packets = test_packets();

        for packet in packets {
            let mut bytes = vec![];
            packet.encode(&mut bytes).unwrap();
            let mut cursor = Cursor::new(bytes);

            let pool = PoolImpl::new(1024, 1);
            let mut buffer = pool.acquire().expect("acquire");

            let decoded = full_decode(&mut cursor, &mut buffer).unwrap();
            assert_eq!(packet, decoded);
        }
    }

    #[test_case::test_case(1u8; "u8")]
    #[test_case::test_case(1u16; "u16")]
    #[test_case::test_case(1u32; "u32")]
    #[test_case::test_case(1u64; "u64")]
    #[test_case::test_case(1usize; "usize")]
    #[test_case::test_case(vec![1, 2, 3]; "vec")]
    #[test_case::test_case(Some(1u8); "option")]
    #[test_case::test_case(vec![Role::Backend(byte_str(b"test")), Role::Observer]; "role")]
    fn test_encode_decode<T>(val: T)
    where
        T: Decode<Cursor<Vec<u8>>, OwnedImpl> + Encode<Vec<u8>> + Debug + PartialEq,
    {
        let mut bytes = vec![];
        val.encode(&mut bytes).unwrap();
        let mut cursor = Cursor::new(bytes);

        let pool = PoolImpl::new(1024, 1);
        let mut buffer = pool.acquire().expect("acquire");

        let decoded = T::decode(&mut cursor, &mut buffer).unwrap();
        assert_eq!(val, decoded);
    }

    fn test_packets() -> Vec<Packet<SharedImpl>> {
        vec![
            Packet::Enqueue(crate::dequeue_codec::Enqueue::new(
                123,
                456,
                byte_str(b"hello"),
                binary_data(&[1, 2, 3]),
            )),
            Packet::EnqueueAck(crate::dequeue_codec::EnqueueAck::new(SUCCESS)),
            Packet::Dequeue(crate::dequeue_codec::Dequeue::new(
                123,
                456,
                byte_str(b"test"),
            )),
            Packet::DequeueAck(crate::dequeue_codec::DequeueAck::new(SUCCESS, None)),
            Packet::Peek(crate::dequeue_codec::Peek::new(1, 1, byte_str(b"test"))),
            Packet::PeekAck(crate::dequeue_codec::PeekAck::new(SUCCESS, None)),
            Packet::Len(crate::dequeue_codec::Len::new(1, 1, byte_str(b"test"))),
            Packet::LenAck(crate::dequeue_codec::LenAck::new(SUCCESS, 1)),
            Packet::CreateQueue(crate::dequeue_codec::Create::new(
                1,
                1,
                byte_str(b"test"),
                123,
            )),
            Packet::CreateQueueAck(crate::dequeue_codec::CreateAck::new(SUCCESS)),
            Packet::DeleteQueue(crate::dequeue_codec::Delete::new(1, 1, byte_str(b"test"))),
            Packet::DeleteQueueAck(crate::dequeue_codec::DeleteAck::new(SUCCESS)),
            Packet::Put(crate::kv_store_codec::Put::new(
                1,
                1,
                test_key(),
                binary_data(&[1, 2, 3]),
            )),
            Packet::PutAck(crate::kv_store_codec::PutAck::new(SUCCESS)),
            Packet::Get(crate::kv_store_codec::Get::new(123, 456, test_key())),
            Packet::GetAck(crate::kv_store_codec::GetAck::new(
                SUCCESS,
                Some(binary_data(&[1, 2, 3])),
            )),
            Packet::Delete(crate::kv_store_codec::Delete::new(123, 456, test_key())),
            Packet::DeleteAck(crate::kv_store_codec::DeleteAck::new(SUCCESS)),
            Packet::Report(Report::new(
                123,
                456,
                Position::Middle {
                    next: byte_str(b"next"),
                },
            )),
            Packet::ReportAck(ReportAck::new(SUCCESS)),
            Packet::Join(Join::new(
                123,
                456,
                Role::Backend(byte_str(b"backend")),
                1,
                false,
            )),
            Packet::JoinAck(JoinAck::new(SUCCESS)),
            Packet::Transfer(Transfer::new(
                123,
                456,
                byte_str(b"/tmp/kitties"),
                binary_data(&[1, 2, 3]),
            )),
            Packet::TransferAck(TransferAck::new(SUCCESS)),
        ]
    }
}
