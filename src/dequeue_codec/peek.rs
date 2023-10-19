use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::PeekAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Peek {
    pub(crate) header: Header,
    pub(crate) path: String,
}

impl Peek {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>, path: String) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::Peek),
            path,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn ack(self, value: Vec<u8>) -> PeekAck {
        PeekAck {
            header: Header::new(Kind::PeekAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
            value,
        }
    }

    pub fn nack(self, response_code: u8) -> PeekAck {
        PeekAck {
            header: Header::new(Kind::PeekAck, self.header.version(), self.header.uuid()),
            response_code,
            value: Vec::new(),
        }
    }
}

impl<R> PartialDecode<R> for Peek
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Peek);

        let path = String::decode(reader)?;

        Ok(Self { header, path })
    }
}

impl<W> Encode<W> for Peek
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
    use crate::{tests::test_encode_decode_packet, Kind};

    use super::Peek;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Peek,
            Peek {
                path: "test".to_string(),
            }
        );
    }
}
