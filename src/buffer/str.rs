use std::{
    fmt::{Debug, Formatter},
    io::{Read, Write},
};

use crate::{DecodeOwned, Encode, Error};

use super::{BinaryData, Owned, Shared};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct ByteStr<S>(BinaryData<S>)
where
    S: Shared;

impl<S> ByteStr<S>
where
    S: Shared,
{
    pub fn new(data: BinaryData<S>) -> Self {
        Self(data)
    }

    pub fn from_owned<O>(data: impl AsRef<str>, owned: &mut O) -> Result<Self, Error>
    where
        O: Owned<Shared = S>,
    {
        let data = BinaryData::from_owned(data.as_ref().as_bytes(), owned)?;
        Ok(Self::new(data))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn inner(&self) -> &BinaryData<S> {
        &self.0
    }

    pub fn data(&self) -> &S {
        self.0.data()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.data().as_slice()
    }

    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.0.data().as_slice())
    }
}

impl<S> Debug for ByteStr<S>
where
    S: Shared,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str().expect("str"))
    }
}

impl<R, O> DecodeOwned<R, O> for ByteStr<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode_owned(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let data = BinaryData::decode_owned(reader, buffer)?;

        Ok(Self(data))
    }
}

impl<W, S> Encode<W> for ByteStr<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.0.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Pool, PoolImpl};

    use super::*;

    #[test]
    fn byte_str() {
        let pool = PoolImpl::new(1024, 1024);
        let mut buffer = pool.acquire("test");
        let data = "hello world";
        let byte_str = ByteStr::from_owned(data, &mut buffer).expect("byte_str");

        assert_eq!(byte_str.len(), data.len());
        assert_eq!(byte_str.is_empty(), false);
        assert_eq!(byte_str.as_str().expect("str"), data);
        assert_eq!(byte_str.data().as_slice(), b"hello world");
    }
}
