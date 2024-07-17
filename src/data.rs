// 定义项目结构体
#[derive(Debug)]
#[derive(Clone)]
pub struct Item {
    pub dimensions: Vec<u32>,
}

impl Item {
    pub fn dimensions_product(&self) -> u32 {
        self.dimensions.iter().product()
    }

    pub fn dimensions_sum(&self) -> u32 {
        self.dimensions.iter().sum()
    }
}

// 定义箱子结构体
#[derive(Debug)]
pub struct Bin {
    capacity: Vec<u32>,
    current_load: Vec<u32>,
}

impl Bin {
    // 创建一个新的箱子
    pub fn new(capacity: Vec<u32>) -> Bin {
        Bin {
            capacity: capacity.clone(),
            current_load: vec![0; capacity.len()],
        }
    }

    // 检查项目是否可以放入当前箱子
    pub fn can_fit(&self, item: &Item) -> bool {
        for (i, &dim) in item.dimensions.iter().enumerate() {
            if self.current_load[i] + dim > self.capacity[i] {
                return false;
            }
        }
        true
    }

    // 将项目放入当前箱子
    pub fn add_item(&mut self, item: &Item) {
        for (i, &dim) in item.dimensions.iter().enumerate() {
            self.current_load[i] += dim;
        }
    }

    pub fn current_remaining(&self) -> Vec<u32> {
        self.capacity.iter().zip(self.current_load.iter()).map(|(a, b)| a - b).collect()
    }
}

pub struct BinPacking {
    pub dimension_num: u32,
    pub items: Vec<Item>,
    pub bin_capacity: Vec<u32>,
}

impl BinPacking {
    pub fn load_from_file(file_path: &str) -> BinPacking {
        let file = std::fs::read_to_string(file_path).expect("Failed to read file");
        let mut lines = file.lines();
        let dimension_num: u32 = lines.next().unwrap().parse().unwrap();
        let bin_capacity: Vec<u32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let items: Vec<Item> = lines.map(|line| {
            let dimensions: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            Item { dimensions }
        }).collect();
        assert!(items.iter().all(|item| item.dimensions.len() == bin_capacity.len()));
        BinPacking { dimension_num, items, bin_capacity }
    }
}
