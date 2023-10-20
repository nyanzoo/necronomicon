use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::DequeueAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Dequeue {
    pub(crate) header: Header,
    pub(crate) path: String,
}

impl Dequeue {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>, path: String) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::Dequeue),
            path,
        }
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
    use crate::{tests::test_encode_decode_packet, Ack, Kind, INTERNAL_ERROR, SUCCESS};

    use super::Dequeue;

    #[test]
    fn test_new() {
        let dequeue = Dequeue::new((1, 2), "test".to_string());

        assert_eq!(dequeue.header().version(), 1);
        assert_eq!(dequeue.header().uuid(), 2);
        assert_eq!(dequeue.path(), "test");
    }

    #[test]
    fn test_acks() {
        let dequeue = Dequeue::new((1, 2), "test".to_string());

        let ack = dequeue.clone().ack(vec![1, 2, 3]);
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = dequeue.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Dequeue,
            Dequeue {
                path: "test".to_string(),
            }
        );
    }
}
