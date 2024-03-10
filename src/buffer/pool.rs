use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use super::{OwnedImpl, Pool, Releaser};

pub struct PoolImpl {
    tx: SyncSender<()>,
    rx: Receiver<()>,
    block_size: usize,
}

impl PoolImpl {
    pub fn new(block_size: usize, capacity: usize) -> Self {
        let (tx, rx) = sync_channel(capacity);

        for _ in 0..capacity {
            tx.send(()).unwrap();
        }

        Self { tx, rx, block_size }
    }
}

impl Pool for PoolImpl {
    type Buffer = OwnedImpl;

    fn acquire(&self) -> Result<Self::Buffer, crate::Error> {
        self.rx.recv().expect("failed to acquire buffer");

        Ok(OwnedImpl::new(
            self.block_size,
            Releaser::new(self.tx.clone()),
        ))
    }
}
