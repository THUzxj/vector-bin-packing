use num_traits::{real::Real, zero, Float, Zero};

use crate::data::{Bin, Item};

// Item heuristics

pub fn dimensions_product_heuristic<T: Copy + Clone + num_traits::NumAssign>(item: &Item<T>) -> T
where T: Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + TryInto<f64>
{
    let mut value: T = zero();
    for dim in item.dimensions.iter() {
        value += dim.clone();
    }
    value
}

// Item and Bin heuristics

pub fn dot_product_heuristic<T>(item: &Item<T>, bin: &Bin<T>) -> T 
where T: Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + TryInto<f64>
{
    let mut value: T = zero();
    for (a, b) in item.dimensions.iter().zip(bin.current_remaining().iter()) {
        value += a.clone() * b.clone();
    }
    value
}

pub fn norm_based_greedy_heuristic<T>(item: &Item<T>, bin: &Bin<T>) -> T 
where T: Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + TryInto<f64> + Float 
{
    let mut value: T = zero();
    for (a, b) in item.dimensions.iter().zip(bin.current_remaining().iter()) {
        value += (a.clone() - b.clone()).powi(2);
    }
    value
}

pub fn least_allocated_heuristic<T>(_: &Item<T>, bin: &Bin<T>) -> T 
where T: Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + TryInto<f64> + for<'a> std::iter::Product<&'a T>
{
    bin.current_remaining().iter().product()
}
