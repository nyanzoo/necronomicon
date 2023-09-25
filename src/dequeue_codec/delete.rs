use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::DeleteAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Delete {
    pub(crate) header: Header,
    pub(crate) path: String,
}

impl Delete {
    pub fn new(header: Header, path: String) -> Self {
        assert_eq!(header.kind(), Kind::DeleteQueue);

        Self { header, path }
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
    use crate::{tests::test_encode_decode_packet, Kind};

    use super::Delete;

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
