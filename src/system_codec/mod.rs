mod chain;
use std::io::{Read, Write};

pub use chain::Chain;

mod chain_ack;
pub use chain_ack::ChainAck;

mod join;
pub use join::Join;

mod join_ack;
pub use join_ack::JoinAck;

mod transfer;
pub use transfer::Transfer;

mod transfer_ack;
pub use transfer_ack::TransferAck;

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

pub const END: u8 = START + 5;

pub fn is_system_message(kind: u8) -> bool {
    (START..=END).contains(&kind)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position {
    Head { next: String },           // 1
    Middle { next: String },         // 2
    Tail { frontend: String },       // 3
    Candidate { candidate: String }, // 4
}

impl<W> Encode<W> for Position
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), crate::Error> {
        match self {
            Position::Head { next } => {
                1u8.encode(writer)?;
                next.encode(writer)?;
            }
            Position::Middle { next } => {
                2u8.encode(writer)?;
                next.encode(writer)?;
            }
            Position::Tail { frontend } => {
                3u8.encode(writer)?;
                frontend.encode(writer)?;
            }
            Position::Candidate { candidate } => {
                4u8.encode(writer)?;
                candidate.encode(writer)?;
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
            1 => {
                let next = String::decode(reader)?;
                Ok(Position::Head { next })
            }
            2 => {
                let next = String::decode(reader)?;
                Ok(Position::Middle { next })
            }
            3 => {
                let frontend = String::decode(reader)?;
                Ok(Position::Tail { frontend })
            }
            4 => {
                let candidate = String::decode(reader)?;
                Ok(Position::Candidate { candidate })
            }
            _ => Err(crate::Error::SystemBadPosition(kind)),
        }
    }
}
