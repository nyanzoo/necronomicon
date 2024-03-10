use std::io::{Read, Write};

use crate::{
    buffer::{ByteStr, Owned, Shared},
    header::{Uuid, Version},
    Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS,
};

use super::LenAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Len<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) path: ByteStr<S>,
}

impl<S> Len<S>
where
    S: Shared,
{
    pub fn new(version: impl Into<Version>, uuid: impl Into<Uuid>, path: ByteStr<S>) -> Self {
        Self {
            header: Header::new(Kind::Len, version, uuid, path.len()),
            path,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &ByteStr<S> {
        &self.path
    }

    pub fn ack(self, len: u64) -> LenAck {
        LenAck {
            header: Header::new(Kind::LenAck, self.header.version, self.header.uuid, 0),
            len,
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> LenAck {
        LenAck {
            header: Header::new(Kind::LenAck, self.header.version, self.header.uuid, 0),
            len: 0,
            response_code,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Len<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Len);

        let path = ByteStr::decode(reader, buffer)?;

        Ok(Self { header, path })
    }
}

impl<W, S> Encode<W> for Len<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::byte_str, tests::verify_encode_decode, Ack, Kind, Packet, INTERNAL_ERROR, SUCCESS,
    };

    use super::Len;

    #[test]
    fn test_new() {
        let len = Len::new(0, 1, byte_str(b"test"));

        assert_eq!(len.header().kind, Kind::Len);
        assert_eq!(len.header().version, 0.into());
        assert_eq!(len.header().uuid, 1.into());
        assert_eq!(len.path(), &byte_str(b"test"));
    }

    #[test]
    fn test_acks() {
        let len = Len::new(0, 1, byte_str(b"test"));

        let ack = len.clone().ack(1);
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = len.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::Len(Len::new(0, 1, byte_str(b"test"))));
    }
}
