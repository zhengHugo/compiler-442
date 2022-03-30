use crate::semantic::symbol_table::SymbolTable;
use std::collections::HashMap;

pub struct TempVarNamePool {
    counter: u64,
}

impl TempVarNamePool {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
    pub fn get_next_name(&mut self) -> String {
        self.counter += 1;
        format!("temp{}", self.counter)
    }

    pub fn get_next_unique_name_in_table(&mut self, table: &SymbolTable) -> String {
        let mut next_name = self.get_next_name();
        while table.get_entry_by_name(&next_name).is_some() {
            next_name = self.get_next_name();
        }
        next_name
    }
}
