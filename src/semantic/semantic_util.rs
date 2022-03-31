use crate::semantic::symbol_table::SymbolTable;
use crate::syntactic::tree::NodeId;

pub mod var_node_utils {
    use crate::semantic::symbol_table::{SymbolTable, SymbolTableEntry};
    use crate::syntactic::tree::NodeId;
    use crate::AbstractSyntaxTree;
    use std::collections::HashMap;

    pub fn get_entry(
        node: NodeId,
        table_name: &str,
        ast: &AbstractSyntaxTree,
        table_container: &HashMap<String, SymbolTable>,
    ) -> SymbolTableEntry {
        let var_children = ast.get_children(node);
        let table = table_container.get(table_name).unwrap();
        let base_var_name = ast
            .get_node_value(var_children[0])
            .get_atomic_concept()
            .get_value();
        table.get_entry_by_name(&base_var_name).unwrap()
    }

    pub fn get_base_size(
        node: NodeId,
        table_name: &str,
        ast: &AbstractSyntaxTree,
        table_container: &HashMap<String, SymbolTable>,
    ) -> u32 {
        let var_children = ast.get_children(node);
        let table = table_container.get(table_name).unwrap();
        let base_var_name = ast
            .get_node_value(var_children[0])
            .get_atomic_concept()
            .get_value();
        let base_var_entry = get_entry(node, table_name, ast, table_container);
        let var_type = base_var_entry.symbol_type.get_type_value();
        let base_type = match var_type.find("[") {
            Some(i) => var_type.chars().take(i).collect(),
            None => var_type,
        };
        match base_type.as_str() {
            "integer" => 4,
            "float" => 8,
            _ => todo!("class size"),
        }
    }
}
