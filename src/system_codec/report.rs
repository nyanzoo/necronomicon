use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{Position, ReportAck};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Report {
    pub(crate) header: Header,
    pub(crate) position: Position,
}

impl Report {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>, position: Position) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::Report),
            position,
        }
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
    use crate::{
        system_codec::Position, tests::test_encode_decode_packet, Ack, Kind, INTERNAL_ERROR,
        SUCCESS,
    };

    use super::Report;

    #[test]
    fn test_new() {
        let report = Report::new(
            (1, 2),
            Position::Head {
                next: "next".to_owned(),
            },
        );

        assert_eq!(report.header().kind(), Kind::Report);
        assert_eq!(report.header().version(), 1);
        assert_eq!(report.header().uuid(), 2);
        assert_eq!(
            report.position(),
            &Position::Head {
                next: "next".to_owned(),
            }
        );
    }

    #[test]
    fn test_ack() {
        let report = Report::new(
            (1, 2),
            Position::Head {
                next: "next".to_owned(),
            },
        );

        let report_ack = report.clone().ack();
        assert_eq!(report_ack.response_code(), SUCCESS);

        let report_nack = report.nack(INTERNAL_ERROR);
        assert_eq!(report_nack.response_code(), INTERNAL_ERROR);
    }

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
