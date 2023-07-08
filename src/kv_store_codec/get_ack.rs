use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct GetAck {
    header: Header,
    response_code: u8,
    value: Vec<u8>,
}

impl PartialDecode for GetAck {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Peek);

        let response_code = u8::decode(reader)?;
        let value = Vec::decode(reader)?;

        Ok(Self {
            header,
            response_code,
            value,
        })
    }
}

impl Encode for GetAck {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

impl Ack for GetAck {
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

    use super::GetAck;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::GetAck, 123, 456);
        let mut buf = Vec::new();
        let get_ack = GetAck {
            header,
            response_code: 0,
            value: vec![1, 2, 3],
        };
        get_ack.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = GetAck::decode(header, &mut buf).unwrap();
        assert_eq!(get_ack, decoded);
    }
}
