use std::io::{Read, Write};

use crate::{buffer::Owned, Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct LenAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
    pub(crate) len: u64,
}

impl<R, O> PartialDecode<R, O> for LenAck
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, _: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::LenAck);

        let response_code = u8::decode(reader)?;
        let len = u64::decode(reader)?;

        Ok(Self {
            header,
            response_code,
            len,
        })
    }
}

impl<W> Encode<W> for LenAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;
        self.len.encode(writer)?;

        Ok(())
    }
}

impl Ack for LenAck {
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

    use super::LenAck;

    impl LenAck {
        pub fn new(response_code: u8, len: u64) -> Self {
            Self {
                header: Header::new(Kind::LenAck, 1, 1, 0),
                response_code,
                len,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::LenAck(LenAck::new(SUCCESS, 123)));
    }
}
