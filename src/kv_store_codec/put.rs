use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{Key, PutAck};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Put {
    pub(crate) header: Header,
    pub(crate) key: Key,
    pub(crate) value: Vec<u8>,
}

impl Put {
    pub fn new(header: Header, key: Key, value: Vec<u8>) -> Self {
        assert_eq!(header.kind(), Kind::Put);

        Self { header, key, value }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }

    pub fn ack(self) -> PutAck {
        PutAck {
            header: Header::new(Kind::PutAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> PutAck {
        PutAck {
            header: Header::new(Kind::PutAck, self.header.version(), self.header.uuid()),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Put
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Put);

        let key = Key::decode(reader)?;
        let value = Vec::decode(reader)?;

        Ok(Self { header, key, value })
    }
}

impl<W> Encode<W> for Put
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.key.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{kv_store_codec::TEST_KEY, tests::test_encode_decode_packet, Kind};

    use super::Put;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Put,
            Put {
                key: TEST_KEY,
                value: vec![1, 2, 3],
            }
        );
    }
}
