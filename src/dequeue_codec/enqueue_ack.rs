use crate::{Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct EnqueueAck {
    header: Header,
    response_code: u8,
}

impl PartialDecode for EnqueueAck {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
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

impl Encode for EnqueueAck {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
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
    use crate::{Encode, Header, Kind, PartialDecode};

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
        let decoded = EnqueueAck::decode(header, &mut buf).unwrap();
        assert_eq!(enqueue_ack, decoded);
    }
}
