use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::CreateAck;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Create {
    pub(crate) header: Header,
    pub(crate) path: String,
}

impl Create {
    pub fn new(header: Header, path: String) -> Self {
        assert_eq!(header.kind(), Kind::CreateQueue);

        Self { header, path }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn ack(self, value: Vec<u8>) -> CreateAck {
        CreateAck {
            header: Header::new(
                Kind::CreateQueueAck,
                self.header.version(),
                self.header.uuid(),
            ),
            response_code: SUCCESS,
            value,
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
            value: Vec::new(),
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

        Ok(Self { header, path })
    }
}

impl<W> Encode<W> for Create
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

    use super::Create;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::CreateQueue,
            Create {
                path: "test".to_string(),
            }
        );
    }
}
