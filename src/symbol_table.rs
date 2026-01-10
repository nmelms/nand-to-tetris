use std::collections::HashMap;

pub struct SymbolTable {
    pub table: HashMap<String, u32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        for i in 0..=15 {
            table.insert(format!("R{}", i), i);
        }
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);

        Self { table }
    }

    pub fn add_entry(&mut self, symbol: String, address: u32) -> () {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, value: &str) -> bool {
        match self.table.get(value) {
            Some(_value) => true,
            None => false,
        }
    }

    pub fn get_address(&self, value: &str) -> u32 {
        *self.table.get(value).expect("symbol not found in table ")
    }
}
