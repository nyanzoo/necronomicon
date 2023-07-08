use crate::{Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Get {
    header: Header,
    path: String,
    key: String,
}

impl PartialDecode for Get {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Peek);

        let path = String::decode(reader)?;
        let key = String::decode(reader)?;

        Ok(Self { header, path, key })
    }
}

impl Encode for Get {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.key.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Encode, Header, Kind, PartialDecode};

    use super::Get;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Get, 123, 456);
        let mut buf = Vec::new();
        let get = Get {
            header,
            path: "test".to_string(),
            key: "test".to_string(),
        };
        get.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = Get::decode(header, &mut buf).unwrap();
        assert_eq!(get, decoded);
    }
}
