use num::Signed;
use std::convert::TryInto;

pub trait Hashable: TryInto<usize> + Clone + Signed {}

impl<T: TryInto<usize> + Clone + Signed> Hashable for T {}

pub fn new_hash<T>(n: usize) -> Box<dyn Fn(T) -> (usize, u8)>
where
    T: Hashable,
{
    Box::new(move |item: T| {
        // naive approach to a "hash" function using mod and division
        // (e.g. -8 will be 9th bit; 0 will be 1st bit; etc)
        let item_abs = match item.abs().try_into() {
            Ok(abs_item) => abs_item,
            Err(_) => panic!("Unable to convert type to usize"),
        };
        let byte_offset = item_abs / n;
        let bit_offset = item_abs % n;

        assert!(byte_offset < n); // check if it doesn't exceed vec capacity
        assert!(bit_offset < 8);
        let byte: u8 = 0b0000_0001;
        (byte_offset, byte << bit_offset)
    })
}
