use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::TransferAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Transfer {
    pub(crate) header: Header,
    pub(crate) path: String,
    pub(crate) content: Vec<u8>,
}

impl Transfer {
    pub fn new(header: Header, path: String, content: Vec<u8>) -> Self {
        assert_eq!(header.kind(), Kind::Transfer);

        Self { header, path, content }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn ack(self) -> TransferAck {
        TransferAck {
            header: Header::new(Kind::TransferAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> TransferAck {
        TransferAck {
            header: Header::new(Kind::TransferAck, self.header.version(), self.header.uuid()),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Transfer
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Transfer);

        let path = String::decode(reader)?;
        let content = Vec::<u8>::decode(reader)?;

        Ok(Self { header, path, content })
    }
}

impl<W> Encode<W> for Transfer
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.content.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{tests::test_encode_decode_packet, Kind};

    use super::Transfer;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Transfer,
            Transfer {
                path: "/tmp/kitty".to_owned(),
                content: vec![0x01, 0x02, 0x03, 0x04],
            }
        );
    }
}
