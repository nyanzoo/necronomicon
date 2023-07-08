use std::io::{Read, Write};

pub mod dequeue_codec;

mod error;
use dequeue_codec::{Dequeue, DequeueAck, Enqueue, EnqueueAck, Len, LenAck, Peek, PeekAck};
pub use error::Error;

mod header;
pub use header::{Header, Kind};
use kv_store_codec::{Delete, DeleteAck, Get, GetAck, Put, PutAck};

pub mod kv_store_codec;

pub mod system_codec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    // dequeue
    Enqueue(dequeue_codec::Enqueue),
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
    Patch(Vec<u8>),
}

pub trait Ack {
    fn header(&self) -> &Header;

    fn response_code(&self) -> u8;
}

/// # Description
/// After decoding the `Header`, the `PartialDecode` trait is used to decode the rest of the bytes.
pub trait PartialDecode {
    fn decode(header: Header, reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized;
}

pub fn partial_decode(header: Header, reader: &mut impl Read) -> Result<Packet, Error> {
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
        Kind::Patch => todo!(),
    };

    Ok(packet)
}

trait Decode {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait Encode {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error>;
}

//
// String + Vec impls
//

impl Decode for String {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
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

impl Encode for String {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        let bytes = self.as_bytes();
        let len = bytes.len() as u16;
        writer
            .write_all(&len.to_be_bytes())
            .map_err(Error::Encode)?;
        writer.write_all(bytes).map_err(Error::Encode)?;
        Ok(())
    }
}

impl Decode for Vec<u8> {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
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

impl Encode for Vec<u8> {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
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

impl Decode for u8 {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 1];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(bytes[0])
    }
}

impl Encode for u8 {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        writer.write_all(&[*self]).map_err(Error::Encode)?;
        Ok(())
    }
}

impl Decode for u16 {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 2];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(u16::from_be_bytes(bytes))
    }
}

impl Encode for u16 {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        writer
            .write_all(&self.to_be_bytes())
            .map_err(Error::Encode)?;
        Ok(())
    }
}

impl Decode for u32 {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 4];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(u32::from_be_bytes(bytes))
    }
}

impl Encode for u32 {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        writer
            .write_all(&self.to_be_bytes())
            .map_err(Error::Encode)?;
        Ok(())
    }
}

impl Decode for u64 {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut bytes = [0; 8];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(u64::from_be_bytes(bytes))
    }
}

impl Encode for u64 {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        writer
            .write_all(&self.to_be_bytes())
            .map_err(Error::Encode)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::{Decode, Encode};

    #[test_case::test_case(1u8; "u8")]
    #[test_case::test_case(1u16; "u16")]
    #[test_case::test_case(1u32; "u32")]
    #[test_case::test_case(1u64; "u64")]
    #[test_case::test_case("hello".to_string(); "string")]
    #[test_case::test_case(vec![1, 2, 3]; "vec")]
    fn test_encode_decode<T>(val: T)
    where
        T: Decode + Encode + Debug + Eq + PartialEq,
    {
        let mut bytes = vec![];
        val.encode(&mut bytes).unwrap();
        let decoded = T::decode(&mut bytes.as_slice()).unwrap();
        assert_eq!(val, decoded);
    }
}
