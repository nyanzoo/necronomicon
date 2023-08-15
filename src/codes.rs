// Ok cases
pub const SUCCESS: u8 = 0x00;

// Error cases
pub const SERVER_BUSY: u8 = 0x10;
// dequeue
pub const QUEUE_DOES_NOT_EXIST: u8 = 0x11;
pub const QUEUE_ALREADY_EXISTS: u8 = 0x12;
pub const QUEUE_FULL: u8 = 0x13;
pub const QUEUE_EMPTY: u8 = 0x14;

// kv store
pub const KEY_DOES_NOT_EXIST: u8 = 0x15;
pub const KEY_ALREADY_EXISTS: u8 = 0x16;

// TODO errors, should be updated later!
// errors (start at 0xa0)
pub const FAILED_TO_PUSH_TO_TRANSACTION_LOG: u8 = 0xa0;
pub const INTERNAL_ERROR: u8 = 0xff;
