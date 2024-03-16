use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, ByteStr, Owned, Shared},
    header::{Uuid, Version},
    response::Response,
    DecodeOwned, Encode, Error, Header, Kind, PartialDecode,
};

use super::DeleteAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Delete<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) key: BinaryData<S>,
}

impl<S> Delete<S>
where
    S: Shared,
{
    pub fn new(version: impl Into<Version>, uuid: impl Into<Uuid>, key: BinaryData<S>) -> Self {
        Self {
            header: Header::new(Kind::Delete, version, uuid, key.len()),
            key,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }
    pub fn key(&self) -> &BinaryData<S> {
        &self.key
    }

    pub fn ack(self) -> DeleteAck<S> {
        DeleteAck {
            header: Header::new(Kind::DeleteAck, self.header.version, self.header.uuid, 0),
            response: Response::success(),
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> DeleteAck<S> {
        DeleteAck {
            header: Header::new(Kind::DeleteAck, self.header.version, self.header.uuid, 0),
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
        assert_eq!(header.kind, Kind::Delete);

        let key = BinaryData::decode_owned(reader, buffer)?;

        Ok(Self { header, key })
    }
}

impl<W, S> Encode<W> for Delete<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.key.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use crate::{
        kv_store_codec::test_key, tests::verify_encode_decode, Ack, Kind, Packet, INTERNAL_ERROR,
        SUCCESS,
    };

    use super::Delete;

    #[test]
    fn test_new() {
        let delete = Delete::new(0, 1, test_key());

        assert_eq!(delete.header().kind, Kind::Delete);
        assert_eq!(delete.header().version, 0.into());
        assert_eq!(delete.header().uuid, 1.into());
        assert_eq!(delete.key(), &test_key());
    }

    #[test]
    fn acks() {
        let delete = Delete::new(0, 1, test_key());

        let ack = delete.clone().ack();
        assert_eq!(ack.response().code(), SUCCESS);

        let nack = delete.nack(INTERNAL_ERROR, None);
        assert_eq!(nack.response().code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Delete(Delete::new(0, 1, test_key())));
    }
}
