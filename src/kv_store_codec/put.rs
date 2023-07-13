use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::PutAck;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Put {
    pub(crate) header: Header,
    pub(crate) path: String,
    pub(crate) key: String,
    pub(crate) value: Vec<u8>,
}

impl Put {
    pub fn new(header: Header, path: String, key: String, value: Vec<u8>) -> Self {
        assert_eq!(header.kind(), Kind::Put);

        Self {
            header,
            path,
            key,
            value,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn key(&self) -> &str {
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

        let path = String::decode(reader)?;
        let key = String::decode(reader)?;
        let value = Vec::decode(reader)?;

        Ok(Self {
            header,
            path,
            key,
            value,
        })
    }
}

impl<W> Encode<W> for Put
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.key.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Decode, Encode, Header, Kind, PartialDecode};

    use super::Put;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Put, 123, 456);
        let mut buf = Vec::new();
        let put = Put {
            header,
            path: "test".to_string(),
            key: "test".to_string(),
            value: vec![1, 2, 3],
        };
        put.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = Put::decode(header, &mut buf).unwrap();
        assert_eq!(put, decoded);
    }
}
