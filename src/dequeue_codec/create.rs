use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::CreateAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Create {
    pub(crate) header: Header,
    pub(crate) path: String,
    pub(crate) node_size: u64,
}

impl Create {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>, path: String, node_size: u64) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::CreateQueue),
            path,
            node_size,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn node_size(&self) -> u64 {
        self.node_size
    }

    pub fn ack(self) -> CreateAck {
        CreateAck {
            header: Header::new(
                Kind::CreateQueueAck,
                self.header.version(),
                self.header.uuid(),
            ),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> CreateAck {
        CreateAck {
            header: Header::new(
                Kind::CreateQueueAck,
                self.header.version(),
                self.header.uuid(),
            ),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Create
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::CreateQueue);

        let path = String::decode(reader)?;
        let node_size = u64::decode(reader)?;

        Ok(Self {
            header,
            path,
            node_size,
        })
    }
}

impl<W> Encode<W> for Create
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.node_size.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{tests::test_encode_decode_packet, Kind};

    use super::Create;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::CreateQueue,
            Create {
                path: "test".to_string(),
                node_size: 1024,
            }
        );
    }
}
