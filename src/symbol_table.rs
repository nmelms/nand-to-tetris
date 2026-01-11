use std::collections::HashMap;

pub struct SymbolTable {
    pub table: HashMap<String, u32>,
    pub var_index: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        // start at 15 since we inc 1 first
        let var_index = 15;
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
        table.insert("i".to_string(), 16 );

        Self { table, var_index }
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

    pub fn get_address(&mut self, value: &str) -> u32 {
        let addr = match self.table.get(value){
            Some(addr) => *addr,
            None => {
                self.var_index += 1;
                self.var_index as u32
                
            }
        };
          addr

    }
}
