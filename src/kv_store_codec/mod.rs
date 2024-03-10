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

/// The maximum length of a key.
pub const MAX_KEY_LENGTH: usize = 32;

pub fn is_kv_store_message(kind: u8) -> bool {
    (START..=END).contains(&kind)
}

#[cfg(test)]
pub(crate) fn test_key() -> crate::buffer::BinaryData<crate::buffer::SharedImpl> {
    crate::buffer::BinaryData::new(7, crate::buffer::SharedImpl::test_new(b"kittens"))
}

#[cfg(test)]
mod tests {

    use super::{is_kv_store_message, END, START};

    #[test]
    fn test_is_kv_store_message() {
        assert!(!is_kv_store_message(START - 1));
        assert!(is_kv_store_message(START));
        assert!(is_kv_store_message(END));
        assert!(!is_kv_store_message(END + 1));
    }
}
