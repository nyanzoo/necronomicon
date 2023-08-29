mod delete_ack;
use std::{io::Read, io::Write};

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

use crate::{Decode, Encode};

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

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for Key {
    fn from(key: [u8; 32]) -> Self {
        Self::new(key)
    }
}

impl From<Key> for [u8; 32] {
    fn from(key: Key) -> Self {
        key.0
    }
}

impl From<&[u8]> for Key {
    fn from(key: &[u8]) -> Self {
        let mut bytes = [0; 32];
        bytes.copy_from_slice(key);
        Self::new(bytes)
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
