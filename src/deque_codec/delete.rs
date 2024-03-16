use std::io::{Read, Write};

use crate::{
    buffer::{ByteStr, Owned, Shared},
    header::{Uuid, Version},
    DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response,
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

    pub fn ack(self) -> DeleteAck<S> {
        DeleteAck {
            header: Header::new(
                Kind::DeleteQueueAck,
                self.header.version,
                self.header.uuid,
                0,
            ),
            response: Response::success(),
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> DeleteAck<S> {
        DeleteAck {
            header: Header::new(
                Kind::DeleteQueueAck,
                self.header.version,
                self.header.uuid,
                0,
            ),
            response: Response::fail(response_code, reason),
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

        let path = ByteStr::decode_owned(reader, buffer)?;

        Ok(Self { header, path })
    }
}

impl<W, S> Encode<W> for Delete<S>
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
        buffer::byte_str, tests::verify_encode_decode, Ack, Packet, INTERNAL_ERROR, SUCCESS,
    };

    use super::Delete;

    #[test]
    fn test_ack() {
        let delete = Delete::new(1, 2, byte_str(b"test"));

        let ack = delete.clone().ack();
        assert_eq!(ack.response().code(), SUCCESS);

        let nack = delete.nack(INTERNAL_ERROR, None);
        assert_eq!(nack.response().code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::DeleteQueue(Delete::new(1, 2, byte_str(b"test"))));
    }
}
