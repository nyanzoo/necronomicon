use std::{cell::UnsafeCell, fmt::Debug, hash::Hash, ops::Range, sync::Arc};

/// A read-write block of memory. This the mutable version of `Block`.
#[derive(Clone)]
pub(crate) struct Block {
    data: Arc<UnsafeCell<Vec<u8>>>,
    range: Range<usize>,
}

impl Block {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Arc::new(UnsafeCell::new(vec![0; capacity])),
            range: 0..capacity,
        }
    }

    pub fn release(&mut self) -> Self {
        let Self { data, range: _ } = self;
        let capacity = {
            let data = unsafe { &mut *data.get() };
            data.fill(0);
            data.len()
        };
        Self {
            data: data.clone(),
            range: 0..capacity,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        let data = unsafe { &*self.data.get() };
        &data[self.range.clone()]
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let data = unsafe { &mut *self.data.get() };
        &mut data[self.range.clone()]
    }

    pub fn capacity(&self) -> usize {
        self.range.end - self.range.start
    }

    pub fn split_at(&mut self, index: usize) -> Self {
        let right = (self.range.start + index)..self.range.end;
        let left = self.range.start..(self.range.start + index);
        self.range = right;
        Self {
            data: self.data.clone(),
            range: left,
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("data", &self.as_slice())
            .field("range", &self.range)
            .finish()
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Block {}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

unsafe impl Send for Block {}
unsafe impl Sync for Block {}
