use crossbeam_channel::{bounded, Receiver, Sender};
use log::trace;

use super::{block::Block, BufferOwner, OwnedImpl, Pool, Releaser};

#[derive(Clone)]
pub struct PoolImpl {
    tx: Sender<Block>,
    rx: Receiver<Block>,

    block_size: usize,
    capacity: usize,
}

impl PoolImpl {
    pub fn new(block_size: usize, capacity: usize) -> Self {
        let (tx, rx) = bounded(capacity);

        for _ in 0..capacity {
            tx.send(Block::new(block_size)).unwrap();
        }

        Self {
            tx,
            rx,
            block_size,
            capacity,
        }
    }
}

impl Pool for PoolImpl {
    type Buffer = OwnedImpl;

    fn acquire(&self, reason: impl BufferOwner) -> Result<Self::Buffer, crate::Error> {
        trace!("acquiring buffer for {}", reason.why());
        #[cfg(feature = "timeout")]
        let block = self
            .rx
            .recv_timeout(std::time::Duration::from_secs(1))
            .expect("failed to acquire buffer");
        #[cfg(not(feature = "timeout"))]
        let block = self.rx.recv().expect("failed to acquire buffer");
        trace!("acquired buffer for {}", reason.why());
        Ok(OwnedImpl::new(block, Releaser::new(self.tx.clone())))
    }

    fn block_size(&self) -> usize {
        self.block_size
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}
