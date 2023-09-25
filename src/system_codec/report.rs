use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{Position, ReportAck};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Report {
    pub(crate) header: Header,
    pub(crate) position: Position,
}

impl Report {
    pub fn new(header: Header, position: Position) -> Self {
        assert_eq!(header.kind(), Kind::Report);

        Self { header, position }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn ack(self) -> ReportAck {
        ReportAck {
            header: Header::new(Kind::ReportAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> ReportAck {
        ReportAck {
            header: Header::new(Kind::ReportAck, self.header.version(), self.header.uuid()),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Report
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Report);

        let position = Position::decode(reader)?;

        Ok(Self { header, position })
    }
}

impl<W> Encode<W> for Report
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.position.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{system_codec::Position, tests::test_encode_decode_packet, Kind};

    use super::Report;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Report,
            Report {
                position: Position::Head {
                    next: "next".to_owned(),
                },
            }
        );
    }
}
