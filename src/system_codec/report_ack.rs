use std::io::{Read, Write};

use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ReportAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R> PartialDecode<R> for ReportAck
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::ReportAck);

        let response_code = u8::decode(reader)?;

        Ok(Self {
            header,
            response_code,
        })
    }
}

impl<W> Encode<W> for ReportAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;

        Ok(())
    }
}

impl Ack for ReportAck {
    fn header(&self) -> &Header {
        &self.header
    }

    fn response_code(&self) -> u8 {
        self.response_code
    }
}

#[cfg(test)]
mod test {
    use crate::{
        tests::{test_ack_packet, test_encode_decode_packet},
        Kind,
    };

    use super::ReportAck;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(Kind::ReportAck, ReportAck { response_code: 0 });
    }

    #[test]
    fn test_ack() {
        test_ack_packet!(Kind::ReportAck, ReportAck { response_code: 0 });
    }
}
