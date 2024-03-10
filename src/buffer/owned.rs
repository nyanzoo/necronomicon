use std::cmp;

use buffer::Owned;

use crate::buffer::{self, block::BlockMut, Releaser};

use super::shared::SharedImpl;

pub struct OwnedImpl {
    inner: Box<BlockMut>,
    filled: usize,
    releaser: Releaser,
}

impl OwnedImpl {
    pub(crate) fn new(block_size: usize, releaser: Releaser) -> Self {
        let inner = Box::new(BlockMut::new(block_size));
        Self {
            inner,
            filled: 0,
            releaser,
        }
    }
}

impl Owned for OwnedImpl {
    type Shared = SharedImpl;

    fn unfilled(&mut self) -> &mut [u8] {
        let slice = self.inner.as_mut_slice();
        &mut slice[self.filled..]
    }

    fn unfilled_capacity(&self) -> usize {
        self.inner.capacity() - self.filled
    }

    fn fill(&mut self, len: usize) {
        self.filled += len;
    }

    fn filled(&self) -> &[u8] {
        let slice = self.inner.as_slice();
        &slice[..self.filled]
    }

    fn filled_len(&self) -> usize {
        self.filled
    }

    fn into_shared(self) -> Self::Shared {
        let Self {
            inner,
            filled: _,
            releaser,
        } = self;

        SharedImpl::new(inner.into_block(), releaser)
    }

    fn split_at(&mut self, index: usize) -> Self {
        let other = Self {
            inner: Box::new(self.inner.split_at(index)),
            filled: cmp::min(self.filled, index),
            releaser: self.releaser.clone(),
        };

        self.filled -= other.filled;

        other
    }
}
