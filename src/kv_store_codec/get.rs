use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{GetAck, Key};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Get {
    pub(crate) header: Header,
    pub(crate) key: Key,
}

impl Get {
    pub fn new(header: Header, key: Key) -> Self {
        assert_eq!(header.kind(), Kind::Get);

        Self { header, key }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn ack(self, value: Vec<u8>) -> GetAck {
        GetAck {
            header: Header::new(Kind::GetAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
            value,
        }
    }

    pub fn nack(self, response_code: u8) -> GetAck {
        GetAck {
            header: Header::new(Kind::GetAck, self.header.version(), self.header.uuid()),
            response_code,
            value: Vec::new(),
        }
    }
}

impl<R> PartialDecode<R> for Get
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Get);

        let key = Key::decode(reader)?;

        Ok(Self { header, key })
    }
}

impl<W> Encode<W> for Get
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

    use super::Get;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(Kind::Get, Get { key: TEST_KEY });
    }
}
