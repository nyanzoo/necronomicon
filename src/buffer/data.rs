use std::io::{Read, Write};

use crate::{Decode, Encode, Error};

use super::{Owned, Shared};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct BinaryData<S>
where
    S: Shared,
{
    len: usize,
    data: S,
}

impl<S> BinaryData<S>
where
    S: Shared,
{
    pub fn new(len: usize, data: S) -> Self {
        Self { len, data }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn data(&self) -> &S {
        &self.data
    }
}

impl<R, O> Decode<R, O> for BinaryData<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let len = usize::decode(reader, buffer)?;
        if buffer.unfilled_capacity() < len {
            return Err(Error::OwnedRemaining {
                acquire: len,
                capacity: buffer.unfilled_capacity(),
            });
        }
        let read = {
            let buffer = buffer.unfilled();
            reader.read(&mut buffer[..len]).map_err(Error::Io)?
        };
        if read != len {
            return Err(Error::BinaryDataSizeMismatch {
                expected: len,
                read,
            });
        }
        buffer.fill(len);
        let data = buffer.split_at(len);
        let data = data.into_shared();

        Ok(Self { len, data })
    }
}

impl<W, S> Encode<W> for BinaryData<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.len.encode(writer)?;
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
