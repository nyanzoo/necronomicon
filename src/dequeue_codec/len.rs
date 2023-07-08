use crate::{Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Len {
    header: Header,
    path: String,
}

impl PartialDecode for Len {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Len);

        let path = String::decode(reader)?;

        Ok(Self { header, path })
    }
}

impl Encode for Len {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{header::Kind, Encode, Header, PartialDecode};

    use super::Len;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Len, 123, 456);
        let mut buf = Vec::new();
        let len = Len {
            header,
            path: "test".to_string(),
        };
        len.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = Len::decode(header, &mut buf).unwrap();
        assert_eq!(len, decoded);
    }
}
