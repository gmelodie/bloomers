pub mod hasher;
use hasher::Hashable;

pub struct BloomFilter<T: Hashable> {
    bytes: Vec<u8>,
    hashes: Vec<Box<dyn Fn(T) -> (usize, u8)>>,
    // TODO: Figure out k and m based on false positives
}

impl<T: Hashable> BloomFilter<T> {
    /// Creates new BloomFilter
    /// n: rough amount of elements you want to store (at max)
    /// pfp: probability of a false positive (between 0 and 1)
    pub fn new(n: usize, pfp: f64) -> Self {
        assert!(
            pfp < 1.0,
            "Probability of a false positive must be between 0.0 and 1.0, got {pfp}"
        );
        assert!(
            pfp > 0.0,
            "Probability of a false positive must be between 0.0 and 1.0, got {pfp}"
        );

        let two = 2.0 as f64;
        let ln2 = two.ln();
        let m = (-1.0 * (n as f64 * pfp.ln() / (ln2 * ln2))).ceil() as usize;
        let k = (-1.0 * (pfp.ln() / ln2)).ceil() as usize;
        println!("Creating bloom filter:\nn = {n}\nm = {m}\nk = {k}");
        Self {
            bytes: vec![0; m / 8], // the +7 is a trick to round the division up
            hashes: (0..k).map(|_| hasher::new_hash(m / 8)).collect(),
        }
    }
    pub fn contains(&self, item: T) -> bool {
        for hash in self.hashes.iter() {
            let (byte_offset, byte) = (hash)(item.clone());
            if byte & self.bytes[byte_offset] == 0 {
                // bit not set, element not present
                return false;
            }
        }
        true
    }
    pub fn add(&mut self, item: T) {
        let item_clone = item.clone();
        let present = self.contains(item_clone);
        if !present {
            for hash in self.hashes.iter() {
                let (byte_offset, byte) = (hash)(item.clone());
                self.bytes[byte_offset] |= byte;
            }
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
