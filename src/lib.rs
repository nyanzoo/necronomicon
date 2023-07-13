use std::io::{Read, Write};

mod codes;
pub use codes::{SERVER_BUSY, SUCCESS};

pub mod dequeue_codec;
use dequeue_codec::{Dequeue, DequeueAck, Enqueue, EnqueueAck, Len, LenAck, Peek, PeekAck};

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

trait Decode<R>
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

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, io::Cursor};

    use super::{Decode, Encode};

    #[test_case::test_case(1u8; "u8")]
    #[test_case::test_case(1u16; "u16")]
    #[test_case::test_case(1u32; "u32")]
    #[test_case::test_case(1u64; "u64")]
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
}
