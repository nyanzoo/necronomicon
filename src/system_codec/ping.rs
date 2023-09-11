use std::io::{Read, Write};

use crate::{Encode, Error, Header, Kind, PartialDecode};

use super::PingAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Ping {
    pub(crate) header: Header,
}

impl Ping {
    pub fn new(header: Header) -> Self {
        assert_eq!(header.kind(), Kind::Ping);

        Self { header }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn ack(self) -> PingAck {
        PingAck {
            header: Header::new(Kind::PingAck, self.header.version(), self.header.uuid()),
        }
    }
}

impl<R> PartialDecode<R> for Ping
where
    R: Read,
{
    fn decode(header: Header, _reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Ping);

        Ok(Self { header })
    }
}

impl<W> Encode<W> for Ping
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{tests::test_encode_decode_packet, Kind};

    use super::Ping;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(Kind::Ping, Ping {});
    }
}
