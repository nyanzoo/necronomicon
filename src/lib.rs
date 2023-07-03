use std::io::{Read, Write};

pub mod dequeue_codec;

pub mod error;
use error::Error;

mod header;
pub use header::Header;

pub mod kv_store_codec;

pub trait Ack {
    fn header(&self) -> &Header;

    fn response_code(&self) -> u8;
}

pub trait Decode {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait Encode {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error>;
}

impl Decode for String {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut len = [0; 2];
        reader.read_exact(&mut len).map_err(Error::Decode)?;
        let len = u16::from_be_bytes(len);
        let mut bytes = vec![0; len as usize];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        String::from_utf8(bytes).map_err(Error::DecodeString)
    }
}

impl Encode for String {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        let bytes = self.as_bytes();
        let len = bytes.len() as u16;
        writer
            .write_all(&len.to_be_bytes())
            .map_err(Error::Encode)?;
        writer.write_all(bytes).map_err(Error::Encode)?;
        Ok(())
    }
}

impl Decode for Vec<u8> {
    fn decode(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut len = [0; 2];
        reader.read_exact(&mut len).map_err(Error::Decode)?;
        let len = u16::from_be_bytes(len);
        let mut bytes = vec![0; len as usize];
        reader.read_exact(&mut bytes).map_err(Error::Decode)?;
        Ok(bytes)
    }
}

impl Encode for Vec<u8> {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        let len = self.len() as u16;
        writer
            .write_all(&len.to_be_bytes())
            .map_err(Error::Encode)?;
        writer.write_all(self).map_err(Error::Encode)?;
        Ok(())
    }
}
