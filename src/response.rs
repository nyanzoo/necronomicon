use std::io::{Read, Write};

use crate::{ByteStr, Decode, DecodeOwned, Encode, Error, Owned, Shared};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Response<S>
where
    S: Shared,
{
    pub code: u8,
    pub reason: Option<ByteStr<S>>,
}

impl<S> Response<S>
where
    S: Shared,
{
    pub fn fail(code: u8, reason: Option<ByteStr<S>>) -> Self {
        Self { code, reason }
    }

    pub const fn success() -> Self {
        Self {
            code: 0,
            reason: None,
        }
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn reason(&self) -> &Option<ByteStr<S>> {
        &self.reason
    }
}

impl<R, O> DecodeOwned<R, O> for Response<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode_owned(reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let code = u8::decode(reader)?;
        let value = Option::decode_owned(reader, buffer)?;

        Ok(Self {
            code,
            reason: value,
        })
    }
}

impl<W, S> Encode<W> for Response<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.code.encode(writer)?;
        self.reason.encode(writer)?;

        Ok(())
    }
}
