use crate::data::{Bin, Item};
use std::collections::HashMap;
use std::io::Write;

use std::error::Error;

use good_lp::{constraint, default_solver, Solution, SolverModel, variables};

// Bin Packing Problem Solution
pub struct BPSolution {
    pub bins: Vec<Bin>,
    pub item_bin_mapping: HashMap<usize, usize>,
}

impl BPSolution {
    pub fn new() -> BPSolution {
        BPSolution { bins: Vec::new(), item_bin_mapping: HashMap::new() }
    }

    pub fn write_to_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::File::create(file_path)?;
        for (k, v) in self.item_bin_mapping.iter() {
            writeln!(file, "{}: {}", k, v)?;
        }
        Ok(())
    }
}

// First Fit Descending (Item-centric version)
pub fn first_fit_descending<F>(items: Vec<Item>, bin_capacity: Vec<f64>, cmp_key: F) -> Vec<Bin>
where 
    F: Fn(&Item) -> f64
{
    let mut bins: Vec<Bin> = Vec::new();

    let mut sorted_items = items.clone();

    // sort items by the custom rule
    // sorted_items.sort_by_key(cmp_key);
    sorted_items.sort_by(|a, b| cmp_key(b).partial_cmp(&cmp_key(a)).unwrap());

    // put items into bins
    for item in sorted_items {
        let mut placed = false;

        for bin in bins.iter_mut() {
            if bin.can_fit(&item) {
                bin.add_item(&item);
                placed = true;
                break;
            }
        }

        if !placed {
            let mut new_bin = Bin::new(bin_capacity.clone());
            new_bin.add_item(&item);
            bins.push(new_bin);
        }
    }

    bins
}

// First Fit Descending (Bin-centric version)
pub fn first_fit_descending_bin_centric<F>(items: Vec<Item>, bin_capacity: Vec<f64>, cmp_key: F) -> BPSolution
where 
    F: Fn(&Item, &Bin) -> f64
{
    let mut solution = BPSolution::new();
    let mut used_item_num = 0;

    let mut sorted_items = items.clone();

    while used_item_num < items.len() {
        let mut new_bin = Bin::new(bin_capacity.clone());

        loop {
            // place "largest" remaining item that fits in the bin
            sorted_items.sort_by(|a, b| cmp_key(b, &new_bin).partial_cmp(&cmp_key(a, &new_bin)).unwrap());
            
            let mut placed = false;
            for (i, item) in sorted_items.iter().enumerate() {
                if new_bin.can_fit(&item) {
                    new_bin.add_item(&item);
                    solution.item_bin_mapping.insert(item.number, solution.bins.len());
                    sorted_items.remove(i);
                    used_item_num += 1;
                    placed = true;
                    break;
                }
            }
            if placed == false {
                break;
            }
        }
        solution.bins.push(new_bin);
    }

    solution
}


// First Fit
pub fn first_fit(items: Vec<Item>, bin_capacity: Vec<f64>) -> BPSolution {
    let mut solution = BPSolution::new();

    for item in items {
        let mut placed = false;
        // try to place the item in an existing bin
        for bin in solution.bins.iter_mut() {
            if bin.can_fit(&item) {
                bin.add_item(&item);
                placed = true;
                break;
            }
        }

        // if the item cannot be placed in any existing bin, create a new bin
        if !placed {
            let mut new_bin = Bin::new(bin_capacity.clone());
            new_bin.add_item(&item);
            solution.bins.push(new_bin);
        }
    }

    solution
}


// pub fn linear_programming() -> Result<(), Box<dyn Error>> {
//     // calculate all the configurations first
    
//     variables! {
//         vars:
//                a <= 1;
//           2 <= b <= 4;
//     } // variables can also be added dynamically
//     let solution = vars.maximise(10 * (a - b / 5) - b)
//         .using(default_solver) // multiple solvers available
//         .with(constraint!(a + 2 <= b))
//         .with(constraint!(1 + a >= 4 - b))
//         .solve()?;
//     println!("a={}   b={}", solution.value(a), solution.value(b));
//     println!("a + b = {}", solution.eval(a + b));
//     Ok(())
// }
