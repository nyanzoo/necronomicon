use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::EnqueueAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Enqueue {
    pub(crate) header: Header,
    pub(crate) path: String,
    pub(crate) value: Vec<u8>,
}

impl Enqueue {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>, path: String, value: Vec<u8>) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::Enqueue),
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
    use crate::{tests::test_encode_decode_packet, Kind};

    use super::Enqueue;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Enqueue,
            Enqueue {
                path: "test".to_string(),
                value: vec![1, 2, 3],
            }
        );
    }
}
