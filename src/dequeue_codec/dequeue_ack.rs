use crate::{error::Error, header::Header, Ack, Decode, Encode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct DequeueAck {
    header: Header,
    response_code: u8,
    value: Vec<u8>,
}

impl Decode for DequeueAck {
    fn decode(reader: &mut impl std::io::Read) -> Result<Self, crate::error::Error>
    where
        Self: Sized,
    {
        let header = Header::decode(reader)?;
        let mut buf = [0; 1];
        reader.read_exact(&mut buf).map_err(Error::Decode)?;
        let response_code = buf[0];
        let value = Vec::<u8>::decode(reader)?;
        Ok(Self {
            header,
            response_code,
            value,
        })
    }
}

impl Encode for DequeueAck {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), crate::error::Error> {
        self.header.encode(writer)?;
        writer
            .write_all(&[self.response_code])
            .map_err(Error::Encode)?;
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
    use crate::{Decode, Encode};

    use super::DequeueAck;

    #[test]
    fn test_encode_decode() {
        let mut buf = Vec::new();
        let dequeue_ack = DequeueAck {
            header: Default::default(),
            response_code: 0,
            value: vec![1, 2, 3],
        };
        dequeue_ack.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = DequeueAck::decode(&mut buf).unwrap();
        assert_eq!(dequeue_ack, decoded);
    }
}
