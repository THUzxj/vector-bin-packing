use crate::data::{Bin, Item};
use std::{collections::HashMap, fmt::Debug};
use std::io::Write;

use std::error::Error;

use good_lp::{constraint, default_solver, Solution, SolverModel, variables};

// Bin Packing Problem Solution
pub struct BPSolution<T> {
    pub bins: Vec<Bin<T>>,
    pub item_bin_mapping: HashMap<usize, usize>,
}

impl<T: Debug> BPSolution<T> {
    pub fn new() -> BPSolution<T> {
        BPSolution { bins: Vec::new(), item_bin_mapping: HashMap::new() }
    }

    pub fn write_to_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::File::create(file_path)?;
        for (k, v) in self.item_bin_mapping.iter() {
            writeln!(file, "{}: {}", k, v)?;
        }
        Ok(())
    }

    pub fn print(&self) {
        for (i, bin) in self.bins.iter().enumerate() {
            println!("Bin {}: {:?}", i + 1, bin);
        }
    }
}

// First Fit Descending (Item-centric version)
pub fn first_fit_descending<F, T>(items: Vec<&Item<T>>, bin_capacity: Vec<T>, cmp_key: F) -> BPSolution<T>
where 
    F: Fn(&Item<T>) -> T,
    T: Debug + Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + TryInto<f64> 
{
    let mut solution = BPSolution::new();

    let mut sorted_items = items.clone();

    // sort items by the custom rule
    // sorted_items.sort_by_key(cmp_key);
    sorted_items.sort_by(|a, b| cmp_key(b).partial_cmp(&cmp_key(a)).unwrap());

    // put items into bins
    for item in sorted_items {
        let mut placed = false;

        for bin in solution.bins.iter_mut() {
            if bin.can_fit(&item) {
                bin.add_item(&item);
                solution.item_bin_mapping.insert(item.number, solution.bins.len());
                placed = true;
                break;
            }
        }

        if !placed {
            let mut new_bin = Bin::new(bin_capacity.clone());
            new_bin.add_item(&item);
            solution.bins.push(new_bin);
        }
    }

    solution
}

// First Fit Descending (Bin-centric version)
pub fn first_fit_descending_bin_centric<F, T>(items: Vec<&Item<T>>, bin_capacity: Vec<T>, cmp_key: F) -> BPSolution<T>
where 
    F: Fn(&Item<T>, &Bin<T>) -> T,
    T: Debug + Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + TryInto<f64>
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
pub fn first_fit<T>(items: Vec<&Item<T>>, bin_capacity: Vec<T>) -> BPSolution<T>
where T: Debug + Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + TryInto<f64> 
{
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
