use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{DeleteAck, Key};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Delete {
    pub(crate) header: Header,
    pub(crate) key: Key,
}

impl Delete {
    pub fn new(header: Header, key: Key) -> Self {
        assert_eq!(header.kind(), Kind::Delete);

        Self { header, key }
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

    use crate::{kv_store_codec::TEST_KEY, tests::test_encode_decode_packet, Kind};

    use super::Delete;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(Kind::Delete, Delete { key: TEST_KEY });
    }
}
