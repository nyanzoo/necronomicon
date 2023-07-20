use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::LenAck;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Len {
    pub(crate) header: Header,
    pub(crate) path: String,
}

impl Len {
    pub fn new(header: Header, path: String) -> Self {
        assert_eq!(header.kind(), Kind::Len);

        Self { header, path }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn ack(self, len: u64) -> LenAck {
        LenAck {
            header: Header::new(Kind::LenAck, self.header.version(), self.header.uuid()),
            len,
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> LenAck {
        LenAck {
            header: Header::new(Kind::LenAck, self.header.version(), self.header.uuid()),
            len: 0,
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Len
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Len);

        let path = String::decode(reader)?;

        Ok(Self { header, path })
    }
}

impl<W> Encode<W> for Len
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{tests::test_encode_decode_packet, Kind};

    use super::Len;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Len,
            Len {
                path: "test".to_string(),
            }
        );
    }
}
