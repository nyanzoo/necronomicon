pub const START: u8 = 0x10;

/// A message that can be used to insert into the kv store.
pub const PUT: u8 = START;
/// An ack for a put message.
pub const PUT_ACK: u8 = START + 1;
/// A message that can be used to get from the kv store.
pub const GET: u8 = START + 1;
/// An ack for a get message.
pub const GET_ACK: u8 = START + 2;
/// A message that can be used to remove from the kv store.
pub const DELETE: u8 = START + 3;
/// An ack for a delete message.
pub const DELETE_ACK: u8 = START + 4;

pub const END: u8 = START + 4;

pub fn is_kv_store_message(kind: u8) -> bool {
    (START..=END).contains(&kind)
}
