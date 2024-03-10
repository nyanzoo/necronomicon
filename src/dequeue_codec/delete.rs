use std::io::{Read, Write};

use crate::{
    buffer::{ByteStr, Owned, Shared},
    header::{Uuid, Version},
    Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS,
};

use super::DeleteAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Delete<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) path: ByteStr<S>,
}

impl<S> Delete<S>
where
    S: Shared,
{
    pub fn new(version: impl Into<Version>, uuid: impl Into<Uuid>, path: ByteStr<S>) -> Self {
        Self {
            header: Header::new(Kind::DeleteQueue, version, uuid, path.len()),
            path,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &ByteStr<S> {
        &self.path
    }

    pub fn ack(self) -> DeleteAck {
        DeleteAck {
            header: Header::new(
                Kind::DeleteQueueAck,
                self.header.version,
                self.header.uuid,
                0,
            ),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> DeleteAck {
        DeleteAck {
            header: Header::new(
                Kind::DeleteQueueAck,
                self.header.version,
                self.header.uuid,
                0,
            ),
            response_code,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Delete<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::DeleteQueue);

        let path = ByteStr::decode(reader, buffer)?;

        Ok(Self { header, path })
    }
}

impl<W, S> Encode<W> for Delete<S>
where
    W: Write,
    S: Shared
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
        buffer::byte_str, tests::verify_encode_decode, Ack, Packet, INTERNAL_ERROR, SUCCESS,
    };

    use super::Delete;

    #[test]
    fn test_ack() {
        let delete = Delete::new(1, 2, byte_str(b"test"));

        let ack = delete.clone().ack();
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = delete.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::DeleteQueue(Delete::new(1, 2, byte_str(b"test"))));
    }
}
