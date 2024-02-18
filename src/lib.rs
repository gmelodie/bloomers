pub mod hasher;
use hasher::Hashable;

pub struct BloomFilter<T: Hashable> {
    bytes: Vec<u8>,
    hash: Box<dyn Fn(T) -> (usize, u8)>, // TODO: multiple hashes
                                         // TODO: Figure out k and m based on false positives
}

impl<T: Hashable> BloomFilter<T> {
    /// Creates new BloomFilter
    /// n: amount of bits in filter
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; (n + 7) / 8], // the +7 is a trick to round the division up
            hash: hasher::new_hash(n),
        }
    }
    pub fn contains(&self, item: T) -> bool {
        let (byte_offset, byte) = (self.hash)(item);
        if byte & self.bytes[byte_offset] == 0 {
            // bit not set, element not present
            return false;
        }
        true
    }
    pub fn add(&mut self, item: T) {
        let item_clone = item.clone();
        let present = self.contains(item_clone);
        if !present {
            let (byte_offset, byte) = (self.hash)(item);
            self.bytes[byte_offset] |= byte;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_one_item() {
        let mut filter = BloomFilter::new(5);
        filter.add(3);
        assert_eq!(filter.bytes[0], 0b0000_1000); // 0b0000_0001 << 3
    }
}
