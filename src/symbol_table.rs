use std::collections::HashMap;

pub struct SymbolTable {
    pub table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let table = HashMap::new();

        Self { table }
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) -> () {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, value: String) -> bool {
        match self.table.get(&value) {
            Some(_value) => true,
            None => false,
        }
    }

    pub fn get_address(&self, value: &str) -> u16{
      
           *self.table.get(value).expect("symbol not found in table ")
    }
}
