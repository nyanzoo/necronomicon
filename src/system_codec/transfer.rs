use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::TransferAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Transfer {
    pub(crate) header: Header,
    pub(crate) path: String,
    pub(crate) content: Vec<u8>,
}

impl Transfer {
    pub fn new(
        version_and_uuid: impl Into<VersionAndUuid>,
        path: String,
        content: Vec<u8>,
    ) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::Transfer),
            path,
            content,
        }
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

        Ok(Self {
            header,
            path,
            content,
        })
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
    use crate::{tests::test_encode_decode_packet, Ack, Kind, INTERNAL_ERROR, SUCCESS};

    use super::Transfer;

    #[test]
    fn test_new() {
        let transfer = Transfer::new(
            (1, 2),
            "/tmp/kitty".to_owned(),
            vec![0x01, 0x02, 0x03, 0x04],
        );

        assert_eq!(transfer.header().kind(), Kind::Transfer);
        assert_eq!(transfer.header().version(), 1);
        assert_eq!(transfer.header().uuid(), 2);
        assert_eq!(transfer.path(), "/tmp/kitty");
        assert_eq!(transfer.content(), &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_acks() {
        let transfer = Transfer::new(
            (1, 2),
            "/tmp/kitty".to_owned(),
            vec![0x01, 0x02, 0x03, 0x04],
        );

        let ack = transfer.clone().ack();
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = transfer.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

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
