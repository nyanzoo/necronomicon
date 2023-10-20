use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::DeleteAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Delete {
    pub(crate) header: Header,
    pub(crate) path: String,
}

impl Delete {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>, path: String) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::DeleteQueue),
            path,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn ack(self) -> DeleteAck {
        DeleteAck {
            header: Header::new(
                Kind::DeleteQueueAck,
                self.header.version(),
                self.header.uuid(),
            ),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> DeleteAck {
        DeleteAck {
            header: Header::new(
                Kind::DeleteQueueAck,
                self.header.version(),
                self.header.uuid(),
            ),
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
        assert_eq!(header.kind(), Kind::DeleteQueue);

        let path = String::decode(reader)?;

        Ok(Self { header, path })
    }
}

impl<W> Encode<W> for Delete
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

    use super::Delete;

    #[test]
    fn test_new() {
        let delete = Delete::new((1, 2), "test".to_string());

        assert_eq!(delete.header().version(), 1);
        assert_eq!(delete.header().uuid(), 2);
        assert_eq!(delete.path(), "test");
    }

    #[test]
    fn test_ack() {
        let delete = Delete::new((1, 2), "test".to_string());

        let ack = delete.clone().ack();
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = delete.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::DeleteQueue,
            Delete {
                path: "test".to_string(),
            }
        );
    }
}
