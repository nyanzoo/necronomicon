use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct EnqueueAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R> PartialDecode<R> for EnqueueAck
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::EnqueueAck);

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
    use crate::{Decode, Encode, Header, Kind, PartialDecode};

    use super::EnqueueAck;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::EnqueueAck, 123, 456);
        let mut buf = Vec::new();
        let enqueue_ack = EnqueueAck {
            header,
            response_code: 0,
        };
        enqueue_ack.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = EnqueueAck::decode(header, &mut buf).unwrap();
        assert_eq!(enqueue_ack, decoded);
    }
}
