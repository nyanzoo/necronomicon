use std::io::{Read, Write};

use crate::{Decode, Encode, Error};

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

    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error>
    where
        S: Into<Vec<u8>>,
    {
        std::str::from_utf8(self.0.data().as_slice())
    }
}

impl<R, O> Decode<R, O> for ByteStr<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let data = BinaryData::decode(reader, buffer)?;

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
