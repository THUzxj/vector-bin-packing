mod data;
mod algorithms;
mod heuristics;
use std::cmp::Reverse;

#[cfg(test)]
mod tests {
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

        let bins = algorithms::first_fit_descending(items, bin_capacity, |a| Reverse(a.dimensions_product()));

        // print results
        for (i, bin) in bins.iter().enumerate() {
            println!("Bin {}: {:?}", i + 1, bin);
        }
    }

    #[test]
    fn example_from_file() {
        let bin_packing = data::BinPacking::load_from_file("dataset/data.txt");

        let solution = algorithms::first_fit_descending_bin_centric(bin_packing.items, bin_packing.bin_capacity, heuristics::norm_based_greedy_heuristic);

        // print results
        for (i, bin) in solution.bins.iter().enumerate() {
            println!("Bin {}: {:?}", i + 1, bin);
    }
}
}
