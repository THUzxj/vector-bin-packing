mod data;
mod algorithms;
mod heuristics;
use std::cmp::Reverse;

fn example_code() {
    // 示例数据
    let items = vec![
        data::Item { dimensions: vec![2, 4] },
        data::Item { dimensions: vec![3, 1] },
        data::Item { dimensions: vec![4, 3] },
        data::Item { dimensions: vec![1, 2] },
    ];

    let bin_capacity = vec![5, 5];

    let bins = algorithms::first_fit_descending(items, bin_capacity, |a| Reverse(a.dimensions_product()));

    // 输出结果
    for (i, bin) in bins.iter().enumerate() {
        println!("Bin {}: {:?}", i + 1, bin);
    }
}

fn example_from_file() {
    let bin_packing = data::BinPacking::load_from_file("dataset/data.txt");

    let bins = algorithms::first_fit_descending_bin_centric(bin_packing.items, bin_packing.bin_capacity, heuristics::norm_based_greedy_heuristic);

    // 输出结果
    for (i, bin) in bins.iter().enumerate() {
        println!("Bin {}: {:?}", i + 1, bin);
    }
}

fn main() {
    example_from_file();
}
