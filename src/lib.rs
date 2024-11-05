#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::io::{Read, Write};

use log::{debug, trace};

mod buffer;
#[cfg(any(test, feature = "test"))]
pub use buffer::{binary_data, byte_str};
pub use buffer::{
    fill, BinaryData, BufferOwner, ByteStr, Owned, OwnedImpl, Pool, PoolImpl, Shared, SharedImpl,
};

mod codes;
pub use codes::{
    CHAIN_NOT_READY, FAILED_TO_PUSH_TO_TRANSACTION_LOG, INTERNAL_ERROR, KEY_ALREADY_EXISTS,
    KEY_DOES_NOT_EXIST, QUEUE_ALREADY_EXISTS, QUEUE_DOES_NOT_EXIST, QUEUE_EMPTY, QUEUE_FULL,
    SERVER_BUSY, SUCCESS,
};

pub mod deque_codec;
use deque_codec::{
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

pub mod response;
pub use response::Response;

pub mod system_codec;
use system_codec::{Join, JoinAck, Ping, PingAck, Report, ReportAck, Transfer, TransferAck};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DequePacket<S>
where
    S: Shared,
{
    Enqueue(Enqueue<S>),
    EnqueueAck(EnqueueAck<S>),
    Dequeue(Dequeue<S>),
    DequeueAck(DequeueAck<S>),
    Peek(Peek<S>),
    PeekAck(PeekAck<S>),
    Len(Len<S>),
    LenAck(LenAck<S>),
    CreateQueue(Create<S>),
    CreateQueueAck(CreateAck<S>),
    DeleteQueue(DeleteQueue<S>),
    DeleteQueueAck(DeleteQueueAck<S>),
}

impl<S> DequePacket<S>
where
    S: Shared,
{
    pub fn header(&self) -> Header {
        match self {
            Self::Enqueue(packet) => packet.header,
            Self::EnqueueAck(packet) => packet.header,
            Self::Dequeue(packet) => packet.header,
            Self::DequeueAck(packet) => packet.header,
            Self::Peek(packet) => packet.header,
            Self::PeekAck(packet) => packet.header,
            Self::Len(packet) => packet.header,
            Self::LenAck(packet) => packet.header,
            Self::CreateQueue(packet) => packet.header,
            Self::CreateQueueAck(packet) => packet.header,
            Self::DeleteQueue(packet) => packet.header,
            Self::DeleteQueueAck(packet) => packet.header,
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> Option<Self> {
        match self {
            Self::Enqueue(packet) => Some(Self::EnqueueAck(packet.nack(response_code, reason))),
            Self::Dequeue(packet) => Some(Self::DequeueAck(packet.nack(response_code, reason))),
            Self::Peek(packet) => Some(Self::PeekAck(packet.nack(response_code, reason))),
            Self::Len(packet) => Some(Self::LenAck(packet.nack(response_code, reason))),
            Self::CreateQueue(packet) => {
                Some(Self::CreateQueueAck(packet.nack(response_code, reason)))
            }
            Self::DeleteQueue(packet) => {
                Some(Self::DeleteQueueAck(packet.nack(response_code, reason)))
            }

            // acks
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorePacket<S>
where
    S: Shared,
{
    Put(Put<S>),
    PutAck(PutAck<S>),
    Get(Get<S>),
    GetAck(GetAck<S>),
    Delete(Delete<S>),
    DeleteAck(DeleteAck<S>),
}

impl<S> StorePacket<S>
where
    S: Shared,
{
    pub fn header(&self) -> Header {
        match self {
            Self::Put(packet) => packet.header,
            Self::PutAck(packet) => packet.header,
            Self::Get(packet) => packet.header,
            Self::GetAck(packet) => packet.header,
            Self::Delete(packet) => packet.header,
            Self::DeleteAck(packet) => packet.header,
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> Option<Self> {
        match self {
            Self::Put(packet) => Some(Self::PutAck(packet.nack(response_code, reason))),
            Self::Get(packet) => Some(Self::GetAck(packet.nack(response_code, reason))),
            Self::Delete(packet) => Some(Self::DeleteAck(packet.nack(response_code, reason))),

            // acks
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemPacket<S>
where
    S: Shared,
{
    Report(Report<S>),
    ReportAck(ReportAck<S>),
    Join(Join<S>),
    JoinAck(JoinAck<S>),
    Transfer(Transfer<S>),
    TransferAck(TransferAck<S>),
    Ping(Ping<S>),
    PingAck(PingAck<S>),
}

impl<S> SystemPacket<S>
where
    S: Shared,
{
    pub fn header(&self) -> Header {
        match self {
            Self::Report(packet) => packet.header,
            Self::ReportAck(packet) => packet.header,
            Self::Join(packet) => packet.header,
            Self::JoinAck(packet) => packet.header,
            Self::Transfer(packet) => packet.header,
            Self::TransferAck(packet) => packet.header,
            Self::Ping(packet) => packet.header,
            Self::PingAck(packet) => packet.header,
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> Option<Self> {
        match self {
            Self::Report(packet) => Some(Self::ReportAck(packet.nack(response_code, reason))),
            Self::Join(packet) => Some(Self::JoinAck(packet.nack(response_code, reason))),
            Self::Transfer(packet) => Some(Self::TransferAck(packet.nack(response_code, reason))),

            // acks & ping
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet<S>
where
    S: Shared,
{
    Deque(DequePacket<S>),
    Store(StorePacket<S>),
    System(SystemPacket<S>),
}

impl<S> From<DequePacket<S>> for Packet<S>
where
    S: Shared,
{
    fn from(packet: DequePacket<S>) -> Self {
        Self::Deque(packet)
    }
}

impl<S> From<StorePacket<S>> for Packet<S>
where
    S: Shared,
{
    fn from(packet: StorePacket<S>) -> Self {
        Self::Store(packet)
    }
}

impl<S> From<SystemPacket<S>> for Packet<S>
where
    S: Shared,
{
    fn from(packet: SystemPacket<S>) -> Self {
        Self::System(packet)
    }
}

impl<S> Packet<S>
where
    S: Shared,
{
    pub fn header(&self) -> Header {
        match self {
            Self::Deque(packet) => packet.header(),
            Self::Store(packet) => packet.header(),
            Self::System(packet) => packet.header(),
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> Option<Self> {
        match self {
            Self::Deque(packet) => packet
                .nack(response_code, reason)
                .map(|packet| Self::Deque(packet)),
            Self::Store(packet) => packet
                .nack(response_code, reason)
                .map(|packet| Self::Store(packet)),
            Self::System(packet) => packet
                .nack(response_code, reason)
                .map(|packet| Self::System(packet)),
        }
    }
}

pub trait Ack<S>
where
    S: Shared,
{
    fn header(&self) -> &Header;

    fn response(&self) -> Response<S>;
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
        // deque messages
        Kind::Enqueue => DequePacket::Enqueue(Enqueue::decode(header, reader, buffer)?).into(),
        Kind::EnqueueAck => {
            DequePacket::EnqueueAck(EnqueueAck::decode(header, reader, buffer)?).into()
        }
        Kind::Deque => DequePacket::Dequeue(Dequeue::decode(header, reader, buffer)?).into(),
        Kind::DequeAck => {
            DequePacket::DequeueAck(DequeueAck::decode(header, reader, buffer)?).into()
        }
        Kind::Peek => DequePacket::Peek(Peek::decode(header, reader, buffer)?).into(),
        Kind::PeekAck => DequePacket::PeekAck(PeekAck::decode(header, reader, buffer)?).into(),
        Kind::Len => DequePacket::Len(Len::decode(header, reader, buffer)?).into(),
        Kind::LenAck => DequePacket::LenAck(LenAck::decode(header, reader, buffer)?).into(),
        Kind::CreateQueue => {
            DequePacket::CreateQueue(Create::decode(header, reader, buffer)?).into()
        }
        Kind::CreateQueueAck => {
            DequePacket::CreateQueueAck(CreateAck::decode(header, reader, buffer)?).into()
        }
        Kind::DeleteQueue => {
            DequePacket::DeleteQueue(DeleteQueue::decode(header, reader, buffer)?).into()
        }
        Kind::DeleteQueueAck => {
            DequePacket::DeleteQueueAck(DeleteQueueAck::decode(header, reader, buffer)?).into()
        }

        // kv store messages
        Kind::Put => StorePacket::Put(Put::decode(header, reader, buffer)?).into(),
        Kind::PutAck => StorePacket::PutAck(PutAck::decode(header, reader, buffer)?).into(),
        Kind::Get => StorePacket::Get(Get::decode(header, reader, buffer)?).into(),
        Kind::GetAck => StorePacket::GetAck(GetAck::decode(header, reader, buffer)?).into(),
        Kind::Delete => StorePacket::Delete(Delete::decode(header, reader, buffer)?).into(),
        Kind::DeleteAck => {
            StorePacket::DeleteAck(DeleteAck::decode(header, reader, buffer)?).into()
        }

        // internal system messages
        Kind::Report => SystemPacket::Report(Report::decode(header, reader, buffer)?).into(),
        Kind::ReportAck => {
            SystemPacket::ReportAck(ReportAck::decode(header, reader, buffer)?).into()
        }
        Kind::Join => SystemPacket::Join(Join::decode(header, reader, buffer)?).into(),
        Kind::JoinAck => SystemPacket::JoinAck(JoinAck::decode(header, reader, buffer)?).into(),
        Kind::Transfer => SystemPacket::Transfer(Transfer::decode(header, reader, buffer)?).into(),
        Kind::TransferAck => {
            SystemPacket::TransferAck(TransferAck::decode(header, reader, buffer)?).into()
        }
        Kind::Ping => SystemPacket::Ping(Ping::decode(header, reader, buffer)?).into(),
        Kind::PingAck => SystemPacket::PingAck(PingAck::decode(header, reader, buffer)?).into(),
    };

    Ok(packet)
}

/// # Description
/// Attempts to fully decode a `Packet` from the given reader.
/// We use a buffer to avoid unnecessary allocations, but if the buffer is not large enough, we will
/// error.
///
/// # Arguments
/// * `reader` - The reader to decode from.
/// * `buffer` - The buffer to place the decoded value into.
/// * `previous_decoded_header` - The previous header that was decoded. This is useful for when
///   we failed to have enough buffer to decode the full packet. We can use this to try again with a new buffer.
///
/// # Errors
/// This function will return an error if the data cannot be decoded from the reader along with a previous header if any.
/// Or if the buffer is not large enough. See [`error::Error`] for more details.
///
/// # Returns
/// The decoded packet.
pub fn full_decode<R, O>(
    reader: &mut R,
    buffer: &mut O,
    previous_decoded_header: Option<Header>,
) -> Result<Packet<O::Shared>, Error>
where
    R: Read,
    O: Owned,
{
    trace!("previous_decoded_header: {:?}", previous_decoded_header);
    // decoding the header does not use up buffer space.
    let header = if let Some(header) = previous_decoded_header {
        header
    } else {
        Header::decode(reader)?
    };

    trace!("header '{:?}'", header);
    if header.len > buffer.unfilled_capacity() {
        return Err(Error::BufferTooSmallForPacketDecode {
            header,
            size: header.len,
            capacity: buffer.unfilled_capacity(),
        });
    }

    partial_decode(header, reader, buffer)
}

//
// Decode
//

/// # Description
/// The `DecodeOwned` trait is used to decode a value from a reader and place in an owned.
///
/// This does require the data to be copied out of the buffer and be owned by the buffer.
pub trait DecodeOwned<R, O>
where
    R: Read,
    O: Owned,
{
    /// # Description
    /// Copies data out of the reader and into the owned buffer.
    ///
    /// # Arguments
    /// * `reader` - The reader to decode from.
    /// * `buffer` - The buffer to place the decoded value into.
    ///
    /// # Errors
    /// This function will return an error if the data cannot be decoded from the reader.
    ///
    /// # Returns
    /// The decoded value.
    fn decode_owned(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized;
}

/// # Description
/// The `Decode` trait is used to decode a value from a reader.
///
/// This does require the data to be copied out of the buffer but not be owned by the buffer.
pub trait Decode<R> {
    /// # Description
    /// Takes data from the reader and decodes it into a value.
    ///
    /// # Arguments
    /// * `reader` - The reader to decode from.
    ///
    /// # Errors
    /// This function will return an error if the data cannot be decoded from the reader.
    ///
    /// # Returns
    /// The decoded value.
    fn decode(reader: &mut R) -> Result<Self, Error>
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

mod packet {
    use std::io::Write;

    use log::trace;

    use crate::{buffer::Shared, DequePacket, Encode, Error, Packet, StorePacket, SystemPacket};

    impl<W, S> Encode<W> for DequePacket<S>
    where
        W: Write,
        S: Shared,
    {
        fn encode(&self, writer: &mut W) -> Result<(), Error> {
            trace!("encode: {:?}", self);
            match self {
                Self::Enqueue(packet) => packet.encode(writer),
                Self::EnqueueAck(packet) => packet.encode(writer),
                Self::Dequeue(packet) => packet.encode(writer),
                Self::DequeueAck(packet) => packet.encode(writer),
                Self::Peek(packet) => packet.encode(writer),
                Self::PeekAck(packet) => packet.encode(writer),
                Self::Len(packet) => packet.encode(writer),
                Self::LenAck(packet) => packet.encode(writer),
                Self::CreateQueue(packet) => packet.encode(writer),
                Self::CreateQueueAck(packet) => packet.encode(writer),
                Self::DeleteQueue(packet) => packet.encode(writer),
                Self::DeleteQueueAck(packet) => packet.encode(writer),
            }
        }
    }

    impl<W, S> Encode<W> for StorePacket<S>
    where
        W: Write,
        S: Shared,
    {
        fn encode(&self, writer: &mut W) -> Result<(), Error> {
            trace!("encode: {:?}", self);
            match self {
                Self::Put(packet) => packet.encode(writer),
                Self::PutAck(packet) => packet.encode(writer),
                Self::Get(packet) => packet.encode(writer),
                Self::GetAck(packet) => packet.encode(writer),
                Self::Delete(packet) => packet.encode(writer),
                Self::DeleteAck(packet) => packet.encode(writer),
            }
        }
    }

    impl<W, S> Encode<W> for SystemPacket<S>
    where
        W: Write,
        S: Shared,
    {
        fn encode(&self, writer: &mut W) -> Result<(), Error> {
            trace!("encode: {:?}", self);
            match self {
                Self::Report(packet) => packet.encode(writer),
                Self::ReportAck(packet) => packet.encode(writer),
                Self::Join(packet) => packet.encode(writer),
                Self::JoinAck(packet) => packet.encode(writer),
                Self::Transfer(packet) => packet.encode(writer),
                Self::TransferAck(packet) => packet.encode(writer),
                Self::Ping(packet) => packet.encode(writer),
                Self::PingAck(packet) => packet.encode(writer),
            }
        }
    }

    impl<W, S> Encode<W> for Packet<S>
    where
        W: Write,
        S: Shared,
    {
        fn encode(&self, writer: &mut W) -> Result<(), Error> {
            trace!("encode: {:?}", self);
            match self {
                Self::Deque(packet) => packet.encode(writer),
                Self::Store(packet) => packet.encode(writer),
                Self::System(packet) => packet.encode(writer),
            }
        }
    }
}

mod integer {
    use std::io::{Read, Write};

    use crate::{Decode, Encode, Error};

    macro_rules! impl_integer_decode {
        ($($t:ty),+) => {
            $(
                impl<R> Decode<R> for $t
                where
                    R: Read,
                {
                    fn decode(reader: &mut R) -> Result<Self, Error>
                    where
                        Self: Sized,
                    {
                        let mut bytes = [0; std::mem::size_of::<$t>()];
                        reader.read_exact(&mut bytes).map_err(|source| Error::Decode {
                            kind: stringify!($t),
                            buffer: None,
                            source: source.into(),
                        })?;
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
                        writer
                            .write_all(&data)
                            .map_err(|source| Error::Encode {
                                kind: stringify!($t),
                                source: source.into(),
                            })
                    }
                }
            )+
        };
    }

    impl_integer_encode!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
}

mod option {
    use std::io::{Read, Write};

    use crate::{buffer::Owned, Decode, DecodeOwned, Encode, Error};

    impl<R, T> Decode<R> for Option<T>
    where
        R: Read,
        T: Decode<R>,
    {
        fn decode(reader: &mut R) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let is_some = u8::decode(reader)? > 0;
            if is_some {
                let value = T::decode(reader)?;
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
            writer.write_all(self).map_err(|source| Error::Encode {
                kind: "&[u8]",
                source: source.into(),
            })?;
            Ok(())
        }
    }
}

mod vector {
    use std::io::{Read, Write};

    use crate::{buffer::Owned, Decode, DecodeOwned, Encode, Error};

    impl<R, T> Decode<R> for Vec<T>
    where
        R: Read,
        T: Decode<R>,
    {
        fn decode(reader: &mut R) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let len = usize::decode(reader)?;
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(T::decode(reader)?);
            }

            Ok(vec)
        }
    }

    impl<R, T, O> DecodeOwned<R, O> for Vec<T>
    where
        R: Read,
        T: DecodeOwned<R, O>,
        O: Owned,
    {
        fn decode_owned(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let len = usize::decode(reader)?;
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(T::decode_owned(reader, buffer)?);
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
        DecodeOwned, DequePacket, Packet, Response, StorePacket, SystemPacket,
    };

    use super::{Decode, Encode};

    pub fn verify_encode_decode(val: impl Into<Packet<SharedImpl>>) {
        let val = val.into();
        let mut bytes = vec![];
        val.encode(&mut bytes).unwrap();
        let mut cursor = Cursor::new(bytes);

        let pool = PoolImpl::new(1024, 1);
        let mut buffer = pool.acquire("test", "full decode");

        let decoded = full_decode(&mut cursor, &mut buffer, None).unwrap();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_full_decode_packets() {
        let packets = test_packets();

        for packet in packets {
            verify_encode_decode(packet);
        }
    }

    #[test_case::test_case(1u8; "u8")]
    #[test_case::test_case(1u16; "u16")]
    #[test_case::test_case(1u32; "u32")]
    #[test_case::test_case(1u64; "u64")]
    #[test_case::test_case(1usize; "usize")]
    #[test_case::test_case(vec![1, 2, 3]; "vec")]
    #[test_case::test_case(Some(1u8); "option")]
    fn encode_decode<T>(val: T)
    where
        T: Decode<Cursor<Vec<u8>>> + Encode<Vec<u8>> + Debug + PartialEq,
    {
        let mut bytes = vec![];
        val.encode(&mut bytes).unwrap();
        let mut cursor = Cursor::new(bytes);

        let decoded = T::decode(&mut cursor).unwrap();
        assert_eq!(val, decoded);
    }

    #[test_case::test_case(vec![byte_str(b"kittens")]; "vec")]
    #[test_case::test_case(Some(byte_str(b"data")); "option")]
    #[test_case::test_case(vec![Role::Backend(byte_str(b"test")), Role::Observer]; "role")]
    fn encode_decode_owned<T>(val: T)
    where
        T: DecodeOwned<Cursor<Vec<u8>>, OwnedImpl> + Encode<Vec<u8>> + Debug + PartialEq,
    {
        let mut bytes = vec![];
        val.encode(&mut bytes).unwrap();
        let mut cursor = Cursor::new(bytes);

        let pool = PoolImpl::new(1024, 1);
        let mut buffer = pool.acquire("test", "decode owned T");

        let decoded = T::decode_owned(&mut cursor, &mut buffer).unwrap();
        assert_eq!(val, decoded);
    }

    fn test_packets() -> Vec<Packet<SharedImpl>> {
        vec![
            DequePacket::Enqueue(crate::deque_codec::Enqueue::new(
                123,
                456,
                byte_str(b"hello"),
                binary_data(&[1, 2, 3]),
            ))
            .into(),
            DequePacket::EnqueueAck(crate::deque_codec::EnqueueAck::new(Response::success()))
                .into(),
            DequePacket::Dequeue(crate::deque_codec::Dequeue::new(
                123,
                456,
                byte_str(b"test"),
            ))
            .into(),
            DequePacket::DequeueAck(crate::deque_codec::DequeueAck::new(
                Response::success(),
                None,
            ))
            .into(),
            DequePacket::Peek(crate::deque_codec::Peek::new(1, 1, byte_str(b"test"), 0)).into(),
            DequePacket::PeekAck(crate::deque_codec::PeekAck::new(Response::success(), None))
                .into(),
            DequePacket::Len(crate::deque_codec::Len::new(1, 1, byte_str(b"test"))).into(),
            DequePacket::LenAck(crate::deque_codec::LenAck::new(Response::success(), 1)).into(),
            DequePacket::CreateQueue(crate::deque_codec::Create::new(
                1,
                1,
                byte_str(b"test"),
                123,
                1024,
            ))
            .into(),
            DequePacket::CreateQueueAck(crate::deque_codec::CreateAck::new(Response::success()))
                .into(),
            DequePacket::DeleteQueue(crate::deque_codec::Delete::new(1, 1, byte_str(b"test")))
                .into(),
            DequePacket::DeleteQueueAck(crate::deque_codec::DeleteAck::new(Response::success()))
                .into(),
            StorePacket::Put(crate::kv_store_codec::Put::new(
                1,
                1,
                test_key(),
                binary_data(&[1, 2, 3]),
            ))
            .into(),
            StorePacket::PutAck(crate::kv_store_codec::PutAck::new(Response::success())).into(),
            StorePacket::Get(crate::kv_store_codec::Get::new(123, 456, test_key())).into(),
            StorePacket::GetAck(crate::kv_store_codec::GetAck::new(
                Response::success(),
                Some(binary_data(&[1, 2, 3])).into(),
            ))
            .into(),
            StorePacket::Delete(crate::kv_store_codec::Delete::new(123, 456, test_key())).into(),
            StorePacket::DeleteAck(crate::kv_store_codec::DeleteAck::new(Response::success()))
                .into(),
            SystemPacket::Report(Report::new(
                123,
                456,
                Position::Middle {
                    next: byte_str(b"next"),
                },
            ))
            .into(),
            SystemPacket::ReportAck(ReportAck::new(Response::success())).into(),
            SystemPacket::Join(Join::new(
                123,
                456,
                Role::Backend(byte_str(b"backend")).into(),
                1,
                false,
            ))
            .into(),
            SystemPacket::JoinAck(JoinAck::new(Response::success(), 1)).into(),
            SystemPacket::Transfer(Transfer::new(
                123,
                456,
                byte_str(b"/tmp/kitties"),
                42,
                binary_data(&[1, 2, 3]),
            ))
            .into(),
            SystemPacket::TransferAck(TransferAck::new(Response::success())).into(),
        ]
    }
}
