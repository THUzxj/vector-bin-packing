use crate::data::{Bin, Item};

pub fn dot_product_heuristic(item: &Item, bin: &Bin) -> u32 {
    item.dimensions.iter().zip(bin.current_remaining().iter()).map(|(a, b)| a * b).sum()
}

pub fn norm_based_greedy_heuristic(item: &Item, bin: &Bin) -> u32 {
    u32::MAX - item.dimensions.iter().zip(bin.current_remaining().iter()).map(|(a, b)| (if b >= a {b - a} else {a - b}).pow(2) ).sum::<u32>()
}
