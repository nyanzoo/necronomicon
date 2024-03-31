use std::cmp;

use buffer::Owned;

use crate::buffer::{self, block::Block, Releaser};

use super::shared::SharedImpl;

pub struct OwnedImpl {
    inner: Block,
    filled: usize,
    releaser: Releaser,
}

impl OwnedImpl {
    pub(crate) fn new(inner: Block, releaser: Releaser) -> Self {
        Self {
            inner,
            filled: 0,
            releaser,
        }
    }
}

impl Drop for OwnedImpl {
    fn drop(&mut self) {
        self.releaser.release(&mut self.inner);
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
        SharedImpl::new(self.inner.clone(), self.releaser.clone())
    }

    fn split_at(&mut self, index: usize) -> Self {
        let other = Self {
            inner: self.inner.split_at(index),
            filled: cmp::min(self.filled, index),
            releaser: self.releaser.clone(),
        };

        self.filled -= other.filled;

        other
    }
}
