pub mod data;
pub mod algorithms;
pub mod heuristics;
pub mod problems;

use std::cmp::Reverse;

#[cfg(test)]
mod tests {
    use data::BinPacking;

    use super::*;

    #[test]
    fn example_code() {
        // example data
        let items = vec![
            data::Item { number: 0, dimensions: vec![2.0, 4.0] },
            data::Item { number: 1, dimensions: vec![3.0, 1.0] },
            data::Item { number: 2, dimensions: vec![4.0, 3.0] },
            data::Item { number: 3, dimensions: vec![1.0, 2.0] },
        ];

        let bin_capacity = vec![5.0, 5.0];

        let solution = algorithms::first_fit_descending(items.iter().collect(), bin_capacity, heuristics::dimensions_product_heuristic);

        // print results
        for (i, bin) in solution.bins.iter().enumerate() {
            println!("Bin {}: {:?}", i + 1, bin);
        }
    }

    #[test]
    fn example_from_file() {
        let bin_packing: BinPacking<f64> = data::BinPacking::load_from_file("dataset/data.txt");

        let solution = algorithms::first_fit_descending_bin_centric(bin_packing.items.iter().collect(), bin_packing.bin_capacity, heuristics::norm_based_greedy_heuristic);

        // print results
        for (i, bin) in solution.bins.iter().enumerate() {
            println!("Bin {}: {:?}", i + 1, bin);
        }
    }

    #[test]
    fn example_bin_packing() {
        let solution = problems::bin_packing::<f64>(vec![2.0, 3.0, 4.0, 1.0], 5.0);
        // print results
        for (i, bin) in solution.bins.iter().enumerate() {
            println!("Bin {}: {:?}", i + 1, bin);
        }
    }

    #[test]
    fn example_integer() {
        let solution = problems::bin_packing::<i32>(vec![2, 3, 4, 1], 5);
        // print results
        for (i, bin) in solution.bins.iter().enumerate() {
            println!("Bin {}: {:?}", i + 1, bin);
        }
    }
}
