use crate::{algorithms::{self, BPSolution}, data::Item, heuristics};



// 1 dimension bin packing
pub fn bin_packing<T>(sizes: Vec<T>, capacity: T) -> BPSolution<T>
where T: Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + std::convert::TryInto<f64> + std::fmt::Debug
{
    let items: Vec<Item<T>> = sizes.iter().enumerate().map(|(i, &size)| {
        Item { number: i, dimensions: vec![size] }
    }).collect();

    let bin_capacity: Vec<T> = vec![capacity];

    let solution = algorithms::first_fit_descending(items.iter().collect(), bin_capacity, heuristics::dimensions_product_heuristic);
    solution
}
