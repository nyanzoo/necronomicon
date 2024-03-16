use std::io::{Read, Write};

use crate::{buffer::Owned, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EnqueueAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R, O> PartialDecode<R, O> for EnqueueAck
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, _: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::EnqueueAck);

        let response_code = u8::decode(reader)?;

        Ok(Self {
            header,
            response_code,
        })
    }
}

impl<W> Encode<W> for EnqueueAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;

        Ok(())
    }
}

impl crate::Ack for EnqueueAck {
    fn header(&self) -> &Header {
        &self.header
    }

    fn response_code(&self) -> u8 {
        self.response_code
    }
}

#[cfg(test)]
mod tests {
    use crate::{tests::verify_encode_decode, Header, Kind, Packet, SUCCESS};

    use super::EnqueueAck;

    impl EnqueueAck {
        pub fn new(response_code: u8) -> Self {
            Self {
                header: Header::new(Kind::EnqueueAck, 1, 1, 0),
                response_code,
            }
        }
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::EnqueueAck(EnqueueAck::new(SUCCESS)));
    }
}
