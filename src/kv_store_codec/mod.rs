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

pub fn is_kv_store_message(kind: u8) -> bool {
    (START..=END).contains(&kind)
}
