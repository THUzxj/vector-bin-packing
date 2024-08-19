// 定义项目结构体
#[derive(Debug)]
#[derive(Clone)]
pub struct Item {
    pub number: usize,
    pub dimensions: Vec<f64>,
}

impl Item {
    pub fn dimensions_product(&self) -> f64 {
        self.dimensions.iter().product()
    }

    pub fn dimensions_sum(&self) -> f64 {
        self.dimensions.iter().sum()
    }
}

#[derive(Debug)]
pub struct Bin {
    capacity: Vec<f64>,
    current_load: Vec<f64>,
}

impl Bin {
    pub fn new(capacity: Vec<f64>) -> Bin {
        Bin {
            capacity: capacity.clone(),
            current_load: vec![0.0; capacity.len()],
        }
    }

    pub fn can_fit(&self, item: &Item) -> bool {
        for (i, &dim) in item.dimensions.iter().enumerate() {
            if self.current_load[i] + dim > self.capacity[i] {
                return false;
            }
        }
        true
    }

    pub fn add_item(&mut self, item: &Item) {
        for (i, &dim) in item.dimensions.iter().enumerate() {
            self.current_load[i] += dim;
        }
    }

    pub fn current_remaining(&self) -> Vec<f64> {
        self.capacity.iter().zip(self.current_load.iter()).map(|(a, b)| a - b).collect()
    }

    pub fn max_load(&self) -> f64 {
        self.current_load.iter().zip(self.capacity.iter()).map(|(a, b)| a / b).fold(0.0, f64::max)
    }
}

pub struct BinPacking {
    pub dimension_num: u32,
    pub items: Vec<Item>,
    pub bin_capacity: Vec<f64>,
}

impl BinPacking {
    pub fn new(dimension_num: u32, items: Vec<Item>, bin_capacity: Vec<f64>) -> BinPacking {
        BinPacking { dimension_num, items, bin_capacity }
    }

    pub fn load_from_file(file_path: &str) -> BinPacking {
        let file = std::fs::read_to_string(file_path).expect("Failed to read file");
        let mut lines = file.lines();
        let dimension_num: u32 = lines.next().unwrap().parse().unwrap();
        let bin_capacity: Vec<f64> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let items: Vec<Item> = lines.enumerate().map(|(i, line)| {
            let dimensions: Vec<f64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            Item { number: i, dimensions }
        }).collect();
        assert!(items.iter().all(|item| item.dimensions.len() == bin_capacity.len()));
        BinPacking { dimension_num, items, bin_capacity }
    }
}
