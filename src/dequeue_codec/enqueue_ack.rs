use crate::{header::Header, Decode, Encode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct EnqueueAck {
    header: Header,
    response_code: u8,
}

impl Decode for EnqueueAck {
    fn decode(reader: &mut impl std::io::Read) -> Result<Self, crate::error::Error>
    where
        Self: Sized,
    {
        let header = Header::decode(reader)?;
        let mut buf = [0; 1];
        reader
            .read_exact(&mut buf)
            .map_err(crate::error::Error::Decode)?;
        let response_code = buf[0];
        Ok(Self {
            header,
            response_code,
        })
    }
}

impl Encode for EnqueueAck {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), crate::error::Error> {
        self.header.encode(writer)?;
        writer
            .write_all(&[self.response_code])
            .map_err(crate::error::Error::Encode)?;
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
    use crate::{Decode, Encode};

    use super::EnqueueAck;

    #[test]
    fn test_encode_decode() {
        let mut buf = Vec::new();
        let enqueue_ack = EnqueueAck {
            header: Default::default(),
            response_code: 0,
        };
        enqueue_ack.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = EnqueueAck::decode(&mut buf).unwrap();
        assert_eq!(enqueue_ack, decoded);
    }
}
