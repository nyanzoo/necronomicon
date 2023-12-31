use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct JoinAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl JoinAck {
    pub fn new(version_and_uuid: impl Into<VersionAndUuid>, response_code: u8) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::JoinAck),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for JoinAck
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::JoinAck);

        let response_code = u8::decode(reader)?;

        Ok(Self {
            header,
            response_code,
        })
    }
}

impl<W> Encode<W> for JoinAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;

        Ok(())
    }
}

impl Ack for JoinAck {
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
        Ack, Kind, SUCCESS,
    };

    use super::JoinAck;

    #[test]
    fn test_new() {
        let join_ack = JoinAck::new((0, 0), 0);

        assert_eq!(join_ack.header().kind(), Kind::JoinAck);
        assert_eq!(join_ack.header().version(), 0);
        assert_eq!(join_ack.header().uuid(), 0);
        assert_eq!(join_ack.response_code(), SUCCESS);
    }

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::JoinAck,
            JoinAck {
                response_code: SUCCESS
            }
        );
    }

    #[test]
    fn test_ack() {
        test_ack_packet!(
            Kind::JoinAck,
            JoinAck {
                response_code: SUCCESS
            }
        );
    }
}
