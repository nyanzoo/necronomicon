use crate::{header::Header, Decode, Encode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Dequeue {
    header: Header,
    path: String,
}

impl Decode for Dequeue {
    fn decode(reader: &mut impl std::io::Read) -> Result<Self, crate::error::Error>
    where
        Self: Sized,
    {
        let header = Header::decode(reader)?;
        let path = String::decode(reader)?;

        Ok(Self { header, path })
    }
}

impl Encode for Dequeue {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), crate::error::Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Decode, Encode};

    use super::Dequeue;

    #[test]
    fn test_encode_decode() {
        let mut buf = Vec::new();
        let dequeue = Dequeue {
            header: Default::default(),
            path: "test".to_string(),
        };
        dequeue.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = Dequeue::decode(&mut buf).unwrap();
        assert_eq!(dequeue, decoded);
    }
}
