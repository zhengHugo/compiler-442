use crate::semantic::concept::{CompositeConcept, Concept};
use crate::semantic::semantic_error::{SemanticErrType, SemanticError};
use crate::semantic::symbol_table::{SymbolKind, SymbolTable, SymbolTableEntry, SymbolType};
use crate::syntactic::tree::{NodeId, Tree};
use std::collections::HashMap;

pub type AbstractSyntaxTree = Tree<Concept>;

pub fn generate_symbol_table(ast: &AbstractSyntaxTree) -> HashMap<String, SymbolTable> {
    let mut table_container = HashMap::new();
    let root = ast.get_root();
    create_table(ast, root, &mut table_container, "".to_string());
    table_container
}

pub fn create_table(
    ast: &AbstractSyntaxTree,
    node: NodeId,
    table_container: &mut HashMap<String, SymbolTable>,
    name_prefix: String,
) -> String {
    let concept = ast.get_node_value(node);
    match concept {
        Concept::AtomicConcept(_) => "".to_string(),
        Concept::CompositeConcept(cc) => match cc {
            // CompositeConcept::Dot => {}
            // CompositeConcept::IndexList => {}
            // CompositeConcept::Var => {}
            // CompositeConcept::Assign => {}
            // CompositeConcept::FuncCall => {}
            // CompositeConcept::RelExpr => {}
            // CompositeConcept::AddExpr => {}
            // CompositeConcept::MultExpr => {}
            // CompositeConcept::NotExpr => {}
            // CompositeConcept::SignedExpr => {}
            // CompositeConcept::IfThenElse => {}
            // CompositeConcept::Read => {}
            // CompositeConcept::Return => {}
            // CompositeConcept::While => {}
            // CompositeConcept::StmtBlock => {}
            // CompositeConcept::Write => {}
            // CompositeConcept::AParams => {}
            // CompositeConcept::ArraySizes => {}
            // CompositeConcept::FParam => {}
            // CompositeConcept::Type => {}
            // CompositeConcept::FParams => {}
            CompositeConcept::FuncDef => {
                let func_def_children = ast.get_children(node);
                let func_name = ast
                    .get_node_value(func_def_children[0])
                    .get_atomic_concept()
                    .get_value();
                let table_name = format!("{}:{}", name_prefix, func_name);
                let mut this_table = SymbolTable::new(table_name.clone());

                for body_stmt_node in ast.get_children(func_def_children[3]) {
                    match SymbolTableEntry::from_node(
                        body_stmt_node,
                        ast,
                        table_container,
                        table_name.clone(),
                    ) {
                        None => {}
                        Some(entry) => {
                            this_table.insert(entry);
                        }
                    }
                }
                table_container.insert(table_name.clone(), this_table);
                table_name
            }
            // CompositeConcept::VarDecl => {}
            // CompositeConcept::FuncBody => {}
            // CompositeConcept::FuncDecl => {}
            // CompositeConcept::FuncDefList => {}
            //CompositeConcept::ImplDef => {}
            CompositeConcept::StructDecl => {
                let struct_decl_children = ast.get_children(node);
                let struct_name = ast
                    .get_node_value(struct_decl_children[0])
                    .get_atomic_concept()
                    .get_value();
                let table_name = format!("{}:{}", name_prefix, struct_name);
                let mut this_table = SymbolTable::new(table_name.clone());

                // inherits as entries
                let inherit_nodes = ast.get_children(struct_decl_children[1]);
                for inherit_node in inherit_nodes {
                    this_table.insert(SymbolTableEntry {
                        name: "".to_string(),
                        kind: SymbolKind::Inherits,
                        symbol_type: SymbolType::from_node(inherit_node, ast),
                        link: Some(
                            ast.get_node_value(inherit_node)
                                .get_atomic_concept()
                                .get_value(),
                        ),
                    });
                }

                // members
                let struct_member_decl_nodes = ast.get_children(struct_decl_children[2]);
                for struct_member_decl_node in struct_member_decl_nodes {
                    match SymbolTableEntry::from_node(
                        struct_member_decl_node,
                        ast,
                        table_container,
                        table_name.clone(),
                    ) {
                        None => {}
                        Some(entry) => {
                            this_table.insert(entry);
                        }
                    }
                }
                table_container.insert(table_name.clone(), this_table);
                table_name
            }
            // CompositeConcept::InheritsList => {}
            // CompositeConcept::StructMemberDeclList => {}
            // CompositeConcept::StructMemberDecl => {}
            CompositeConcept::Prog => {
                let mut this_table = SymbolTable::new("global".to_string());
                for child in ast.get_children(node) {
                    match SymbolTableEntry::from_node(
                        child,
                        &ast,
                        table_container,
                        "global".to_string(),
                    ) {
                        None => {}
                        Some(entry) => {
                            this_table.insert(entry);
                        }
                    }
                }
                table_container.insert("global".to_string(), this_table);
                "global".to_string()
            }
            _ => panic!(""),
        },
    }
    // pre-actions
    // post-actions
}
