use crate::data::{Bin, Item};
use std::cmp::{Reverse};

// First Fit Descending (item-centric)
pub fn first_fit_descending<F>(items: Vec<Item>, bin_capacity: Vec<u32>, cmp_key: F) -> Vec<Bin>
where 
    F: Fn(&Item) -> Reverse<u32>
{
    let mut bins: Vec<Bin> = Vec::new();

    let mut sorted_items = items.clone();

    // sort items by the custom rule
    sorted_items.sort_by_key(cmp_key);

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

// First Fit Descending Bin-centric
pub fn first_fit_descending_bin_centric<F>(items: Vec<Item>, bin_capacity: Vec<u32>, cmp_key: F) -> Vec<Bin>
where 
    F: Fn(&Item, &Bin) -> u32
{
    let mut bins: Vec<Bin> = Vec::new();

    // let mut sorted_items = items.clone();
    // sorted_items.sort_by_key(cmp_key);
    let mut used_item_num = 0;

    let mut sorted_items = items.clone();

    while used_item_num < items.len() {
        let mut new_bin = Bin::new(bin_capacity.clone());

        loop {
            // place "largest" remaining item that fits in the bin
            sorted_items.sort_by_key(|item| Reverse(cmp_key(item, &new_bin)));
            
            let mut placed = false;
            for (i, item) in sorted_items.iter().enumerate() {
                if new_bin.can_fit(&item) {
                    new_bin.add_item(&item);
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
        bins.push(new_bin);
    }

    bins
}


// First Fit 算法
pub fn first_fit(items: Vec<Item>, bin_capacity: Vec<u32>) -> Vec<Bin> {
    let mut bins: Vec<Bin> = Vec::new();

    for item in items {
        let mut placed = false;
        
        // 尝试将项目放入已有的箱子
        for bin in bins.iter_mut() {
            if bin.can_fit(&item) {
                bin.add_item(&item);
                placed = true;
                break;
            }
        }

        // 如果没有合适的箱子，创建一个新的箱子
        if !placed {
            let mut new_bin = Bin::new(bin_capacity.clone());
            new_bin.add_item(&item);
            bins.push(new_bin);
        }
    }

    bins
}
