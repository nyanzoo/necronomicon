use std::io::{Read, Write};

use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct JoinAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
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
        Kind,
    };

    use super::JoinAck;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(Kind::JoinAck, JoinAck { response_code: 0 });
    }

    #[test]
    fn test_ack() {
        test_ack_packet!(Kind::JoinAck, JoinAck { response_code: 0 });
    }
}
