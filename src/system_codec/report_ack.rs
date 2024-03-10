use std::io::{Read, Write};

use crate::{buffer::Owned, Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ReportAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R, O> PartialDecode<R, O> for ReportAck
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::ReportAck);

        let response_code = u8::decode(reader, buffer)?;

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
    use crate::{tests::verify_encode_decode, Header, Kind, Packet, SUCCESS};

    use super::ReportAck;

    impl ReportAck {
        pub fn new(response_code: u8) -> Self {
            Self {
                header: Header::new_test_ack(Kind::ReportAck),
                response_code,
            }
        }
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::ReportAck(ReportAck::new(SUCCESS)));
    }
}
