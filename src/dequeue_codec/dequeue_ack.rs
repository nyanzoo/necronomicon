use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct DequeueAck {
    header: Header,
    response_code: u8,
    value: Vec<u8>,
}

impl PartialDecode for DequeueAck {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::DequeueAck);

        let response_code = u8::decode(reader)?;
        let value = Vec::<u8>::decode(reader)?;

        Ok(Self {
            header,
            response_code,
            value,
        })
    }
}

impl Encode for DequeueAck {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

impl Ack for DequeueAck {
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

    use super::DequeueAck;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::DequeueAck, 123, 456);
        let mut buf = Vec::new();
        let dequeue_ack = DequeueAck {
            header,
            response_code: 0,
            value: vec![1, 2, 3],
        };
        dequeue_ack.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = DequeueAck::decode(header, &mut buf).unwrap();
        assert_eq!(dequeue_ack, decoded);
    }
}
