use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::TransferAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Transfer {
    pub(crate) header: Header,
    pub(crate) candidate: String,
}

impl Transfer {
    pub fn new(header: Header, candidate: String) -> Self {
        assert_eq!(header.kind(), Kind::Report);

        Self { header, candidate }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn candidate(&self) -> &str {
        &self.candidate
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

        let candidate = String::decode(reader)?;

        Ok(Self { header, candidate })
    }
}

impl<W> Encode<W> for Transfer
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.candidate.encode(writer)?;

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
                candidate: "candidate".to_owned(),
            }
        );
    }
}
