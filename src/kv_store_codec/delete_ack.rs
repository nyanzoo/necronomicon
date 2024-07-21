use std::io::{Read, Write};

use crate::{buffer::Owned, Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DeleteAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R, O> PartialDecode<R, O> for DeleteAck
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, _: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::DeleteAck);

        let response_code = u8::decode(reader)?;

        Ok(Self {
            header,
            response_code,
        })
    }
}

impl<W> Encode<W> for DeleteAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;

        Ok(())
    }
}

impl Ack for DeleteAck {
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

    use super::DeleteAck;

    impl DeleteAck {
        pub fn new(response_code: u8) -> Self {
            Self {
                header: Header::new(Kind::DeleteAck, 1, 1, 0),
                response_code,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::DeleteAck(DeleteAck::new(SUCCESS)));
    }
}
