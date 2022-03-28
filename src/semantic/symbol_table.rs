use crate::semantic::ast::{create_table, AbstractSyntaxTree};
use crate::semantic::concept::{AtomicConceptType, CompositeConcept, Concept};
use crate::semantic::semantic_error::{SemanticErrType, SemanticError};
use crate::syntactic::tree::NodeId;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SymbolTable {
    name: String,
    entries: HashMap<(String, SymbolType), SymbolTableEntry>,
}

impl SymbolTable {
    pub fn new(name: String) -> Self {
        Self {
            name,
            entries: HashMap::new(),
        }
    }

    pub fn get_table_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_all_entries(&self) -> Vec<SymbolTableEntry> {
        self.entries.values().cloned().collect()
    }

    pub fn get_all_entries_by_name(&self, entry_name: &str) -> Vec<SymbolTableEntry> {
        self.entries
            .values()
            .cloned()
            .filter(|entry| entry.name.eq(entry_name))
            .collect::<Vec<SymbolTableEntry>>()
    }

    pub fn get_entry_by_name_and_type(
        &self,
        entry_name: &str,
        entry_type: &str,
    ) -> Option<&SymbolTableEntry> {
        self.entries.get(&(
            entry_name.to_string(),
            SymbolType {
                name: entry_type.to_string(),
            },
        ))
    }

    pub fn get_all_entries_by_kind(&self, symbol_kind: SymbolKind) -> Vec<SymbolTableEntry> {
        self.entries
            .values()
            .cloned()
            .filter(|entry| entry.kind.eq(&symbol_kind))
            .collect::<Vec<SymbolTableEntry>>()
    }

    pub fn insert(&mut self, entry: SymbolTableEntry) -> Option<SymbolTableEntry> {
        let key = (entry.name.clone(), entry.symbol_type.clone());
        if matches!(entry.kind, SymbolKind::Function) {
            // to insert function
            if self.entries.contains_key(&key) {
                if self.entries.get(&key).unwrap().link.is_none() {
                    // existing entry has no link: implementing
                    self.entries.insert(key, entry)
                } else {
                    // existing entry has link: duplicate definition
                    SemanticError::report_error(&format!(
                        "function {} of the same type is already defined.",
                        &entry.name
                    ));
                    None
                }
            } else {
                if entry.link.is_none() {
                    // no existing entry and new entry has no link: function decl
                    if self.entries.keys().any(|key| key.0.eq(&entry.name)) {
                        SemanticError::report(
                            SemanticErrType::Warning,
                            &format!("function {} is overloaded", &entry.name),
                        );
                    }
                    self.entries.insert(key, entry)
                } else {
                    return if self.name.eq("global") {
                        self.entries.insert(key, entry)
                    } else {
                        // no existing entry and new entry has link: impl without decl
                        SemanticError::report_error(&format!(
                            "definition provided for undeclared function {}. ",
                            &entry.name
                        ));
                        None
                    };
                }
            }
        } else {
            // insert entries other than function
            if self.entries.keys().any(|key| key.0.eq(&entry.name)) {
                // name is already in the table: duplicate definition
                SemanticError::report_error(&format!("{} is already defined. ", &entry.name));
                None
            } else {
                // name is new, then key must be new
                self.entries.insert(key, entry)
            }
        }
    }
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "=========================================================\n"
        );
        write!(f, "{}\n", self.name);
        write!(
            f,
            "---------------------------------------------------------\n"
        );
        write!(
            f,
            "{0: <14} | {1: <14} | {2: <20} | {3: <14}\n",
            "name", "kind", "type", "link"
        );
        write!(
            f,
            "---------------------------------------------------------\n"
        );
        for (_, entry) in self.entries.iter() {
            write!(f, "{}\n", entry);
        }
        write!(
            f,
            "=========================================================\n"
        );
        Ok(())
    }
}

#[derive(Debug, Clone)]
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
                    name: name.clone(),
                    kind: SymbolKind::Class,
                    symbol_type: SymbolType { name },
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
            CompositeConcept::ImplDef => {
                let impl_def_children = ast.get_children(node);
                let target_struct_name = ast
                    .get_node_value(impl_def_children[0])
                    .get_atomic_concept()
                    .get_value();
                let table_name = format!("{}:{}", name_prefix, target_struct_name);

                let mut new_entry_set = vec![];
                for func_def_node in ast.get_children(impl_def_children[1]) {
                    let new_entry = match SymbolTableEntry::from_node(
                        func_def_node,
                        ast,
                        table_container,
                        table_name.clone(),
                    ) {
                        None => {
                            panic!("Something other than funcDef in impl")
                        }
                        Some(entry) => entry,
                    };
                    new_entry_set.push(new_entry);
                }

                match table_container.get_mut(&*table_name) {
                    None => {
                        SemanticError::report_error(&format!(
                            "struct {} is undefined",
                            target_struct_name
                        ));
                    }
                    Some(table) => {
                        for new_entry in new_entry_set {
                            table.insert(new_entry);
                        }
                    }
                }
                None
            }
            CompositeConcept::FParam => {
                let param_children = ast.get_children(node);
                let name = ast
                    .get_node_value(param_children[0])
                    .get_atomic_concept()
                    .get_value();
                Some(SymbolTableEntry {
                    name,
                    kind: SymbolKind::Parameter,
                    symbol_type: SymbolType::from_node(node, ast),
                    link: None,
                })
            }
            _ => None,
        }
    }
}

impl Display for SymbolTableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0: <14} | {1: <14} | {2: <20} | {3: <14}",
            self.name,
            format!("{}", self.kind),
            format!("{}", self.symbol_type),
            match &self.link {
                None => "None",
                Some(s) => s,
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
    Inherits,
    Class,
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SymbolKind::Variable => "Variable",
                SymbolKind::Function => "Function",
                SymbolKind::Parameter => "Parameter",
                SymbolKind::Inherits => "Inherits",
                SymbolKind::Class => "Class",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolType {
    name: String,
}

impl SymbolType {
    pub fn from_node(node: NodeId, ast: &AbstractSyntaxTree) -> Self {
        let concept = ast.get_node_value(node);
        let name = match concept {
            Concept::AtomicConcept(ac) => match ac.atomic_concept_type {
                AtomicConceptType::Float | AtomicConceptType::FloatLit => "float".to_string(),
                AtomicConceptType::Integer
                | AtomicConceptType::IntLit
                | AtomicConceptType::EmptyArraySize => "integer".to_string(),
                AtomicConceptType::Id => ac.get_value(),
                AtomicConceptType::Void => "".to_string(),
                _ => panic!("Error in getting atomic concept symbol type"),
                // AtomicConceptType::Visibility => {}
                // AtomicConceptType::Epsilon => {}
                // | AtomicConceptType::RelOp
                // | AtomicConceptType::MultiOp
                // | AtomicConceptType::AddOp
                // | AtomicConceptType::Sign
            },
            Concept::CompositeConcept(cc) => match cc {
                CompositeConcept::ArraySizes => {
                    let array_sizes_elements = ast.get_children(node);
                    array_sizes_elements
                        .iter()
                        .map(|x| SymbolType::from_node(*x, ast).name)
                        .map(|x| format!("[{}]", x))
                        .collect::<Vec<String>>()
                        .join("")
                }
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
                    SymbolType::from_node(type_children[0], ast).name
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
                cc => panic!("Unhandled cc: {}", cc),
            },
        };
        Self { name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_f_params_same(&self, other: &SymbolType) -> bool {
        if !self.name.contains(":") || !other.name.contains(":") {
            panic!("Not function type")
        }
        let self_param_split = self.name.split(":").collect::<Vec<&str>>();
        let other_param_split = other.name.split(":").collect::<Vec<&str>>();
        self_param_split[1].eq(other_param_split[1])
    }
}

impl Display for SymbolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
