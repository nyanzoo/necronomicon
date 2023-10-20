mod report;
use std::io::{Read, Write};

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

use crate::{Decode, Encode};

pub const START: u8 = 0x70;

/// A message to inform a node of position in the chain.
pub const CHAIN: u8 = START;
/// An ack for a chain message.
pub const CHAIN_ACK: u8 = START + 1;
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
pub enum Role {
    Backend(String),  // 1
    Frontend(String), // 2
    Observer,         // 3
}

impl<W> Encode<W> for Role
where
    W: Write,
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

impl<R> Decode<R> for Role
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        let kind = u8::decode(reader)?;
        match kind {
            1 => Ok(Role::Backend(String::decode(reader)?)),
            2 => Ok(Role::Frontend(String::decode(reader)?)),
            _ => Err(crate::Error::SystemBadRole(kind)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position {
    // Backends
    Head {
        next: String,
    }, // 1
    Middle {
        next: String,
    }, // 2
    Tail {
        candidate: Option<String>,
    }, // 3
    Candidate, // 4

    // Frontends
    Frontend {
        head: Option<String>,
        tail: Option<String>,
    }, // 5

    // Observer
    Observer {
        chain: Vec<Role>, // head -> tail
    }, // 6
}

impl<W> Encode<W> for Position
where
    W: Write,
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

impl<R> Decode<R> for Position
where
    R: Read,
{
    fn decode(reader: &mut R) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        let kind = u8::decode(reader)?;
        match kind {
            // Backends
            1 => {
                let next = String::decode(reader)?;
                Ok(Position::Head { next })
            }
            2 => {
                let next = String::decode(reader)?;
                Ok(Position::Middle { next })
            }
            3 => {
                let candidate = Option::<String>::decode(reader)?;
                Ok(Position::Tail { candidate })
            }
            4 => Ok(Position::Candidate),

            // Frontends
            5 => {
                let head = Option::<String>::decode(reader)?;
                let tail = Option::<String>::decode(reader)?;
                Ok(Position::Frontend { head, tail })
            }

            // Unknown
            _ => Err(crate::Error::SystemBadPosition(kind)),
        }
    }
}
