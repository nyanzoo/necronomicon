use std::{
    fmt::Debug,
    sync::{mpsc::SyncSender, Arc},
};

use crate::Error;

mod block;
pub use block::{Block, BlockMut};

mod data;
pub use data::BinaryData;

mod owned;
pub use owned::OwnedImpl;

mod pool;
pub use pool::PoolImpl;

mod shared;
pub use shared::SharedImpl;

mod str;
pub use str::ByteStr;

/// A thread-safe read-only buffer.
pub trait Shared:
    Send + Sync + AsRef<[u8]> + Clone + Debug + Eq + PartialEq + PartialOrd + Ord
{
    /// Returns the length of the buffer.
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    /// Returns `true` if the buffer is empty.
    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    /// Returns a slice of the buffer.
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
}

/// A read-write buffer.
pub trait Owned {
    type Shared: Shared;

    /// Returns `true` if the buffer is empty.
    fn is_empty(&self) -> bool {
        self.filled_len() == 0
    }

    /// Returns a mutable slice of the buffer.
    fn unfilled(&mut self) -> &mut [u8];

    /// Returns the capacity of the unfilled part of the buffer.
    fn unfilled_capacity(&self) -> usize;

    /// Fills the buffer with the given length.
    fn fill(&mut self, len: usize);

    /// Returns a slice of the buffer.
    fn filled(&self) -> &[u8];

    /// Returns the length of the filled part of the buffer.
    fn filled_len(&self) -> usize;

    /// Takes ownership of the buffer and returns a read-only buffer.
    fn into_shared(self) -> Self::Shared;

    /// Splits the buffer into two at the given index. Returns the left part and keeps the right part.
    fn split_at(&mut self, index: usize) -> Self;
}

pub fn fill<O>(buffer: &mut O, data: &[u8])
where
    O: Owned,
{
    let len = data.len().min(buffer.unfilled_capacity());
    buffer.unfilled()[..len].copy_from_slice(&data[..len]);
    buffer.fill(len);
}

pub trait Pool {
    type Buffer: Owned;

    fn acquire(&self) -> Result<Self::Buffer, Error>;
}

/// A mechanism for releasing memory back to the pool.
/// When this is dropped, it releases the memory back to the pool.
#[derive(Clone)]
pub(crate) struct Releaser(Arc<SyncSender<()>>);

impl Releaser {
    pub fn new(sender: SyncSender<()>) -> Self {
        Self(Arc::new(sender))
    }
}

impl Drop for Releaser {
    fn drop(&mut self) {
        if Arc::strong_count(&self.0) == 1 {
            self.0.send(()).expect("failed to release buffer");
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub fn binary_data(data: &[u8]) -> BinaryData<SharedImpl> {
    BinaryData::new(data.len(), SharedImpl::test_new(data))
}

#[cfg(any(test, feature = "test"))]
pub fn byte_str(data: &[u8]) -> ByteStr<SharedImpl> {
    ByteStr::new(binary_data(data))
}
