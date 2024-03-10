use std::io::{Read, Write};

use crate::{buffer::Owned, Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct JoinAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R, O> PartialDecode<R, O> for JoinAck
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::JoinAck);

        let response_code = u8::decode(reader, buffer)?;

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
    use crate::{tests::verify_encode_decode, Header, Kind, Packet, SUCCESS};

    use super::JoinAck;

    impl JoinAck {
        pub fn new(response_code: u8) -> Self {
            Self {
                header: Header::new_test_ack(Kind::JoinAck),
                response_code,
            }
        }
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::JoinAck(JoinAck::new(SUCCESS)));
    }
}
