use std::{
    fmt::Debug,
    io::{Read, Write},
};

use log::trace;

use crate::{Decode, DecodeOwned, Encode, Error};

use super::{Owned, Shared};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct BinaryData<S>
where
    S: Shared,
{
    data: S,
}

impl<S> BinaryData<S>
where
    S: Shared,
{
    pub fn new(data: S) -> Self {
        Self { data }
    }

    pub fn from_owned(
        data: impl AsRef<[u8]>,
        owned: &mut impl Owned<Shared = S>,
    ) -> Result<Self, Error> {
        let len = data.as_ref().len();
        if owned.unfilled_capacity() < len {
            trace!("data: {:?}", data.as_ref());
            return Err(Error::OwnedRemaining {
                acquire: len,
                capacity: owned.unfilled_capacity(),
            });
        }
        let buffer = owned.unfilled();
        buffer[..len].copy_from_slice(data.as_ref());
        owned.fill(len);
        let data = owned.split_at(len);
        let data = data.into_shared();

        Ok(Self { data })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn data(&self) -> &S {
        &self.data
    }
}

impl<S> Debug for BinaryData<S>
where
    S: Shared,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryData")
            .field("data", &self.data.as_slice())
            .finish()
    }
}

impl<R, O> DecodeOwned<R, O> for BinaryData<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode_owned(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let len = usize::decode(reader)?;
        if buffer.unfilled_capacity() < len {
            return Err(Error::OwnedRemaining {
                acquire: len,
                capacity: buffer.unfilled_capacity(),
            });
        }

        {
            let buffer = buffer.unfilled();
            reader.read_exact(&mut buffer[..len]).map_err(Error::Io)?;
        }

        buffer.fill(len);
        let data = buffer.split_at(len);
        let data = data.into_shared();

        Ok(Self { data })
    }
}

impl<W, S> Encode<W> for BinaryData<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.len().encode(writer)?;
        writer.write_all(self.data.as_ref()).map_err(Error::Io)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Pool, PoolImpl};

    #[test]
    fn binary_data() {
        let data = vec![1, 2, 3, 4, 5];
        let pool = PoolImpl::new(10, 10);
        let mut buffer = pool.acquire("test");
        let binary_data = BinaryData::from_owned(&data, &mut buffer).expect("from_owned");
        assert_eq!(binary_data.len(), 5);
        assert!(!binary_data.is_empty());
        assert_eq!(binary_data.data().as_slice(), &[1, 2, 3, 4, 5]);

        let mut buffer = pool.acquire("test");
        let binary_data = BinaryData::from_owned([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &mut buffer);
        assert!(binary_data.is_err());
    }
}
