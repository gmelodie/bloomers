use num::Signed;
use rand::Rng;
use std::convert::TryInto;

pub trait Hashable: TryInto<usize> + Clone + Signed {}

impl<T: TryInto<usize> + Clone + Signed> Hashable for T {}

pub fn new_hash<T>(n: usize) -> Box<dyn Fn(T) -> (usize, u8)>
where
    T: Hashable,
{
    let mut rng = rand::thread_rng();
    let salt: usize = rng.gen();

    Box::new(move |item: T| {
        // naive approach to a "hash" function using mod and division
        // added a salt to make every hash slightly different
        let mut hash = match item.abs().try_into() {
            Ok(abs_item) => abs_item,
            Err(_) => panic!("Unable to convert type to usize"),
        };

        let number_of_bytes = (n + 7) / 8;
        hash += salt;

        let byte_offset = (hash / 8) % number_of_bytes;
        let bit_offset = hash % 8;

        assert!(byte_offset < number_of_bytes); // check if it doesn't exceed vec capacity
        assert!(bit_offset < 8);
        let byte: u8 = 0b0000_0001;
        (byte_offset, byte << bit_offset)
    })
}
