use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use super::{block::Block, OwnedImpl, Pool, Releaser};

pub struct PoolImpl {
    tx: SyncSender<Block>,
    rx: Receiver<Block>,
}

impl PoolImpl {
    pub fn new(block_size: usize, capacity: usize) -> Self {
        let (tx, rx) = sync_channel(capacity);

        for _ in 0..capacity {
            tx.send(Block::new(block_size)).unwrap();
        }

        Self { tx, rx }
    }
}

impl Pool for PoolImpl {
    type Buffer = OwnedImpl;

    fn acquire(&self) -> Result<Self::Buffer, crate::Error> {
        let block = self.rx.recv().expect("failed to acquire buffer");

        Ok(OwnedImpl::new(block, Releaser::new(self.tx.clone())))
    }
}
