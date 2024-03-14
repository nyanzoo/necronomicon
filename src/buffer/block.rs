/// A read-only block of memory.
/// See `Packet` for the sizes of different packets. We allow
/// end-user to decide pool block size for packets, so it is possible
/// that packets may not fit.
#[derive(Debug)]
pub struct Block {
    data: Vec<u8>,
}

impl Block {
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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
        Self { data: left }
    }
}
