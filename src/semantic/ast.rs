use crate::semantic::concept::{AtomicConceptType, CompositeConcept, Concept};
use crate::semantic::semantic_error::{SemanticErrType, SemanticError};
use crate::semantic::symbol_table::{SymbolKind, SymbolTable, SymbolTableEntry, SymbolType};
use crate::syntactic::tree::{NodeId, Tree};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct AbstractSyntaxTree(pub(crate) Tree<Concept>);

impl AbstractSyntaxTree {
    pub fn get_root(&self) -> NodeId {
        self.0.get_root()
    }

    pub fn get_node_value(&self, node: NodeId) -> &Concept {
        self.0.get_node_value(node)
    }

    pub fn get_children(&self, parent_node: NodeId) -> Vec<NodeId> {
        self.0.get_children(parent_node)
    }

    pub fn get_code_snippet_from_node(&self, node: NodeId) -> String {
        let node_concept = self.get_node_value(node);
        match node_concept {
            Concept::AtomicConcept(ac) => ac.get_value(),
            Concept::CompositeConcept(cc) => match cc {
                CompositeConcept::Dot => {
                    let dot_children = self.get_children(node);
                    format!(
                        "{}.{}",
                        self.get_code_snippet_from_node(dot_children[0]),
                        self.get_code_snippet_from_node(dot_children[1])
                    )
                }
                CompositeConcept::IndexList | CompositeConcept::ArraySizes => {
                    let index_list_children = self.get_children(node);
                    index_list_children
                        .iter()
                        .map(|x| format!("[{}]", self.get_code_snippet_from_node(x.clone())))
                        .collect::<Vec<String>>()
                        .join("")
                }
                CompositeConcept::Var => {
                    let var_children = self.get_children(node);
                    format!(
                        "{}{}",
                        self.get_code_snippet_from_node(var_children[0]),
                        self.get_code_snippet_from_node(var_children[1])
                    )
                }
                CompositeConcept::Assign => {
                    let assign_children = self.get_children(node);
                    format!(
                        "{} = {}",
                        self.get_code_snippet_from_node(assign_children[0]),
                        self.get_code_snippet_from_node(assign_children[1]),
                    )
                }
                CompositeConcept::FuncCall => {
                    let func_call_children = self.get_children(node);
                    format!(
                        "{}{}",
                        self.get_code_snippet_from_node(func_call_children[0]),
                        self.get_code_snippet_from_node(func_call_children[1]),
                    )
                }
                CompositeConcept::RelExpr
                | CompositeConcept::AddExpr
                | CompositeConcept::MultExpr => {
                    let expr_children = self.get_children(node);
                    format!(
                        "{} {} {}",
                        self.get_code_snippet_from_node(expr_children[0]),
                        self.get_code_snippet_from_node(expr_children[1]),
                        self.get_code_snippet_from_node(expr_children[2]),
                    )
                }
                CompositeConcept::NotExpr | CompositeConcept::SignedExpr => {
                    let expr_children = self.get_children(node);
                    format!(
                        "{}{}",
                        self.get_code_snippet_from_node(expr_children[0]),
                        self.get_code_snippet_from_node(expr_children[1])
                    )
                }
                CompositeConcept::AParams | CompositeConcept::FParams => {
                    let a_params_children = self.get_children(node);
                    let mut result = String::new();
                    for child in a_params_children {
                        result.push_str(&self.get_code_snippet_from_node(child));
                        result.push_str(", ");
                    }
                    result.pop();
                    format!("({})", result)
                }
                CompositeConcept::FParam => {
                    let f_param_children = self.get_children(node);
                    format!(
                        "{}: {}{}",
                        self.get_code_snippet_from_node(f_param_children[0]),
                        self.get_code_snippet_from_node(f_param_children[1]),
                        self.get_code_snippet_from_node(f_param_children[2]),
                    )
                }
                CompositeConcept::Type => {
                    let type_children = self.get_children(node);
                    format!("{}", self.get_code_snippet_from_node(type_children[0]))
                }
                _ => panic!("Unhandled get_code_snippet_from_node of type {}", cc),
            },
        }
    }

    pub fn generate_symbol_tables(&self) -> HashMap<String, SymbolTable> {
        let mut table_container = HashMap::new();
        let root = self.get_root();
        self.create_table(root, &mut table_container, "".to_string());
        check_func_def(&table_container);
        self.refer_type_on_node(root, "global", &table_container);
        table_container
    }

    pub fn create_table(
        &self,
        node: NodeId,
        table_container: &mut HashMap<String, SymbolTable>,
        name_prefix: String,
    ) -> String {
        let concept = self.get_node_value(node);
        match concept {
            Concept::AtomicConcept(_) => "".to_string(),
            Concept::CompositeConcept(cc) => match cc {
                CompositeConcept::FuncDef => {
                    let func_def_children = self.get_children(node);
                    let func_name = self
                        .get_node_value(func_def_children[0])
                        .get_atomic_concept()
                        .get_value();
                    let table_name = format!("{}:{}", name_prefix, func_name);
                    let mut this_table = SymbolTable::new(table_name.clone());

                    // insert entries of params
                    for f_param in self.get_children(func_def_children[1]) {
                        if let Some(entry) = SymbolTableEntry::from_node(
                            f_param,
                            self,
                            table_container,
                            table_name.clone(),
                        ) {
                            this_table.insert(entry);
                        }
                    }

                    for body_stmt_node in self.get_children(func_def_children[3]) {
                        if let Some(entry) = SymbolTableEntry::from_node(
                            body_stmt_node,
                            self,
                            table_container,
                            table_name.clone(),
                        ) {
                            this_table.insert(entry);
                        }
                    }
                    table_container.insert(table_name.clone(), this_table);
                    table_name
                }
                CompositeConcept::StructDecl => {
                    let struct_decl_children = self.get_children(node);
                    let struct_name = self
                        .get_node_value(struct_decl_children[0])
                        .get_atomic_concept()
                        .get_value();
                    let table_name = format!("{}:{}", name_prefix, struct_name);
                    let mut this_table = SymbolTable::new(table_name.clone());

                    // inherits as entries
                    let inherit_nodes = self.get_children(struct_decl_children[1]);
                    for inherit_node in inherit_nodes {
                        let inherited_table_name = format!(
                            "{}:{}",
                            "global",
                            self.get_node_value(inherit_node)
                                .get_atomic_concept()
                                .get_value()
                        );
                        if table_container.contains_key(&inherited_table_name) {
                            this_table.insert(SymbolTableEntry {
                                name: "".to_string(),
                                kind: SymbolKind::Inherits,
                                symbol_type: SymbolType::from_node(inherit_node, self),
                                link: Some(inherited_table_name.clone()),
                            });
                        } else {
                            SemanticError::report_error(&format!(
                                "inherited class {} doesn't exist",
                                inherited_table_name
                            ));
                        }
                    }

                    // members
                    let struct_member_decl_nodes = self.get_children(struct_decl_children[2]);
                    for struct_member_decl_node in struct_member_decl_nodes {
                        match SymbolTableEntry::from_node(
                            struct_member_decl_node,
                            self,
                            table_container,
                            table_name.clone(),
                        ) {
                            None => {}
                            Some(entry) => {
                                let inherit_entries =
                                    this_table.get_all_entries_by_kind(SymbolKind::Inherits);
                                let inherit_table_names = inherit_entries
                                    .iter()
                                    .map(|entry| entry.link.as_ref().unwrap())
                                    .collect::<Vec<&String>>();
                                for inherit_table_name in inherit_table_names {
                                    if search_inherited_class_from_member(
                                        &entry.name,
                                        inherit_table_name,
                                        table_container,
                                    )
                                    .is_some()
                                    {
                                        SemanticError::report(
                                            SemanticErrType::Warning,
                                            &format!("Overriding member {}", &entry.name),
                                        );
                                    }
                                }
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
                    for child in self.get_children(node) {
                        match SymbolTableEntry::from_node(
                            child,
                            &self,
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
    }

    pub fn refer_type_on_node(
        &self,
        node: NodeId,
        scope: &str,
        table_container: &HashMap<String, SymbolTable>,
    ) -> Result<String, SemanticError> {
        let concept = self.get_node_value(node);
        let table = table_container.get(scope).unwrap();

        match concept {
            Concept::AtomicConcept(ac) => match ac.atomic_concept_type {
                AtomicConceptType::Id => {
                    let entry_option = table.get_entry_by_name(&ac.get_value());
                    match entry_option {
                        Some(entry) => Ok(entry.symbol_type.get_name()),
                        None => {
                            if scope.eq("global") {
                                // already searching in the global scope and still not found. Error
                                return Err(SemanticError::report_error(&format!(
                                    "{} referred is undeclared",
                                    ac.get_value()
                                )));
                            } else {
                                let mut scope_split_vec = scope.split(":").collect::<Vec<&str>>();
                                if let Some((_, parent_scope)) = scope_split_vec.split_last() {
                                    return self.refer_type_on_node(
                                        node,
                                        &parent_scope.join(":"),
                                        table_container,
                                    );
                                } else {
                                    panic!("Unexpected scope string");
                                }
                            }
                        }
                    }
                }
                AtomicConceptType::FloatLit => Ok("float".to_string()),
                AtomicConceptType::Float => Ok("float".to_string()),
                AtomicConceptType::IntLit => Ok("integer".to_string()),
                AtomicConceptType::Integer => Ok("integer".to_string()),
                AtomicConceptType::Void => Ok("void".to_string()),
                //AtomicConceptType::RelOp => {}
                // AtomicConceptType::MultiOp => {}
                // AtomicConceptType::AddOp => {}
                // AtomicConceptType::Sign => {}
                // AtomicConceptType::Visibility => {}
                // AtomicConceptType::Epsilon => {}
                _ => Ok("".to_string()),
            },
            Concept::CompositeConcept(cc) => match cc {
                CompositeConcept::Dot => {
                    let dot_children = self.get_children(node);
                    let left_side_type =
                        self.refer_type_on_node(dot_children[0], scope, table_container)?;
                    let global_table = table_container.get("global").unwrap();
                    let left_side_entry_option = global_table.get_entry_by_name(&left_side_type);
                    // check caller is a defined class
                    match left_side_entry_option {
                        None => Err(SemanticError::report_error(&format!(
                            "Type of caller of a \".\" operator should be a class. {} is found",
                            &left_side_type
                        ))),
                        Some(left_side_entry) => {
                            let left_side_table = table_container
                                .get(&left_side_entry.link.clone().unwrap())
                                .unwrap();
                            let right_side_name = self
                                .get_node_value(dot_children[1])
                                .get_atomic_concept()
                                .get_value();
                            // check callee is in a table
                            let target_table_option = search_inherited_class_from_member(
                                &right_side_name,
                                &left_side_table.get_table_name(),
                                table_container,
                            );
                            match target_table_option {
                                None => Err(SemanticError::report_error(&format!(
                                    "{} is not a member of {} or its super classes",
                                    right_side_name, left_side_type
                                ))),
                                Some(target_table_name) => Ok(table_container
                                    .get(&target_table_name)
                                    .unwrap()
                                    .get_entry_by_name(&right_side_name)
                                    .unwrap()
                                    .symbol_type
                                    .get_name()),
                            }
                        }
                    }
                }
                CompositeConcept::IndexList => {
                    let indices = self.get_children(node);
                    if indices.len() == 0 {
                        return Ok("".to_string());
                    }
                    let mut return_type = String::from("");
                    for index in indices {
                        let index_type = self.refer_type_on_node(index, scope, table_container)?;
                        if !index_type.eq("integer") {
                            return Err(SemanticError::report_error(&format!(
                                "array index should be integer, but {} is found",
                                index_type
                            )));
                        }
                        return_type.push_str("[");
                        return_type.push_str(&index_type);
                        return_type.push_str("]");
                    }
                    Ok(return_type)
                }
                CompositeConcept::Var => {
                    let var_children = self.get_children(node);
                    let left_type =
                        self.refer_type_on_node(var_children[0], scope, table_container)?;
                    let right_type =
                        self.refer_type_on_node(var_children[1], scope, table_container)?;
                    if right_type.eq("") {
                        return Ok(left_type);
                    }

                    // check index number should match
                    let left_index_split = left_type.split("[").collect::<Vec<&str>>();
                    let right_index_split = right_type.split("[").collect::<Vec<&str>>();
                    return if left_index_split.len() == right_index_split.len() {
                        Ok(left_index_split[0].to_string())
                    } else {
                        Err(SemanticError::report_error(&format!(
                            "array index call on variable of type {} does not match",
                            left_type
                        )))
                    };
                }
                CompositeConcept::Assign => {
                    let assign_children = self.get_children(node);
                    let left_type =
                        self.refer_type_on_node(assign_children[0], scope, table_container)?;
                    let right_type =
                        self.refer_type_on_node(assign_children[1], scope, table_container)?;
                    if !left_type.eq(&right_type) {
                        SemanticError::report_error(
                            &format!("Left and right hand side of assignment operator have different types: {} vs. {}",left_type, right_type)
                        );
                    }
                    Ok("".to_string())
                }
                CompositeConcept::FuncCall => {
                    // get entry from callee and verify params
                    let func_call_children = self.get_children(node);
                    let caller_type =
                        self.refer_type_on_node(func_call_children[0], scope, table_container)?;
                    let function_snippet = self.get_code_snippet_from_node(func_call_children[0]);
                    if !caller_type.contains(":") {
                        return Err(SemanticError::report_error(&format!(
                            "{} is not a function",
                            &function_snippet,
                        )));
                    }
                    let caller_type_vec: Vec<&str> = caller_type.split(":").collect();
                    let return_type = caller_type_vec[0];
                    let params_type =
                        self.refer_type_on_node(func_call_children[1], scope, table_container)?;
                    if params_type.eq(caller_type_vec[1]) {
                        Ok(return_type.to_string())
                    } else {
                        Err(SemanticError::report_error(&format!(
                            "function {} should be called on parameter {}. Parameter {} is found",
                            &function_snippet, caller_type_vec[1], params_type
                        )))
                    }
                }
                CompositeConcept::RelExpr => {
                    let rel_expr_children = self.get_children(node);
                    let operator = self
                        .get_node_value(rel_expr_children[1])
                        .get_atomic_concept()
                        .get_value();
                    let left_operand_type =
                        self.refer_type_on_node(rel_expr_children[0], scope, table_container)?;
                    let right_operand_type =
                        self.refer_type_on_node(rel_expr_children[2], scope, table_container)?;
                    if !left_operand_type.eq("float") && !left_operand_type.eq("integer") {
                        return Err(SemanticError::report_error(&format!(
                            "real operator applied on {left_operand_type}, which is not a number"
                        )));
                    }
                    if !right_operand_type.eq("float") && !right_operand_type.eq("integer") {
                        return Err(SemanticError::report_error(&format!(
                            "real operator applied on {right_operand_type}, which is not a number"
                        )));
                    }
                    if !left_operand_type.eq(&right_operand_type) {
                        return Err(SemanticError::report_error(&format!(
                            "two operands of operator {} have different types",
                            operator
                        )));
                    }
                    Ok("bool".to_string())
                }
                CompositeConcept::AddExpr => {
                    let add_expr_children = self.get_children(node);
                    let operator = self
                        .get_node_value(add_expr_children[1])
                        .get_atomic_concept()
                        .get_value();
                    let left_operand_type =
                        self.refer_type_on_node(add_expr_children[0], scope, table_container)?;
                    let right_operand_type =
                        self.refer_type_on_node(add_expr_children[2], scope, table_container)?;
                    match operator.as_str() {
                        "+" | "-" => {
                            if !left_operand_type.eq("float") && !left_operand_type.eq("integer") {
                                return Err(SemanticError::report_error(&format!(
                                    "add operator applied on {left_operand_type}, which is not a number"
                                )));
                            }
                            if !right_operand_type.eq("float") && !right_operand_type.eq("integer")
                            {
                                return Err(SemanticError::report_error(&format!(
                                    "add operator applied on {right_operand_type}, which is not a number"
                                )));
                            }
                            // if either one operand is float, return float
                            return if left_operand_type.eq("float")
                                && right_operand_type.eq("float")
                            {
                                Ok("float".to_string())
                            } else if left_operand_type.eq("integer")
                                && right_operand_type.eq("integer")
                            {
                                Ok("integer".to_string())
                            } else {
                                Err(SemanticError::report_error(&format!(
                                    "Two operands of operator {} have different types",
                                    operator
                                )))
                            };
                        }
                        "or" => {
                            return if left_operand_type.eq("bool")
                                && left_operand_type.eq(&right_operand_type)
                            {
                                Ok("bool".to_string())
                            } else {
                                Err(SemanticError::report_error(
                                    "\"or\" can only be applied on bool",
                                ))
                            }
                        }
                        _ => panic!("Unexpected add operator {operator}"),
                    }
                }
                CompositeConcept::MultExpr => {
                    let mult_expr_children = self.get_children(node);
                    let operator = self
                        .get_node_value(mult_expr_children[1])
                        .get_atomic_concept()
                        .get_value();
                    let left_operand_type =
                        self.refer_type_on_node(mult_expr_children[0], scope, table_container)?;
                    let right_operand_type =
                        self.refer_type_on_node(mult_expr_children[2], scope, table_container)?;
                    match operator.as_str() {
                        "*" | "/" => {
                            if !left_operand_type.eq("float") && !left_operand_type.eq("integer") {
                                return Err(SemanticError::report_error(&format!(
                                    "mult operator applied on {left_operand_type}, which is not a number"
                                )));
                            }
                            if !right_operand_type.eq("float") && !right_operand_type.eq("integer")
                            {
                                return Err(SemanticError::report_error(&format!(
                                    "mult operator applied on {right_operand_type}, which is not a number"
                                )));
                            }
                            // if either one operand is float, return float
                            return if left_operand_type.eq("float")
                                && right_operand_type.eq("float")
                            {
                                Ok("float".to_string())
                            } else if left_operand_type.eq("integer")
                                && right_operand_type.eq("integer")
                            {
                                Ok("integer".to_string())
                            } else {
                                Err(SemanticError::report_error(&format!(
                                    "Two operands of operator {} have different types",
                                    operator
                                )))
                            };
                        }
                        "and" => {
                            return if left_operand_type.eq("bool")
                                && left_operand_type.eq(&right_operand_type)
                            {
                                Ok("bool".to_string())
                            } else {
                                Err(SemanticError::report_error(
                                    "\"and\" can only be applied on bool",
                                ))
                            }
                        }
                        _ => panic!("Unexpected mult operator {operator}"),
                    }
                }
                CompositeConcept::NotExpr => {
                    let not_expr_children = self.get_children(node);
                    let not_expr_type =
                        self.refer_type_on_node(not_expr_children[0], scope, table_container)?;
                    if not_expr_type.eq("bool") {
                        Ok("bool".to_string())
                    } else {
                        Err(SemanticError::report_error(&format!(
                            "\"not\" can only be applied on bool, but {not_expr_type} is found."
                        )))
                    }
                }
                CompositeConcept::SignedExpr => {
                    let signed_expr_children = self.get_children(node);
                    let signed_expr_type =
                        self.refer_type_on_node(signed_expr_children[1], scope, table_container)?;
                    if signed_expr_type.eq("integer") || signed_expr_type.eq("float") {
                        Ok(signed_expr_type)
                    } else {
                        Err(SemanticError::report_error(&format!(
                            "sign can only be applied on bool, but {signed_expr_type} is found."
                        )))
                    }
                }
                CompositeConcept::IfThenElse => {
                    let if_then_else_children = self.get_children(node);
                    let condition_type =
                        self.refer_type_on_node(if_then_else_children[0], scope, table_container)?;
                    if !condition_type.eq("bool") {
                        return Err(SemanticError::report_error(&format!(
                            "If condition should be bool, but {} is found",
                            condition_type
                        )));
                    }
                    let then_type =
                        self.refer_type_on_node(if_then_else_children[1], scope, table_container)?;
                    let else_type =
                        self.refer_type_on_node(if_then_else_children[2], scope, table_container)?;
                    return if then_type.eq(&else_type) {
                        Ok(then_type)
                    } else {
                        Ok("".to_string())
                    };
                }
                CompositeConcept::While => {
                    let while_children = self.get_children(node);
                    let condition_type =
                        self.refer_type_on_node(while_children[0], scope, table_container)?;
                    if !condition_type.eq("bool") {
                        return Err(SemanticError::report_error(&format!(
                            "while condition should be bool, but {} is found",
                            condition_type
                        )));
                    }
                    self.refer_type_on_node(while_children[1], scope, table_container)
                }
                CompositeConcept::Return => {
                    let return_children = self.get_children(node);
                    let return_type =
                        self.refer_type_on_node(return_children[0], scope, table_container)?;
                    Ok(return_type)
                }
                CompositeConcept::StmtBlock => {
                    let statements = self.get_children(node);
                    let mut stmt_type: String = "".to_string();
                    for statement in statements {
                        stmt_type =
                            self.refer_type_on_node(statement.clone(), scope, table_container)?;
                    }
                    Ok(stmt_type)
                }
                CompositeConcept::AParams => {
                    let a_params = self.get_children(node);
                    let mut result = String::from("");
                    for node in a_params {
                        let node_type =
                            self.refer_type_on_node(node.clone(), scope, table_container)?;
                        result.push_str(&node_type);
                        result.push_str(",");
                    }
                    if result.len() > 0 {
                        return Ok(result[0..result.len() - 1].to_string());
                    } else {
                        return Ok("".to_string());
                    }
                }
                CompositeConcept::FuncDef => {
                    // get the table belong to this funcDef
                    let func_def_children = self.get_children(node);
                    let func_name = self
                        .get_node_value(func_def_children[0])
                        .get_atomic_concept()
                        .get_value();
                    let this_table = table_container
                        .get(&*format!("{}:{}", &scope, &func_name))
                        .expect(&format!("cannot get table {}:{}", &scope, &func_name));

                    // check statements in func body and return type matches

                    let body_return_type = self.refer_type_on_node(
                        func_def_children[3], // funcBody
                        &this_table.get_table_name(),
                        table_container,
                    )?;
                    let defined_return_type = self.refer_type_on_node(
                        func_def_children[2],
                        &this_table.get_table_name(),
                        table_container,
                    )?;
                    return if body_return_type.eq("") && !defined_return_type.eq("void") {
                        Err(SemanticError::report_error(&format!(
                            "function {} doesn't contain return statement of type {}",
                            &this_table.get_table_name(),
                            &defined_return_type
                        )))
                    } else if body_return_type.eq("integer") && defined_return_type.eq("float") {
                        // auto-cast integer to float
                        Ok("float".to_string())
                    } else if !body_return_type.eq("") && !body_return_type.eq(&defined_return_type)
                    {
                        Err(SemanticError::report_error(&format!(
                            "function {} should return {} but {} is returned",
                            &this_table.get_table_name(),
                            &defined_return_type,
                            &body_return_type
                        )))
                    } else {
                        Ok(body_return_type)
                    };
                }
                CompositeConcept::FuncBody => {
                    let func_body_children = self.get_children(node);
                    let mut return_type = String::from("");
                    for stmt in func_body_children {
                        let stmt_concept = self.get_node_value(stmt.clone());
                        if matches!(
                            stmt_concept,
                            Concept::CompositeConcept(CompositeConcept::Return)
                        ) {
                            match self.refer_type_on_node(stmt, scope, table_container) {
                                Ok(t) => return_type = t,
                                Err(e) => return Err(e),
                            }
                        } else {
                            if let Err(e) = self.refer_type_on_node(stmt, scope, table_container) {
                                return Err(e);
                            }
                        }
                    }
                    Ok(return_type)
                }
                CompositeConcept::Prog => {
                    // for each child (funcDef, implDef, structDecl), check type
                    for child in self.get_children(node) {
                        self.refer_type_on_node(child, "global", table_container)?;
                    }
                    Ok("".to_string())
                }
                CompositeConcept::Write => Ok("".to_string()),
                CompositeConcept::Read => Ok("".to_string()),
                CompositeConcept::ArraySizes => {
                    let mut array_sizes_type = String::from("");
                    for child in self.get_children(node) {
                        let int_lit_type =
                            self.refer_type_on_node(child.clone(), "global", table_container)?;
                        if !int_lit_type.eq("integer") {
                            return Err(SemanticError::report_error("array size must be integer"));
                        }
                        let int_lit_value =
                            self.get_node_value(child).get_atomic_concept().get_value();
                        array_sizes_type.push('[');
                        array_sizes_type.push_str(&int_lit_value);
                        array_sizes_type.push(']');
                    }
                    Ok(array_sizes_type)
                }
                CompositeConcept::StructDecl => {
                    let struct_children = self.get_children(node);
                    let struct_name = self
                        .get_node_value(struct_children[0])
                        .get_atomic_concept()
                        .get_value();
                    // structMemberDeclList
                    self.refer_type_on_node(
                        struct_children[2],
                        &format!("{}:{}", scope, struct_name),
                        table_container,
                    )
                }
                CompositeConcept::ImplDef => {
                    let impl_children = self.get_children(node);
                    let impl_name = self
                        .get_node_value(impl_children[0])
                        .get_atomic_concept()
                        .get_value();
                    self.refer_type_on_node(
                        impl_children[1],
                        &format!("{}:{}", scope, impl_name),
                        table_container,
                    )
                }

                CompositeConcept::FuncDefList
                | CompositeConcept::StructMemberDecl
                | CompositeConcept::StructMemberDeclList => {
                    for child in self.get_children(node) {
                        self.refer_type_on_node(child, scope, table_container)?;
                    }
                    Ok("".to_string())
                }
                CompositeConcept::FParam => Ok("".to_string()),
                CompositeConcept::Type => {
                    let type_children = self.get_children(node);
                    self.refer_type_on_node(type_children[0], scope, table_container)
                }
                CompositeConcept::FParams => Ok("".to_string()),
                CompositeConcept::VarDecl => Ok("".to_string()),
                CompositeConcept::FuncDecl => Ok("".to_string()),
                CompositeConcept::InheritsList => Ok("".to_string()),
            },
        }
    }
}

impl Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn check_func_def(table_container: &HashMap<String, SymbolTable>) {
    // check main function is defined
    let global_table = table_container.get("global").unwrap();
    if global_table.get_entry_by_name("main").is_none() {
        SemanticError::report_error(&format!("main function is not defined"));
    }
    // for each table, each function should have link
    for (table_name, table) in table_container {
        for entry in table.get_all_entries() {
            if matches!(entry.kind, SymbolKind::Function) {
                if entry.link.is_none() {
                    SemanticError::report_error(&format!(
                        "function {}:{} is declared but not defined",
                        table_name, entry.name
                    ));
                }
            }
        }
    }
}

// return the table name that contains the member
fn search_inherited_class_from_member(
    member_name: &str,
    table_name: &str,
    table_container: &HashMap<String, SymbolTable>,
) -> Option<String> {
    let table = table_container.get(table_name).unwrap();

    // not looking for a function. Search by name only
    let entries_in_table = table.get_entry_by_name(member_name);
    if entries_in_table.is_none() {
        // member not in the table. Check inherits
        let inherits_entries = table.get_all_entries_by_kind(SymbolKind::Inherits);
        for inherits in inherits_entries {
            if let Some(target_table_name) = search_inherited_class_from_member(
                member_name,
                &inherits.link.unwrap(),
                table_container,
            ) {
                return Some(target_table_name);
            }
        }
        None
    } else {
        // entry is directly in the table
        Some(table_name.to_string())
    }
}
