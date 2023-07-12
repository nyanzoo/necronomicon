use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::EnqueueAck;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Enqueue {
    header: Header,
    path: String,
    value: Vec<u8>,
}

impl Enqueue {
    pub fn new(header: Header, path: String, value: Vec<u8>) -> Self {
        assert_eq!(header.kind(), Kind::Enqueue);

        Self {
            header,
            path,
            value,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }

    pub fn ack(self) -> EnqueueAck {
        EnqueueAck {
            header: Header::new(Kind::EnqueueAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> EnqueueAck {
        EnqueueAck {
            header: Header::new(Kind::EnqueueAck, self.header.version(), self.header.uuid()),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Enqueue
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Enqueue);

        let path = String::decode(reader)?;
        let value = Vec::<u8>::decode(reader)?;

        Ok(Self {
            header,
            path,
            value,
        })
    }
}

impl<W> Encode<W> for Enqueue
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{header::Kind, Decode, Encode, Header, PartialDecode};

    use super::Enqueue;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Enqueue, 123, 456);
        let mut buf = Vec::new();
        let enqueue = Enqueue {
            header,
            path: "test".to_string(),
            value: vec![1, 2, 3],
        };
        enqueue.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = Enqueue::decode(header, &mut buf).unwrap();
        assert_eq!(enqueue, decoded);
    }
}
