use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{Position, TransferAck};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Transfer {
    pub(crate) header: Header,
    pub(crate) candidate: Position,
}

impl Transfer {
    pub fn new(header: Header, position: Position) -> Self {
        assert_eq!(header.kind(), Kind::Chain);

        Self {
            header,
            candidate: position,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn candidate(&self) -> &Position {
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

        let candidate = Position::decode(reader)?;

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
    use crate::{system_codec::Position, Decode, Encode, Header, Kind, PartialDecode};

    use super::Transfer;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Transfer, 123, 456);
        let mut buf = Vec::new();
        let transfer = Transfer {
            header,
            candidate: Position::Head {
                next: "next".to_owned(),
            },
        };
        transfer.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = Transfer::decode(header, &mut buf).unwrap();
        assert_eq!(transfer, decoded);
    }
}
