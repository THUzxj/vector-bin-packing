use crate::data::{Bin, Item};

// Item heuristics

pub fn dimensions_product(item: &Item) -> f64 {
    f64::MAX - item.dimensions.iter().product::<f64>()
}

// Item and Bin heuristics

pub fn dot_product_heuristic(item: &Item, bin: &Bin) -> f64 {
    f64::MAX - item.dimensions.iter().zip(bin.current_remaining().iter()).map(|(a, b)| a * b).sum::<f64>()
}

pub fn norm_based_greedy_heuristic(item: &Item, bin: &Bin) -> f64 {
    f64::MAX - item.dimensions.iter().zip(bin.current_remaining().iter()).map(|(a, b)| (if b >= a {b - a} else {a - b}).powf(2.0) ).sum::<f64>()
}

pub fn least_allocated_heuristic(item: &Item, bin: &Bin) -> f64 {
    bin.current_remaining().iter().product()
}
