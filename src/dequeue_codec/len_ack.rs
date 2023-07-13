use std::io::{Read, Write};

use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct LenAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
    pub(crate) len: u64,
}

impl<R> PartialDecode<R> for LenAck
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::LenAck);

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
    use crate::{header::Kind, Decode, Encode, Header, PartialDecode};

    use super::LenAck;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::LenAck, 123, 456);
        let mut buf = Vec::new();
        let len = LenAck {
            header,
            response_code: 0,
            len: 123,
        };
        len.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = LenAck::decode(header, &mut buf).unwrap();
        assert_eq!(len, decoded);
    }
}
