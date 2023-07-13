use std::io::{Read, Write};

use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct DeleteAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R> PartialDecode<R> for DeleteAck
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::DeleteAck);

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
    use crate::{Decode, Encode, Header, Kind, PartialDecode};

    use super::DeleteAck;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::DeleteAck, 123, 456);
        let mut buf = Vec::new();
        let delete_ack = DeleteAck {
            header,
            response_code: 0,
        };
        delete_ack.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = DeleteAck::decode(header, &mut buf).unwrap();
        assert_eq!(delete_ack, decoded);
    }
}
