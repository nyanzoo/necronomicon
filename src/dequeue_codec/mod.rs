mod create;
pub use create::Create;

mod create_ack;
pub use create_ack::CreateAck;

mod delete;
pub use delete::Delete;

mod delete_ack;
pub use delete_ack::DeleteAck;

mod dequeue;
pub use dequeue::Dequeue;

mod dequeue_ack;
pub use dequeue_ack::DequeueAck;

mod enqueue;
pub use enqueue::Enqueue;

mod enqueue_ack;
pub use enqueue_ack::EnqueueAck;

mod len;
pub use len::Len;

mod len_ack;
pub use len_ack::LenAck;

mod peek;
pub use peek::Peek;

mod peek_ack;
pub use peek_ack::PeekAck;

pub const START: u8 = 0;

/// A message that can be used to enqueue a new value.
pub const ENQUEUE: u8 = START;
/// An ack for an enqueue message.
pub const ENQUEUE_ACK: u8 = START + 1;
/// A message that can be used to dequeue a value.
pub const DEQUEUE: u8 = START + 2;
/// An ack for a dequeue message.
pub const DEQUEUE_ACK: u8 = START + 3;
/// A message that can be used to peek at the next value.
pub const PEEK: u8 = START + 4;
/// An ack for a peek message.
pub const PEEK_ACK: u8 = START + 5;
/// A message that can be used to get the length of the queue.
pub const LEN: u8 = START + 6;
/// An ack for a len message.
pub const LEN_ACK: u8 = START + 7;
/// A message for creating a new queue.
pub const CREATE: u8 = START + 8;
/// An ack for a create message.
pub const CREATE_ACK: u8 = START + 9;
/// A message for deleting a queue.
pub const DELETE: u8 = START + 10;
/// An ack for a delete message.
pub const DELETE_ACK: u8 = START + 11;

pub const END: u8 = START + 11;

pub fn is_dequeue_message(kind: u8) -> bool {
    (START..=END).contains(&kind)
}

#[cfg(test)]
mod tests {

    use super::{END, START};

    #[test]
    fn is_dequeue_message() {
        assert!(super::is_dequeue_message(START));
        assert!(super::is_dequeue_message(END));
        assert!(!super::is_dequeue_message(END + 1));
    }
}
