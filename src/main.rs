#[derive(Clone)]
struct Warehouse {
    parts: Vec<String>,
}

impl Warehouse {
    fn new() -> Warehouse {
        Warehouse { parts: Vec::new() }
    }

    fn sell_parts(&self) {
        for part in &self.parts {
            println!("sell: {}", part);
        }
    }
}

trait Builder {
    fn fill_warehouse_part_a(&mut self);
    fn fill_warehouse_part_b(&mut self);
    fn name(&mut self) -> String;
    fn get_warehouse(&mut self) -> Warehouse;
}

struct HumanBuilder {
    warehouse: Warehouse,
}

impl HumanBuilder {
    fn new() -> HumanBuilder {
        HumanBuilder {
            warehouse: Warehouse::new(),
        }
    }
}

struct MachineBuilder {
    warehouse: Warehouse,
}

impl MachineBuilder {
    fn new() -> Self {
        MachineBuilder {
            warehouse: Warehouse::new(),
        }
    }
}

impl Builder for MachineBuilder {
    fn fill_warehouse_part_a(&mut self) {
        self.warehouse
            .parts
            .push("machine product a part".to_string())
    }

    fn fill_warehouse_part_b(&mut self) {
        self.warehouse
            .parts
            .push("machine product b part".to_string())
    }

    fn name(&mut self) -> String {
        "Machine".to_string()
    }

    fn get_warehouse(&mut self) -> Warehouse {
        let warehouse = self.warehouse.clone();
        self.warehouse = Warehouse::new();
        warehouse
    }
}

impl Builder for HumanBuilder {
    fn fill_warehouse_part_a(&mut self) {
        self.warehouse
            .parts
            .push("human product a part".to_string())
    }

    fn fill_warehouse_part_b(&mut self) {
        self.warehouse
            .parts
            .push("human product b part".to_string())
    }

    fn name(&mut self) -> String {
        "Human".to_string()
    }

    fn get_warehouse(&mut self) -> Warehouse {
        let warehouse = self.warehouse.clone();
        self.warehouse = Warehouse::new();
        warehouse
    }
}

struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    fn new(builder: Box<dyn Builder>) -> Director {
        Director { builder }
    }

    fn fill_warehouse(&mut self) {
        self.builder.fill_warehouse_part_a();
        self.builder.fill_warehouse_part_b();
    }

    fn get_which_warehouse(&mut self) {
        println!("get {} type warehouse", self.builder.name())
    }

    fn get_warehouse(&mut self) -> Warehouse {
        self.builder.get_warehouse()
    }
}

fn main() {
    // 创建
    let human_builder = HumanBuilder::new();
    // 将 builder 装入 Director
    let mut director = Director::new(Box::new(human_builder));
    // Director 进行指挥工作
    // 1: 生产产品装入 warehouse
    director.fill_warehouse();
    // 2: 获取装满通知
    director.get_which_warehouse();
    // 3： 获取 warehouse
    let warehouse = director.get_warehouse();
    // 清空 warehouse 产品
    warehouse.sell_parts();

    // 创建
    let machine_builder = MachineBuilder::new();
    // 将 builder 装入 Director
    let director = Director::new(Box::new(machine_builder));
    work(director)
}

fn work(mut director: Director) {
    // Director 进行指挥工作
    // 1: 生产产品装入 warehouse
    director.fill_warehouse();
    // 2: 获取装满通知
    director.get_which_warehouse();
    // 3： 获取 warehouse
    let warehouse = director.get_warehouse();
    // 清空 warehouse 产品
    warehouse.sell_parts();
}
