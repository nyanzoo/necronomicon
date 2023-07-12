use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::DeleteAck;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Delete {
    pub(crate) header: Header,
    pub(crate) path: String,
    pub(crate) key: String,
}

impl Delete {
    pub fn new(header: Header, path: String, key: String) -> Self {
        assert_eq!(header.kind(), Kind::Delete);

        Self { header, path, key }
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

        let path = String::decode(reader)?;
        let key = String::decode(reader)?;

        Ok(Self { header, path, key })
    }
}

impl<W> Encode<W> for Delete
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.key.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Decode, Encode, Header, Kind, PartialDecode};

    use super::Delete;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Delete, 123, 456);
        let mut buf = Vec::new();
        let delete = Delete {
            header,
            path: "test".to_string(),
            key: "test".to_string(),
        };
        delete.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = Delete::decode(header, &mut buf).unwrap();
        assert_eq!(delete, decoded);
    }
}
