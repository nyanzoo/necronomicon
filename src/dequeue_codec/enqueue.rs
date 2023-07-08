use crate::{Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Enqueue {
    header: Header,
    path: String,
    value: Vec<u8>,
}

impl PartialDecode for Enqueue {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Enqueue);

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
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{header::Kind, Encode, Header, PartialDecode};

    use super::Enqueue;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Enqueue, 123, 456);
        let mut buf = Vec::new();
        let enqueue = Enqueue {
            header,
            path: "test".to_string(),
            value: vec![1, 2, 3],
        };
        enqueue.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = Enqueue::decode(header, &mut buf).unwrap();
        assert_eq!(enqueue, decoded);
    }
}
