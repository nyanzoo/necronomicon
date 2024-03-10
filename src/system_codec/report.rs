use std::io::{Read, Write};

use crate::{
    buffer::{Owned, Shared},
    header::{Uuid, Version},
    Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS,
};

use super::{Position, ReportAck};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Report<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) position: Position<S>,
}

impl<S> Report<S>
where
    S: Shared,
{
    pub fn new(version: impl Into<Version>, uuid: impl Into<Uuid>, position: Position<S>) -> Self {
        Self {
            header: Header::new(Kind::Report, version, uuid, position.len()),
            position,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn position(&self) -> &Position<S> {
        &self.position
    }

    pub fn ack(self) -> ReportAck {
        ReportAck {
            header: Header::new(Kind::ReportAck, self.header.version, self.header.uuid, 0),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> ReportAck {
        ReportAck {
            header: Header::new(Kind::ReportAck, self.header.version, self.header.uuid, 0),
            response_code,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Report<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Report);

        let position = Position::decode(reader, buffer)?;

        Ok(Self { header, position })
    }
}

impl<W, S> Encode<W> for Report<S>
where
    W: Write,
    S: Shared,
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
        buffer::byte_str, system_codec::Position, tests::verify_encode_decode, Ack, Packet,
        INTERNAL_ERROR, SUCCESS,
    };

    use super::Report;

    #[test]
    fn test_ack() {
        let report = Report::new(
            1,
            2,
            Position::Head {
                next: byte_str(b"next"),
            },
        );

        let report_ack = report.clone().ack();
        assert_eq!(report_ack.response_code(), SUCCESS);

        let report_nack = report.nack(INTERNAL_ERROR);
        assert_eq!(report_nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::Report(Report::new(
            1,
            2,
            Position::Head {
                next: byte_str(b"next"),
            },
        )));
    }
}
