use std::{fmt::Debug, str::FromStr};

use num_traits::zero;

// 定义项目结构体
#[derive(Debug)]
#[derive(Clone)]
pub struct Item<T> {
    pub number: usize,
    pub dimensions: Vec<T>,
}

impl<T: for<'a> std::iter::Product<&'a T> + for<'a> std::iter::Sum<&'a T>> Item<T> {
    pub fn dimensions_product(&self) -> T {
        self.dimensions.iter().product()
    }

    pub fn dimensions_sum(&self) -> T {
        self.dimensions.iter().sum()
    }
}

#[derive(Debug)]
pub struct Bin<T> {
    capacity: Vec<T>,
    current_load: Vec<T>,
}

impl<T>  Bin<T>
where
T: Copy + Clone + num_traits::NumAssign + std::cmp::PartialOrd + TryInto<f64>
{
    pub fn new(capacity: Vec<T>) -> Bin<T> {
        Bin {
            capacity: capacity.clone(),
            current_load: vec![zero(); capacity.len()],
        }
    }

    pub fn can_fit(&self, item: &Item<T>) -> bool {
        for (i, &dim) in item.dimensions.iter().enumerate() {
            if self.current_load[i] + dim > self.capacity[i] {
                return false;
            }
        }
        true
    }

    pub fn add_item(&mut self, item: &Item<T>) {
        for (i, &dim) in item.dimensions.iter().enumerate() {
            self.current_load[i] += dim;
        }
    }

    pub fn current_remaining(&self) -> Vec<T> {
        self.capacity.iter().zip(self.current_load.iter()).map(|(a, b)| a.clone() - b.clone()).collect()
    }

    pub fn max_load(&self) -> f64 {
        self.current_load.iter().zip(self.capacity.iter()).map(|(a, b)| {
            let a: f64 = match a.clone().try_into() {
                Ok(n) => n,
                Err(e) => panic!("try_into<f64> error")
            };
            let b: f64 = match b.clone().try_into() {
                Ok(n) => n,
                Err(e) => panic!("try_into<f64> error")
            };
            a / b
        }).fold(0.0, f64::max)
    }
}

pub struct BinPacking<T> {
    pub dimension_num: u32,
    pub items: Vec<Item<T>>,
    pub bin_capacity: Vec<T>,
}

impl<T: FromStr> BinPacking<T>
where <T as FromStr>::Err: Debug
{
    pub fn new(dimension_num: u32, items: Vec<Item<T>>, bin_capacity: Vec<T>) -> BinPacking<T> {
        BinPacking { dimension_num, items, bin_capacity }
    }

    pub fn load_from_file(file_path: &str) -> BinPacking<T> {
        let file = std::fs::read_to_string(file_path).expect("Failed to read file");
        let mut lines = file.lines();
        let dimension_num: u32 = lines.next().unwrap().parse().unwrap();
        let bin_capacity: Vec<T> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let items: Vec<Item<T>> = lines.enumerate().map(|(i, line)| {
            let dimensions: Vec<T> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            Item { number: i, dimensions }
        }).collect();
        assert!(items.iter().all(|item| item.dimensions.len() == bin_capacity.len()));
        BinPacking { dimension_num, items, bin_capacity }
    }
}
