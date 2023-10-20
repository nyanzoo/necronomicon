use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{DeleteAck, Key};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Delete {
    pub(crate) header: Header,
    pub(crate) key: Key,
}

impl Delete {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>, key: Key) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::Delete),
            key,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }
    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn ack(self) -> DeleteAck {
        DeleteAck {
            header: Header::new(Kind::DeleteAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> DeleteAck {
        DeleteAck {
            header: Header::new(Kind::DeleteAck, self.header.version(), self.header.uuid()),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Delete
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Delete);

        let key = Key::decode(reader)?;

        Ok(Self { header, key })
    }
}

impl<W> Encode<W> for Delete
where
    W: Write,
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
        kv_store_codec::TEST_KEY, tests::test_encode_decode_packet, Ack, Kind, INTERNAL_ERROR,
        SUCCESS,
    };

    use super::Delete;

    #[test]
    fn test_new() {
        let delete = Delete::new((0, 1), TEST_KEY);

        assert_eq!(delete.header().kind(), Kind::Delete);
        assert_eq!(delete.header().version(), 0);
        assert_eq!(delete.header().uuid(), 1);
        assert_eq!(delete.key(), &TEST_KEY);
    }

    #[test]
    fn test_acks() {
        let delete = Delete::new((0, 1), TEST_KEY);

        let ack = delete.clone().ack();
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = delete.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(Kind::Delete, Delete { key: TEST_KEY });
    }
}
