use crate::semantic::ast::{create_table, AbstractSyntaxTree};
use crate::semantic::concept::{AtomicConcept, AtomicConceptType, CompositeConcept, Concept};
use crate::syntactic::tree::NodeId;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SymbolTable {
    name: String,
    entries: HashMap<String, SymbolTableEntry>,
}

impl SymbolTable {
    pub fn new(name: String) -> Self {
        Self {
            name,
            entries: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_entry(&self, entry_name: String) -> Option<&SymbolTableEntry> {
        self.entries.get(&*entry_name)
    }

    pub fn insert(&mut self, entry: SymbolTableEntry) -> Option<SymbolTableEntry> {
        self.entries.insert(entry.name.clone(), entry)
    }
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.name);
        for (_, entry) in self.entries.iter() {
            write!(f, "{}\n", entry);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SymbolTableEntry {
    pub name: String,
    pub kind: SymbolKind,
    pub symbol_type: SymbolType,
    pub link: Option<String>,
}

impl SymbolTableEntry {
    pub fn from_node(
        node: NodeId,
        ast: &AbstractSyntaxTree,
        table_container: &mut HashMap<String, SymbolTable>,
        name_prefix: String,
    ) -> Option<Self> {
        let concept = ast.get_node_value(node);
        match concept.get_composite_concept() {
            CompositeConcept::FuncDef => {
                let func_def_elements = ast.get_children(node);

                let name = ast
                    .get_node_value(func_def_elements[0])
                    .get_atomic_concept()
                    .get_value();

                Some(SymbolTableEntry {
                    name,
                    kind: SymbolKind::Function,
                    symbol_type: SymbolType::from_node(node, ast), // fParams type
                    link: Some(create_table(ast, node, table_container, name_prefix)), // funcDef table
                })
            }
            CompositeConcept::FuncDecl => {
                let func_decl_elements = ast.get_children(node);
                let name = ast
                    .get_node_value(func_decl_elements[0])
                    .get_atomic_concept()
                    .get_value();

                Some(SymbolTableEntry {
                    name,
                    kind: SymbolKind::Function,
                    symbol_type: SymbolType::from_node(node, ast),
                    link: None,
                })
            }
            CompositeConcept::StructDecl => {
                let struct_decl_elements = ast.get_children(node);
                let name = ast
                    .get_node_value(struct_decl_elements[0])
                    .get_atomic_concept()
                    .get_value();

                Some(SymbolTableEntry {
                    name,
                    kind: SymbolKind::Class,
                    symbol_type: SymbolType {
                        name: "".to_string(),
                    },
                    link: Some(create_table(ast, node, table_container, name_prefix)),
                })
            }
            CompositeConcept::VarDecl => {
                let var_decl_elements = ast.get_children(node);
                let name = ast
                    .get_node_value(var_decl_elements[0])
                    .get_atomic_concept()
                    .get_value();
                Some(SymbolTableEntry {
                    name,
                    kind: SymbolKind::Variable,
                    symbol_type: SymbolType::from_node(node, ast),
                    link: None,
                })
            }
            CompositeConcept::StructMemberDecl => {
                let struct_member_decl_elements = ast.get_children(node);
                Self::from_node(
                    struct_member_decl_elements[1], // funcDecl or varDecl
                    ast,
                    table_container,
                    name_prefix,
                )
            }
            _ => None,
        }
    }
}

impl Display for SymbolTableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {:?}, {}, {:?}",
            self.name, self.kind, self.symbol_type, self.link
        )
    }
}

#[derive(Debug)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
    Inherits,
    Class,
}

#[derive(Debug)]
pub struct SymbolType {
    name: String,
}

impl SymbolType {
    pub fn from_node(node: NodeId, ast: &AbstractSyntaxTree) -> Self {
        let concept = ast.get_node_value(node);
        let name = match concept {
            Concept::AtomicConcept(ac) => match ac.atomic_concept_type {
                AtomicConceptType::Id
                | AtomicConceptType::FloatLit
                | AtomicConceptType::Integer
                | AtomicConceptType::Void
                | AtomicConceptType::RelOp
                | AtomicConceptType::MultiOp
                | AtomicConceptType::AddOp
                | AtomicConceptType::Sign
                | AtomicConceptType::IntLit
                | AtomicConceptType::Float => ac.get_value(),
                _ => panic!("Error in getting atomic concept symbol type"),
            },
            Concept::CompositeConcept(cc) => match cc {
                // CompositeConcept::IndexList => {}
                CompositeConcept::ArraySizes => {
                    let array_sizes_elements = ast.get_children(node);
                    array_sizes_elements
                        .iter()
                        .map(|x| SymbolType::from_node(*x, ast).name)
                        .map(|x| format!("[{}]", x))
                        .collect::<Vec<String>>()
                        .join("")
                }
                // CompositeConcept::Var => {}
                // CompositeConcept::RelExpr => {}
                // CompositeConcept::AddExpr => {}
                // CompositeConcept::MultExpr => {}
                // CompositeConcept::NotExpr => {}
                // CompositeConcept::SignedExpr => {}
                // CompositeConcept::Return => {}
                CompositeConcept::FParam => {
                    let f_param_elements = ast.get_children(node);
                    // get second node type text: type
                    let type_symbol_type = SymbolType::from_node(f_param_elements[1], ast);
                    let array_sizes_symbol_type = SymbolType::from_node(f_param_elements[2], ast);
                    format!("{}{}", type_symbol_type.name, array_sizes_symbol_type.name)
                }
                CompositeConcept::FParams => {
                    let f_params_elements = ast.get_children(node);
                    f_params_elements
                        .iter()
                        .map(|x| SymbolType::from_node(*x, ast).name)
                        .collect::<Vec<String>>()
                        .join(",")
                }
                CompositeConcept::Type => {
                    let type_children = ast.get_children(node);
                    let atomic_concept = ast.get_node_value(type_children[0]).get_atomic_concept();
                    match atomic_concept.atomic_concept_type {
                        AtomicConceptType::Id => atomic_concept.get_value(),
                        AtomicConceptType::Float => "float".to_string(),
                        AtomicConceptType::Integer => "integer".to_string(),
                        _ => panic!("Error in getting symbol type from Type"),
                    }
                }
                CompositeConcept::VarDecl => {
                    let var_decl_elements = ast.get_children(node);
                    let type_symbol_type = SymbolType::from_node(var_decl_elements[1], ast);
                    let array_sizes_symbol_type = SymbolType::from_node(var_decl_elements[2], ast);
                    format!("{}{}", type_symbol_type.name, array_sizes_symbol_type.name)
                }
                CompositeConcept::FuncDecl | CompositeConcept::FuncDef => {
                    let func_def_elements = ast.get_children(node);
                    let f_params = SymbolType::from_node(func_def_elements[1], ast);
                    let return_type = SymbolType::from_node(func_def_elements[2], ast);
                    format!("{}:{}", return_type.name, f_params.name)
                }
                CompositeConcept::ImplDef => "".to_string(),
                // CompositeConcept::StructDecl => {}
                // CompositeConcept::StructMemberDecl => {}
                cc => panic!("Unhandled cc: {}", cc),
            },
        };
        Self { name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Display for SymbolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
