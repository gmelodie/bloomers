pub struct BloomFilter {
    items: Vec<i32>,
}

impl BloomFilter {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn contains(&self, item: i32) -> bool {
        self.items.contains(&item)
    }
    pub fn add(&mut self, item: i32) {
        let present = self.contains(item);
        if !present {
            self.items.push(item);
        }
    }
}
