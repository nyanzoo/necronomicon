use crate::{Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Peek {
    header: Header,
    path: String,
}

impl PartialDecode for Peek {
    fn decode(header: Header, reader: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Peek);

        let path = String::decode(reader)?;

        Ok(Self { header, path })
    }
}

impl Encode for Peek {
    fn encode(&self, writer: &mut impl std::io::Write) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Encode, Header, Kind, PartialDecode};

    use super::Peek;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Peek, 123, 456);
        let mut buf = Vec::new();
        let peek = Peek {
            header,
            path: "test".to_string(),
        };
        peek.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let decoded = Peek::decode(header, &mut buf).unwrap();
        assert_eq!(peek, decoded);
    }
}
