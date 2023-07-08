use crate::{Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Put {
    header: Header,
    path: String,
    key: String,
    value: Vec<u8>,
}

impl PartialDecode for Put {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Peek);

        let path = String::decode(reader)?;
        let key = String::decode(reader)?;
        let value = Vec::decode(reader)?;

        Ok(Self {
            header,
            path,
            key,
            value,
        })
    }
}

impl Encode for Put {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.key.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Encode, Header, Kind, PartialDecode};

    use super::Put;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Put, 123, 456);
        let mut buf = Vec::new();
        let put = Put {
            header,
            path: "test".to_string(),
            key: "test".to_string(),
            value: vec![1, 2, 3],
        };
        put.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = Put::decode(header, &mut buf).unwrap();
        assert_eq!(put, decoded);
    }
}
