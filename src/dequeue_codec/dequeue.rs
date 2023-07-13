use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::DequeueAck;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Dequeue {
    pub(crate) header: Header,
    pub(crate) path: String,
}

impl Dequeue {
    pub fn new(header: Header, path: String) -> Self {
        assert_eq!(header.kind(), Kind::Dequeue);

        Self { header, path }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn ack(self, value: Vec<u8>) -> DequeueAck {
        DequeueAck {
            header: Header::new(Kind::DequeueAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
            value,
        }
    }

    pub fn nack(self, response_code: u8) -> DequeueAck {
        DequeueAck {
            header: Header::new(Kind::DequeueAck, self.header.version(), self.header.uuid()),
            response_code,
            value: Vec::new(),
        }
    }
}

impl<R> PartialDecode<R> for Dequeue
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Dequeue);

        let path = String::decode(reader)?;

        Ok(Self { header, path })
    }
}

impl<W> Encode<W> for Dequeue
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Decode, Encode, Header, Kind, PartialDecode};

    use super::Dequeue;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Dequeue, 123, 456);
        let mut buf = Vec::new();
        let dequeue = Dequeue {
            header,
            path: "test".to_string(),
        };
        dequeue.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = Dequeue::decode(header, &mut buf).unwrap();
        assert_eq!(dequeue, decoded);
    }
}
