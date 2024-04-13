use std::{
    fmt::Debug,
    io::{Read, Write},
};

use crate::{error::Error, Decode, Encode, Kind};

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Version(u8);

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Version> for u8 {
    fn from(value: Version) -> Self {
        value.0
    }
}

impl<R> Decode<R> for Version
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error> {
        u8::decode(reader).map(Version::from)
    }
}

impl<W> Encode<W> for Version
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.0.encode(writer)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Uuid(u128);

impl From<u128> for Uuid {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl<R> Decode<R> for Uuid
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error> {
        u128::decode(reader).map(Uuid::from)
    }
}

impl<W> Encode<W> for Uuid
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.0.encode(writer)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    pub kind: Kind,
    pub version: Version,
    pub len: usize,
    pub uuid: Uuid,
}

impl Header {
    pub fn new(
        kind: impl Into<Kind>,
        version: impl Into<Version>,
        uuid: impl Into<Uuid>,
        len: usize,
    ) -> Self {
        Self {
            kind: kind.into(),
            version: version.into(),
            uuid: uuid.into(),
            len,
        }
    }

    #[cfg(test)]
    pub fn new_test(kind: impl Into<Kind>, len: usize) -> Self {
        Self {
            kind: kind.into(),
            version: 1.into(),
            uuid: 1.into(),
            len,
        }
    }

    #[cfg(any(test, feature = "test"))]
    pub fn new_test_ack(kind: impl Into<Kind>) -> Self {
        Self::new_test(kind, 0)
    }
}

impl<R> Decode<R> for Header
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, Error> {
        let kind = Kind::decode(reader)?;
        let version = Version::decode(reader)?;
        let len = usize::decode(reader)?;
        let uuid = Uuid::decode(reader)?;

        Ok(Header {
            kind,
            version,
            len,
            uuid,
        })
    }
}

impl<W> Encode<W> for Header
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.kind.encode(writer)?;
        self.version.encode(writer)?;
        self.len.encode(writer)?;
        self.uuid.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use test_case::test_case;

    use crate::{Decode, Encode, Kind};

    use super::Header;

    #[cfg_attr(nightly, no_coverage)]
    #[test_case(0, 0, 0; "zero")]
    #[test_case(1, 1, 1; "one")]
    #[test_case(2, 2, 2; "two")]
    fn test_header_encode_decode(kind: u8, version: u8, uuid: u128) {
        let mut buf = Vec::new();
        let header = Header::new(kind, version, uuid, 0);
        header.encode(&mut buf).expect("encode");

        let mut reader = Cursor::new(buf);
        let actual = Header::decode(&mut reader).expect("decode");
        assert_eq!(Kind::from(kind), header.kind);
        assert_eq!(header.version, version.into());
        assert_eq!(header.uuid, uuid.into());
        assert_eq!(actual, header);
    }
}
