pub const START: u8 = 0x10;

/// A message that can be used to insert into the kv store.
pub const INSERT: u8 = START;
/// A message that can be used to get from the kv store.
pub const GET: u8 = START + 1;
/// A message that can be used to remove from the kv store.
pub const DELETE: u8 = START + 2;

pub const END: u8 = START + 3;


pub fn is_kv_store_message(kind: u8) -> bool {
    (START..END).contains(&kind)
}