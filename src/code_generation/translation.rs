use crate::code_generation::register::RegisterPool;
use crate::semantic::concept::{CompositeConcept, Concept};
use crate::semantic::symbol_table::SymbolTable;
use crate::syntactic::tree::NodeId;
use crate::AbstractSyntaxTree;
use core::panicking::panic;
use std::collections::HashMap;

pub fn generate_moon_code(
    ast: &AbstractSyntaxTree,
    table_container: &HashMap<String, SymbolTable>,
) {
    let mut result_code = String::from("");
    let mut register_pool = RegisterPool::new();
    translate(ast.get_root(), ast, table_container, &mut register_pool);
}

pub fn translate(
    node: NodeId,
    ast: &AbstractSyntaxTree,
    table_container: &HashMap<String, SymbolTable>,
    register_pool: &mut RegisterPool,
) {
    let node_value = ast.get_node_value(node);
    match node_value {
        Concept::AtomicConcept(ac) => {}
        Concept::CompositeConcept(cc) => match cc {
            CompositeConcept::VarDecl => {}
            _ => panic!("Unhandled composite concept"),
        },
    }
}
