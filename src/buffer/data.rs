use std::io::{Read, Write};

use crate::{Decode, DecodeOwned, Encode, Error};

use super::{Owned, Shared};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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

    pub fn from_owned(data: impl AsRef<[u8]>, owned: &mut impl Owned<Shared = S>) -> Result<Self, Error> {
        let len = data.as_ref().len();
        if owned.unfilled_capacity() < len {
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
