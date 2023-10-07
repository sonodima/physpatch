use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use memflow::types::Address;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Range {
    address: Address,
    size: usize,
}

impl Range {
    pub fn new(address: Address, size: usize) -> Self {
        Self { address, size }
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Hash for Range {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.to_umem().hash(state);
        self.size.hash(state);
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.address.partial_cmp(&other.address)
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.address.cmp(&other.address)
    }
}
