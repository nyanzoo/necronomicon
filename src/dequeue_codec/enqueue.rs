use crate::{header::Header, Decode, Encode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Enqueue {
    header: Header,
    path: String,
    value: Vec<u8>,
}

impl Decode for Enqueue {
    fn decode(reader: &mut impl std::io::Read) -> Result<Self, crate::error::Error>
    where
        Self: Sized,
    {
        let header = Header::decode(reader)?;
        let path = String::decode(reader)?;
        let value = Vec::<u8>::decode(reader)?;

        Ok(Self {
            header,
            path,
            value,
        })
    }
}

impl Encode for Enqueue {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), crate::error::Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Encode, Decode};

    use super::Enqueue;

    #[test]
    fn test_encode_decode() {
        let mut buf = Vec::new();
        let enqueue = Enqueue {
            header: Default::default(),
            path: "test".to_string(),
            value: vec![1, 2, 3],
        };
        enqueue.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = Enqueue::decode(&mut buf).unwrap();
        assert_eq!(enqueue, decoded);
    }
}
