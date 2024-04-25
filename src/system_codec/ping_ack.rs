use std::io::{Read, Write};

use crate::{buffer::Owned, Ack, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PingAck {
    pub(crate) header: Header,
}

impl<R, O> PartialDecode<R, O> for PingAck
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, _reader: &mut R, _: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::PingAck);

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
    use crate::{tests::verify_encode_decode, Header, Kind, Packet};

    use super::PingAck;

    impl PingAck {
        fn new() -> Self {
            Self {
                header: Header::new_test_ack(Kind::PingAck),
            }
        }
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::PingAck(PingAck::new()));
    }
}
