use std::io::{Read, Write};

use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct ChainAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R> PartialDecode<R> for ChainAck
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::ChainAck);

        let response_code = u8::decode(reader)?;

        Ok(Self {
            header,
            response_code,
        })
    }
}

impl<W> Encode<W> for ChainAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;

        Ok(())
    }
}

impl Ack for ChainAck {
    fn header(&self) -> &Header {
        &self.header
    }

    fn response_code(&self) -> u8 {
        self.response_code
    }
}

#[cfg(test)]
mod test {
    use crate::{Decode, Encode, Header, Kind, PartialDecode};

    use super::ChainAck;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::ChainAck, 123, 456);
        let mut buf = Vec::new();
        let chain_ack = ChainAck {
            header,
            response_code: 0,
        };
        chain_ack.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = ChainAck::decode(header, &mut buf).unwrap();
        assert_eq!(chain_ack, decoded);
    }
}
