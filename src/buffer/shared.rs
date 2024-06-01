use std::{
    cmp,
    fmt::{self, Debug, Formatter},
    hash::Hash,
};

use super::{Block, Releaser};

#[derive(Clone)]
pub struct SharedImpl {
    inner: Block,
    _releaser: Option<Releaser>,
}

impl Drop for SharedImpl {
    fn drop(&mut self) {
        if let Some(mut releaser) = self._releaser.take() {
            releaser.release(&mut self.inner);
        }
    }
}

impl Debug for SharedImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedImpl")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Eq for SharedImpl {}

impl PartialEq for SharedImpl {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialOrd for SharedImpl {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SharedImpl {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl Hash for SharedImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl SharedImpl {
    pub(crate) fn new(inner: Block, releaser: Releaser) -> Self {
        Self {
            inner,
            _releaser: Some(releaser),
        }
    }

    pub fn test_new(data: &[u8]) -> Self {
        let mut block = Block::new(data.len());
        block.as_mut_slice().copy_from_slice(data);
        Self {
            inner: block,
            _releaser: None,
        }
    }
}

impl AsRef<[u8]> for SharedImpl {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

impl super::Shared for SharedImpl {}
