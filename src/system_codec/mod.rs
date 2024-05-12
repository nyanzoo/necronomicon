mod report;
use std::{
    io::{Read, Write},
    mem::size_of,
};

pub use report::Report;

mod report_ack;
pub use report_ack::ReportAck;

mod join;
pub use join::Join;

mod join_ack;
pub use join_ack::JoinAck;

mod transfer;
pub use transfer::Transfer;

mod transfer_ack;
pub use transfer_ack::TransferAck;

mod ping;
pub use ping::Ping;

mod ping_ack;
pub use ping_ack::PingAck;

use crate::{
    buffer::{ByteStr, Owned, Shared},
    Decode, DecodeOwned, Encode,
};

pub const START: u8 = 0x70;

/// A message to inform a node of position in the chain.
pub const REPORT: u8 = START;
/// An ack for a chain message.
pub const REPORT_ACK: u8 = START + 1;
/// A message to indicate to operator that the transfer is complete (failure recovery).
pub const JOIN: u8 = START + 2;
/// An ack for a join message.
pub const JOIN_ACK: u8 = START + 3;
/// A message to transfer a node to a new node (failure recovery).
pub const TRANSFER: u8 = START + 4;
/// An ack for a transfer message.
pub const TRANSFER_ACK: u8 = START + 5;
/// For checking liveness
pub const PING: u8 = START + 6;
/// Ack for a ping
pub const PING_ACK: u8 = START + 7;

pub const END: u8 = START + 7;

pub fn is_system_message(kind: u8) -> bool {
    (START..=END).contains(&kind)
}

// Role + Identifier
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role<S>
where
    S: Shared,
{
    Backend(ByteStr<S>),  // 1
    Frontend(ByteStr<S>), // 2
    Observer,             // 3
}

impl<S> Role<S>
where
    S: Shared,
{
    pub fn encode_len(&self) -> usize {
        match self {
            Role::Backend(addr) | Role::Frontend(addr) => addr.len(),
            Role::Observer => 0,
        }
    }
}

impl<W, S> Encode<W> for Role<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), crate::Error> {
        match self {
            Role::Backend(addr) => {
                1u8.encode(writer)?;
                addr.encode(writer)?;
            }
            Role::Frontend(addr) => {
                2u8.encode(writer)?;
                addr.encode(writer)?;
            }
            Role::Observer => {
                3u8.encode(writer)?;
            }
        }

        Ok(())
    }
}

impl<R, O> DecodeOwned<R, O> for Role<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode_owned(reader: &mut R, buffer: &mut O) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        let kind = u8::decode(reader)?;
        match kind {
            1 => Ok(Role::Backend(ByteStr::decode_owned(reader, buffer)?)),
            2 => Ok(Role::Frontend(ByteStr::decode_owned(reader, buffer)?)),
            3 => Ok(Role::Observer),
            _ => Err(crate::Error::SystemBadRole(kind)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position<S>
where
    S: Shared,
{
    // Backends
    Head {
        next: ByteStr<S>,
    }, // 1
    Middle {
        next: ByteStr<S>,
    }, // 2
    Tail {
        candidate: Option<ByteStr<S>>,
    }, // 3
    Candidate, // 4

    // Frontends
    Frontend {
        head: Option<ByteStr<S>>,
        tail: Option<ByteStr<S>>,
    }, // 5

    // Observer
    Observer {
        chain: Vec<Role<S>>, // head -> tail
    }, // 6
}

impl<S> Position<S>
where
    S: Shared,
{
    pub fn encode_len(&self) -> usize {
        match self {
            Position::Head { next } | Position::Middle { next } => next.len(),
            Position::Tail { candidate } => candidate.as_ref().map(|c| c.len()).unwrap_or(0),
            Position::Candidate => 0,
            Position::Frontend { head, tail } => {
                head.as_ref().map(|h| h.len()).unwrap_or(0)
                    + tail.as_ref().map(|t| t.len()).unwrap_or(0)
            }
            Position::Observer { chain } => {
                chain.iter().map(|role| role.encode_len()).sum::<usize>() + size_of::<u64>()
            }
        }
    }
}

impl<W, S> Encode<W> for Position<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), crate::Error> {
        match self {
            // Backends
            Position::Head { next } => {
                1u8.encode(writer)?;
                next.encode(writer)?;
            }
            Position::Middle { next } => {
                2u8.encode(writer)?;
                next.encode(writer)?;
            }
            Position::Tail { candidate } => {
                3u8.encode(writer)?;
                candidate.encode(writer)?;
            }
            Position::Candidate => {
                4u8.encode(writer)?;
            }

            // Frontends
            Position::Frontend { head, tail } => {
                5u8.encode(writer)?;
                head.encode(writer)?;
                tail.encode(writer)?;
            }

            // Observer
            Position::Observer { chain } => {
                6u8.encode(writer)?;
                chain.encode(writer)?;
            }
        }

        Ok(())
    }
}

impl<R, O> DecodeOwned<R, O> for Position<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode_owned(reader: &mut R, buffer: &mut O) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        let kind = u8::decode(reader)?;
        match kind {
            // Backends
            1 => {
                let next = ByteStr::decode_owned(reader, buffer)?;
                Ok(Position::Head { next })
            }
            2 => {
                let next = ByteStr::decode_owned(reader, buffer)?;
                Ok(Position::Middle { next })
            }
            3 => {
                let candidate = Option::decode_owned(reader, buffer)?;
                Ok(Position::Tail { candidate })
            }
            4 => Ok(Position::Candidate),

            // Frontends
            5 => {
                let head = Option::decode_owned(reader, buffer)?;
                let tail = Option::decode_owned(reader, buffer)?;
                Ok(Position::Frontend { head, tail })
            }

            // Observer
            6 => {
                let chain = Vec::decode_owned(reader, buffer)?;
                Ok(Position::Observer { chain })
            }

            // Unknown
            _ => Err(crate::Error::SystemBadPosition(kind)),
        }
    }
}

#[cfg(test)]
mod test {
    use matches::assert_matches;

    use crate::{
        buffer::{byte_str, Pool, PoolImpl},
        DecodeOwned, Encode,
    };

    use super::{Position, Role, END, START};

    #[test]
    fn is_system_message() {
        assert!(!super::is_system_message(0));
        assert!(super::is_system_message(START));
        assert!(super::is_system_message(END));
        assert!(!super::is_system_message(END + 1));
    }

    #[test]
    fn encode_decode_role() {
        for role in &[
            Role::Backend(byte_str(b"backend")),
            Role::Frontend(byte_str(b"frontend")),
            Role::Observer,
        ] {
            let mut bytes = Vec::new();
            role.encode(&mut bytes).unwrap();

            let pool = PoolImpl::new(1024, 1);
            let mut buffer = pool.acquire("decode role").unwrap();

            let decoded = Role::decode_owned(&mut bytes.as_slice(), &mut buffer).unwrap();
            assert_eq!(*role, decoded);
        }

        let pool = PoolImpl::new(1024, 1);
        let mut buffer = pool.acquire("decode role").unwrap();

        assert_matches!(
            Role::decode_owned(&mut [0u8].as_ref(), &mut buffer),
            Err(crate::Error::SystemBadRole(0))
        );
    }

    #[test]
    fn encode_decode_position() {
        for position in &[
            Position::Head {
                next: byte_str(b"next"),
            },
            Position::Middle {
                next: byte_str(b"next"),
            },
            Position::Tail {
                candidate: Some(byte_str(b"candidate")),
            },
            Position::Candidate,
            Position::Frontend {
                head: Some(byte_str(b"head")),
                tail: Some(byte_str(b"tail")),
            },
            Position::Observer {
                chain: vec![
                    Role::Backend(byte_str(b"backend")),
                    Role::Frontend(byte_str(b"frontend")),
                    Role::Observer,
                ],
            },
        ] {
            let mut bytes = Vec::new();
            position.encode(&mut bytes).unwrap();

            let pool = PoolImpl::new(1024, 1);
            let mut buffer = pool.acquire("decode position").unwrap();

            let decoded = Position::decode_owned(&mut bytes.as_slice(), &mut buffer).unwrap();
            assert_eq!(*position, decoded);
        }

        let pool = PoolImpl::new(1024, 1);
        let mut buffer = pool.acquire("decode position").unwrap();

        assert_matches!(
            Position::decode_owned(&mut [0u8].as_ref(), &mut buffer),
            Err(crate::Error::SystemBadPosition(0))
        );
    }
}
