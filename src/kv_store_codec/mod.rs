use std::io::{Read, Write};

mod delete_ack;
pub use delete_ack::DeleteAck;

mod delete;
pub use delete::Delete;

mod get_ack;
pub use get_ack::GetAck;

mod get;
pub use get::Get;

mod put_ack;
pub use put_ack::PutAck;

mod put;
pub use put::Put;

use crate::{Decode, Encode, Error};

pub const START: u8 = 0x10;

/// A message that can be used to insert into the kv store.
pub const PUT: u8 = START;
/// An ack for a put message.
pub const PUT_ACK: u8 = START + 1;
/// A message that can be used to get from the kv store.
pub const GET: u8 = START + 2;
/// An ack for a get message.
pub const GET_ACK: u8 = START + 3;
/// A message that can be used to remove from the kv store.
pub const DELETE: u8 = START + 4;
/// An ack for a delete message.
pub const DELETE_ACK: u8 = START + 5;

pub const END: u8 = START + 5;

pub fn is_kv_store_message(kind: u8) -> bool {
    (START..=END).contains(&kind)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key([u8; 32]);

impl Key {
    pub const fn new(key: [u8; 32]) -> Self {
        Self(key)
    }
}

impl TryFrom<String> for Key {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for Key {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}

impl TryFrom<&[u8]> for Key {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > 32 {
            return Err(Error::InvalidKeyLength(
                String::from_utf8_lossy(value).to_string(),
            ));
        }

        let mut bytes = [0; 32];
        let len = std::cmp::min(bytes.len(), value.len());
        bytes[..len].copy_from_slice(&value[..len]);

        Ok(Self::new(bytes))
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<W> Encode<W> for Key
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), crate::Error> {
        writer.write_all(&self.0)?;

        Ok(())
    }
}

impl<R> Decode<R> for Key
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, crate::Error> {
        let mut bytes = [0; 32];
        reader.read_exact(&mut bytes)?;

        Ok(Self::new(bytes))
    }
}

#[cfg(test)]
pub const TEST_KEY: Key = Key([
    0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0xa1, 0xb2, 0xc3, 0xd4, 0xe5, 0xf6, 0x11, 0x22, 0x33, 0x44,
    0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
]);

#[cfg(test)]
mod tests {

    use super::{is_kv_store_message, Key, END, START};

    #[test]
    fn test_key() {
        let key = Key::try_from("test").unwrap();

        let mut expected = [0u8; 32];
        expected[..4].copy_from_slice(b"test");

        assert_eq!(key.as_ref(), expected);

        let key = Key::try_from("test".to_owned()).unwrap();
        assert_eq!(key.as_ref(), expected);

        let key = Key::try_from(&[1u8; 32][..]).unwrap();
        assert_eq!(key.as_ref(), &[1; 32]);

        let key = Key::try_from(&[1; 33][..]);
        assert!(key.is_err());
    }

    #[test]
    fn test_is_kv_store_message() {
        assert!(!is_kv_store_message(START - 1));
        assert!(is_kv_store_message(START));
        assert!(is_kv_store_message(END));
        assert!(!is_kv_store_message(END + 1));
    }
}
