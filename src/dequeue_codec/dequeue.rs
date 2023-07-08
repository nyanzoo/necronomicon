use crate::{Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Dequeue {
    header: Header,
    path: String,
}

impl PartialDecode for Dequeue {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Dequeue);

        let path = String::decode(reader)?;

        Ok(Self { header, path })
    }
}

impl Encode for Dequeue {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Encode, Header, Kind, PartialDecode};

    use super::Dequeue;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Dequeue, 123, 456);
        let mut buf = Vec::new();
        let dequeue = Dequeue {
            header,
            path: "test".to_string(),
        };
        dequeue.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = Dequeue::decode(header, &mut buf).unwrap();
        assert_eq!(dequeue, decoded);
    }
}
