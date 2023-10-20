use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Ack, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PingAck {
    pub(crate) header: Header,
}

impl PingAck {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::PingAck),
        }
    }
}

impl<R> PartialDecode<R> for PingAck
where
    R: Read,
{
    fn decode(header: Header, _reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::PingAck);

        Ok(Self { header })
    }
}

impl<W> Encode<W> for PingAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;

        Ok(())
    }
}

impl Ack for PingAck {
    fn header(&self) -> &Header {
        &self.header
    }

    fn response_code(&self) -> u8 {
        SUCCESS
    }
}

#[cfg(test)]
mod test {
    use crate::{
        tests::{test_ack_packet, test_encode_decode_packet},
        Ack, Kind,
    };

    use super::PingAck;

    #[test]
    fn test_new() {
        let ping_ack = PingAck::new((1, 2));

        assert_eq!(ping_ack.header().kind(), Kind::PingAck);
        assert_eq!(ping_ack.header().version(), 1);
        assert_eq!(ping_ack.header().uuid(), 2);
    }

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(Kind::PingAck, PingAck {});
    }

    #[test]
    fn test_ack() {
        test_ack_packet!(Kind::PingAck, PingAck {});
    }
}
