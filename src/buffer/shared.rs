use std::{
    fmt::{self, Debug, Formatter},
    hash::Hash,
};

use super::{Block, BlockMut, Releaser};

#[derive(Clone)]
pub struct SharedImpl {
    inner: Arc<Block>,
    _releaser: Option<Releaser>,
}

impl Debug for SharedImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedImpl")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Eq for SharedImpl {}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for SharedImpl {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialOrd for SharedImpl {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Ord for SharedImpl {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Hash for SharedImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl PartialOrd for SharedImpl {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SharedImpl {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner
            .as_ref()
            .as_slice()
            .cmp(other.inner.as_ref().as_slice())
    }
}

impl SharedImpl {
    pub(crate) fn new(inner: Block, releaser: Releaser) -> Self {
        Self {
            inner: Arc::new(inner),
            _releaser: Some(releaser),
        }
    }

    pub fn test_new(data: &[u8]) -> Self {
        let mut block = BlockMut::new(data.len());
        block.as_mut_slice().copy_from_slice(data);
        Self {
            inner: Arc::new(block.into_block()),
            _releaser: None,
        }
    }
}

impl AsRef<[u8]> for SharedImpl {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref().as_slice()
    }
}

impl super::Shared for SharedImpl {}
