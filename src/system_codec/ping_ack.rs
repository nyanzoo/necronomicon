use std::io::{Read, Write};

use crate::{Ack, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PingAck {
    pub(crate) header: Header,
}

impl PingAck {
    pub fn new(header: Header) -> Self {
        assert_eq!(header.kind(), Kind::PingAck);

        Self { header }
    }

    pub fn header(&self) -> Header {
        self.header
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
    use crate::{tests::test_encode_decode_packet, Kind};

    use super::PingAck;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(Kind::PingAck, PingAck {});
    }
}
