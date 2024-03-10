use std::{cell::UnsafeCell, fmt::Debug, hash::Hash, ops::Range, sync::Arc};

/// A read-write block of memory. This the mutable version of `Block`.
#[derive(Clone)]
pub(crate) struct Block {
    data: Arc<UnsafeCell<Vec<u8>>>,
    range: Range<usize>,
}

impl Block {
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
impl From<Vec<u8>> for Block {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

#[cfg(test)]
impl From<&'static [u8]> for Block {
    fn from(data: &'static [u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}

#[cfg(test)]
impl From<&'static str> for Block {
    fn from(data: &'static str) -> Self {
        Self {
            data: data.as_bytes().to_vec(),
        }
    }
}

/// A read-write block of memory. This the mutable version of `Block`.
#[derive(Debug)]
pub struct BlockMut {
    data: Vec<u8>,
}

impl BlockMut {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity],
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn into_block(self) -> Block {
        Block { data: self.data }
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn split_at(&mut self, index: usize) -> Self {
        let right = self.data.split_off(index);
        let left = std::mem::replace(&mut self.data, right);
        Self {
            data: left,
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

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

unsafe impl Send for Block {}
unsafe impl Sync for Block {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let mut block = Block::new(10);
        assert_eq!(
            format!("{:?}", block),
            "Block { data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], range: 0..10 }"
        );

        let other = block.split_at(4);
        assert_eq!(
            format!("{:?}", other),
            "Block { data: [0, 0, 0, 0], range: 0..4 }"
        );
        assert_eq!(
            format!("{:?}", block),
            "Block { data: [0, 0, 0, 0, 0, 0], range: 4..10 }"
        );
    }
}
