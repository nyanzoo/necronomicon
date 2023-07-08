use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct PutAck {
    header: Header,
    response_code: u8,
}

impl PartialDecode for PutAck {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Peek);

        let response_code = u8::decode(reader)?;

        Ok(Self {
            header,
            response_code,
        })
    }
}

impl Encode for PutAck {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;

        Ok(())
    }
}

impl Ack for PutAck {
    fn header(&self) -> &Header {
        &self.header
    }

    fn response_code(&self) -> u8 {
        self.response_code
    }
}

#[cfg(test)]
mod test {
    use crate::{Encode, Header, Kind, PartialDecode};

    use super::PutAck;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::PutAck, 123, 456);
        let mut buf = Vec::new();
        let put_ack = PutAck {
            header,
            response_code: 0,
        };
        put_ack.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = PutAck::decode(header, &mut buf).unwrap();
        assert_eq!(put_ack, decoded);
    }
}
