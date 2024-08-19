mod data;
mod algorithms;
mod heuristics;
use std::cmp::Reverse;



fn example_vm() {
    let file_path = "dataset/cpu_usage_matrix_0.01_0_100000.csv";
    let file = std::fs::read_to_string(file_path).expect("Failed to read file");
    let mut lines = file.lines();

    let items: Vec<data::Item> = lines.enumerate().map(|(i, line)| {
            let dimensions: Vec<f64> = line.split_terminator(",").map(|s| s.parse().unwrap()).collect();
            data::Item { number: i,  dimensions }
        }).collect();

    let dimension_num = items[0].dimensions.len();
    // all capacities are the same
    let bin_capacity: Vec<f64> = vec![64.0; dimension_num as usize];
    let bin_packing = data::BinPacking::new(dimension_num.try_into().unwrap(), items, bin_capacity);

    let solution = algorithms::first_fit_descending_bin_centric(bin_packing.items, bin_packing.bin_capacity, heuristics::dot_product_heuristic);

    // print results
    for (i, bin) in solution.bins.iter().enumerate() {
        println!("Bin {}: {:?}", i + 1, bin);
    }

    solution.write_to_file("output/packing_results.txt");
}

fn main() {
    example_vm();
}
