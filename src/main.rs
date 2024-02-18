use bloomers::BloomFilter;

fn main() {
    let mut filter = BloomFilter::new();
    filter.add(3);
    filter.add(4);
    filter.add(6);
    assert!(filter.contains(3));
    assert!(filter.contains(4));
    assert!(!filter.contains(5)); // does not contain 5
    assert!(filter.contains(6));
    println!("bloom filter")
}
