use crate::semantic::concept::{AtomicConceptType, CompositeConcept, Concept};
use crate::semantic::semantic_error::{SemanticErrType, SemanticError};
use crate::semantic::symbol_table::{SymbolKind, SymbolTable, SymbolTableEntry, SymbolType};
use crate::syntactic::tree::{NodeId, Tree};
use std::collections::HashMap;

pub type AbstractSyntaxTree = Tree<Concept>;

pub fn generate_symbol_tables(ast: &AbstractSyntaxTree) -> HashMap<String, SymbolTable> {
    let mut table_container = HashMap::new();
    let root = ast.get_root();
    create_table(ast, root, &mut table_container, "".to_string());
    check_func_def(&table_container);
    refer_type_on_node(root, ast, "global", &table_container);
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

                // insert entries of params
                for f_param in ast.get_children(func_def_children[1]) {
                    if let Some(entry) = SymbolTableEntry::from_node(
                        f_param,
                        ast,
                        table_container,
                        table_name.clone(),
                    ) {
                        this_table.insert(entry);
                    }
                }

                for body_stmt_node in ast.get_children(func_def_children[3]) {
                    if let Some(entry) = SymbolTableEntry::from_node(
                        body_stmt_node,
                        ast,
                        table_container,
                        table_name.clone(),
                    ) {
                        this_table.insert(entry);
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
                        link: Some(format!(
                            "{}:{}",
                            "global",
                            ast.get_node_value(inherit_node)
                                .get_atomic_concept()
                                .get_value()
                        )),
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
}

fn check_semantics(root: NodeId, ast: &AbstractSyntaxTree) {}

fn refer_type_on_node(
    node: NodeId,
    ast: &AbstractSyntaxTree,
    scope: &str,
    table_container: &HashMap<String, SymbolTable>,
) -> Result<String, SemanticError> {
    let concept = ast.get_node_value(node);
    let table = table_container.get(scope).unwrap();

    match concept {
        Concept::AtomicConcept(ac) => match ac.atomic_concept_type {
            AtomicConceptType::Id => {
                let entries = table.get_all_entries_by_name(&ac.get_value());
                if entries.is_empty() {
                    Err(SemanticError::report_error(&format!(
                        "{} referred is undeclared",
                        ac.get_value()
                    )))
                } else {
                    Ok(entries[0].symbol_type.get_name())
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
                let dot_children = ast.get_children(node);
                let left_side_type =
                    refer_type_on_node(dot_children[0], ast, scope, table_container)?;
                let global_table = table_container.get("global").unwrap();
                match global_table.get_entry_by_name_and_type(&left_side_type, "class") {
                    // check caller is a defined class
                    None => Err(SemanticError::report_error(&format!(
                        "Type of caller of a \".\" operator should be a class. {} is found",
                        &left_side_type
                    ))),
                    Some(class_entry) => {
                        let left_side_table = table_container
                            .get(&class_entry.link.clone().unwrap())
                            .unwrap();
                        let right_side_name = ast
                            .get_node_value(dot_children[1])
                            .get_atomic_concept()
                            .get_value();

                        // check callee is in a table
                        let target_table_option = search_inherited_class_from_member(
                            &right_side_name,
                            None,
                            &left_side_table.get_table_name(),
                            table_container,
                        );
                        match target_table_option {
                            None => Err(SemanticError::report_error(&format!(
                                "{} is not a member of {} or its super classes",
                                right_side_name, left_side_type
                            ))),
                            Some(target_table_name) => {
                                let entries = table_container
                                    .get(&target_table_name)
                                    .unwrap()
                                    .get_all_entries_by_name(&right_side_name);
                                return Ok(entries[0].symbol_type.get_name());
                            }
                        }
                    }
                }
            }
            CompositeConcept::IndexList => {
                let indices = ast.get_children(node);
                if indices.len() == 0 {
                    return Ok("".to_string());
                }
                let mut return_type = String::from("");
                for index in indices {
                    let index_type = refer_type_on_node(index, ast, scope, table_container)?;
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
                let var_children = ast.get_children(node);
                let left_type = refer_type_on_node(var_children[0], ast, scope, table_container)?;
                let right_type = refer_type_on_node(var_children[1], ast, scope, table_container)?;
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
                let assign_children = ast.get_children(node);
                let left_type =
                    refer_type_on_node(assign_children[0], ast, scope, table_container)?;
                let right_type =
                    refer_type_on_node(assign_children[1], ast, scope, table_container)?;
                if !left_type.eq(&right_type) {
                    SemanticError::report_error(
                        "Left and right hand side of assignment operator have different types",
                    );
                }
                Ok("".to_string())
            }
            CompositeConcept::FuncCall => {
                // get entry from callee and verify params
                let func_call_children = ast.get_children(node);
                if matches!(
                    ast.get_node_value(func_call_children[0]),
                    Concept::CompositeConcept(CompositeConcept::Dot)
                ) {
                    // left hand side is a dot. must check all types of this dot
                    let dot_children = ast.get_children(func_call_children[0]);
                    let dot_callee_name = ast
                        .get_node_value(dot_children[1])
                        .get_atomic_concept()
                        .get_value();
                    let dot_caller_type =
                        refer_type_on_node(dot_children[0], ast, scope, table_container)?;
                    let dot_caller_table = table_container
                        .get(&format!("global:{dot_caller_type}"))
                        .unwrap();
                    let params_type =
                        refer_type_on_node(func_call_children[1], ast, scope, table_container)?;
                    let target_table_name_option = search_inherited_class_from_member(
                        &dot_callee_name,
                        Some(params_type.clone()),
                        &dot_caller_table.get_table_name(),
                        table_container,
                    );
                    match target_table_name_option {
                        None => Err(SemanticError::report_error(&format!(
                            "function {} of parameter type {} is not found as a member of {}",
                            dot_callee_name, params_type, dot_caller_type
                        ))),
                        Some(target_table_name) => {
                            let func_entry_symbol_type = table_container
                                .get(&target_table_name)
                                .unwrap()
                                .get_all_entries_by_name(&dot_callee_name)
                                .iter()
                                .find(|entry| {
                                    entry
                                        .symbol_type
                                        .get_name()
                                        .split(":")
                                        .last()
                                        .unwrap()
                                        .eq(&params_type)
                                })
                                .unwrap()
                                .symbol_type
                                .get_name();
                            Ok(func_entry_symbol_type
                                .split(":")
                                .collect::<Vec<&str>>()
                                .get(0)
                                .unwrap()
                                .to_string())
                        }
                    }
                } else {
                    // left hand side is an id
                    let caller_name = ast
                        .get_node_value(func_call_children[0])
                        .get_atomic_concept()
                        .get_value();
                    let caller_type =
                        refer_type_on_node(func_call_children[0], ast, scope, table_container)?;
                    if !caller_type.contains(":") {
                        return Err(SemanticError::report_error(&format!(
                            "{} is not a function",
                            caller_name
                        )));
                    }
                    let caller_type_vec: Vec<&str> = caller_type.split(":").collect();
                    let params_type =
                        refer_type_on_node(func_call_children[1], ast, scope, table_container)?;
                    if params_type.eq(caller_type_vec[1]) {
                        Ok(caller_type_vec[0].to_string())
                    } else {
                        Err(SemanticError::report_error(&format!(
                            "function {} should be called on parameter {}. Parameter {} is found",
                            caller_name, caller_type_vec[1], params_type
                        )))
                    }
                }
            }
            CompositeConcept::RelExpr => {
                let rel_expr_children = ast.get_children(node);
                let operator = ast
                    .get_node_value(rel_expr_children[1])
                    .get_atomic_concept()
                    .get_value();
                let left_operand_type =
                    refer_type_on_node(rel_expr_children[0], ast, scope, table_container)?;
                let right_operand_type =
                    refer_type_on_node(rel_expr_children[2], ast, scope, table_container)?;
                if !left_operand_type.eq("float") || !left_operand_type.eq("integer") {
                    return Err(SemanticError::report_error(&format!(
                        "real operator applied on {left_operand_type}, which is not a number"
                    )));
                }
                if !right_operand_type.eq("float") || !right_operand_type.eq("integer") {
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
                let add_expr_children = ast.get_children(node);
                let operator = ast
                    .get_node_value(add_expr_children[1])
                    .get_atomic_concept()
                    .get_value();
                let left_operand_type =
                    refer_type_on_node(add_expr_children[0], ast, scope, table_container)?;
                let right_operand_type =
                    refer_type_on_node(add_expr_children[2], ast, scope, table_container)?;
                match operator.as_str() {
                    "+" | "-" => {
                        if !left_operand_type.eq("float") || !left_operand_type.eq("integer") {
                            return Err(SemanticError::report_error(&format!(
                                "real operator applied on {left_operand_type}, which is not a number"
                            )));
                        }
                        if !right_operand_type.eq("float") || !right_operand_type.eq("integer") {
                            return Err(SemanticError::report_error(&format!(
                                "real operator applied on {right_operand_type}, which is not a number"
                            )));
                        }
                        // if either one operand is float, return float
                        return if left_operand_type.eq("float") && right_operand_type.eq("float") {
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
                let mult_expr_children = ast.get_children(node);
                let operator = ast
                    .get_node_value(mult_expr_children[1])
                    .get_atomic_concept()
                    .get_value();
                let left_operand_type =
                    refer_type_on_node(mult_expr_children[0], ast, scope, table_container)?;
                let right_operand_type =
                    refer_type_on_node(mult_expr_children[2], ast, scope, table_container)?;
                match operator.as_str() {
                    "*" | "/" => {
                        if !left_operand_type.eq("float") || !left_operand_type.eq("integer") {
                            return Err(SemanticError::report_error(&format!(
                                "real operator applied on {left_operand_type}, which is not a number"
                            )));
                        }
                        if !right_operand_type.eq("float") || !right_operand_type.eq("integer") {
                            return Err(SemanticError::report_error(&format!(
                                "real operator applied on {right_operand_type}, which is not a number"
                            )));
                        }
                        // if either one operand is float, return float
                        return if left_operand_type.eq("float") && right_operand_type.eq("float") {
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
                let not_expr_children = ast.get_children(node);
                let not_expr_type =
                    refer_type_on_node(not_expr_children[0], ast, scope, table_container)?;
                if not_expr_type.eq("bool") {
                    Ok("bool".to_string())
                } else {
                    Err(SemanticError::report_error(&format!(
                        "\"not\" can only be applied on bool, but {not_expr_type} is found."
                    )))
                }
            }
            CompositeConcept::SignedExpr => {
                let signed_expr_children = ast.get_children(node);
                let signed_expr_type =
                    refer_type_on_node(signed_expr_children[0], ast, scope, table_container)?;
                if signed_expr_type.eq("integer") || signed_expr_type.eq("float") {
                    Ok(signed_expr_type)
                } else {
                    Err(SemanticError::report_error(&format!(
                        "sign can only be applied on bool, but {signed_expr_type} is found."
                    )))
                }
            }
            CompositeConcept::IfThenElse => {
                let if_then_else_children = ast.get_children(node);
                let condition_type =
                    refer_type_on_node(if_then_else_children[0], ast, scope, table_container)?;
                if !condition_type.eq("bool") {
                    return Err(SemanticError::report_error(&format!(
                        "If condition should be bool, but {} is found",
                        condition_type
                    )));
                }
                let then_type =
                    refer_type_on_node(if_then_else_children[1], ast, scope, table_container)?;
                let else_type =
                    refer_type_on_node(if_then_else_children[2], ast, scope, table_container)?;
                return if then_type.eq(&else_type) {
                    Ok(then_type)
                } else {
                    Ok("".to_string())
                };
            }
            CompositeConcept::While => {
                let while_children = ast.get_children(node);
                let condition_type =
                    refer_type_on_node(while_children[0], ast, scope, table_container)?;
                if !condition_type.eq("bool") {
                    return Err(SemanticError::report_error(&format!(
                        "while condition should be bool, but {} is found",
                        condition_type
                    )));
                }
                refer_type_on_node(while_children[1], ast, scope, table_container)
            }
            CompositeConcept::Return => {
                let return_children = ast.get_children(node);
                let return_type =
                    refer_type_on_node(return_children[0], ast, scope, table_container)?;
                Ok(return_type)
            }
            CompositeConcept::StmtBlock => {
                let statements = ast.get_children(node);
                let mut stmt_type: String = "".to_string();
                for statement in statements {
                    stmt_type = refer_type_on_node(statement.clone(), ast, scope, table_container)?;
                }
                Ok(stmt_type)
            }
            CompositeConcept::AParams => {
                let a_params = ast.get_children(node);
                let mut result = String::from("");
                for node in a_params {
                    let node_type = refer_type_on_node(node.clone(), ast, scope, table_container)?;
                    result.push_str(&node_type);
                    result.push_str(",");
                }
                Ok(result[0..result.len() - 1].to_string())
            }
            CompositeConcept::FuncDef => {
                // get the table belong to this funcDef
                let func_def_children = ast.get_children(node);
                let func_name = ast
                    .get_node_value(func_def_children[0])
                    .get_atomic_concept()
                    .get_value();
                let this_table = table_container
                    .get(&*format!("{}:{}", &scope, &func_name))
                    .expect(&format!("cannot get table {}:{}", &scope, &func_name));

                // check statements in func body and return type matches

                let body_return_type = refer_type_on_node(
                    func_def_children[3], // funcBody
                    ast,
                    &this_table.get_table_name(),
                    table_container,
                )?;
                let defined_return_type = refer_type_on_node(
                    func_def_children[2],
                    ast,
                    &this_table.get_table_name(),
                    table_container,
                )?;
                return if body_return_type.eq("") && !defined_return_type.eq("void") {
                    Err(SemanticError::report_error(&format!(
                        "function {} doesn't contain return statement of type {}",
                        &this_table.get_table_name(),
                        &defined_return_type
                    )))
                } else if !body_return_type.eq("") && !body_return_type.eq(&defined_return_type) {
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
            // CompositeConcept::VarDecl => Ok("".to_string)
            CompositeConcept::FuncBody => {
                let func_body_children = ast.get_children(node);
                let mut return_type = String::from("");
                for stmt in func_body_children {
                    let stmt_concept = ast.get_node_value(stmt.clone());
                    if matches!(
                        stmt_concept,
                        Concept::CompositeConcept(CompositeConcept::Return)
                    ) {
                        match refer_type_on_node(stmt, ast, scope, table_container) {
                            Ok(t) => return_type = t,
                            Err(e) => return Err(e),
                        }
                    } else {
                        if let Err(e) = refer_type_on_node(stmt, ast, scope, table_container) {
                            return Err(e);
                        }
                    }
                }
                Ok(return_type)
            }
            CompositeConcept::Prog => {
                // for each child (funcDef, implDef, structDecl), check type
                for child in ast.get_children(node) {
                    refer_type_on_node(child, ast, "global", table_container)?;
                }
                Ok("".to_string())
            }
            CompositeConcept::Write => Ok("".to_string()),
            CompositeConcept::Read => Ok("".to_string()),
            CompositeConcept::ArraySizes => {
                let mut array_sizes_type = String::from("");
                for child in ast.get_children(node) {
                    let int_lit_type =
                        refer_type_on_node(child.clone(), ast, "global", table_container)?;
                    if !int_lit_type.eq("integer") {
                        return Err(SemanticError::report_error("array size must be integer"));
                    }
                    let int_lit_value = ast.get_node_value(child).get_atomic_concept().get_value();
                    array_sizes_type.push('[');
                    array_sizes_type.push_str(&int_lit_value);
                    array_sizes_type.push(']');
                }
                Ok(array_sizes_type)
            }
            CompositeConcept::StructDecl => {
                let struct_children = ast.get_children(node);
                let struct_name = ast
                    .get_node_value(struct_children[0])
                    .get_atomic_concept()
                    .get_value();
                refer_type_on_node(
                    struct_children[2],
                    ast,
                    &format!("{}:{}", scope, struct_name),
                    table_container,
                )
            }

            CompositeConcept::ImplDef => {
                let impl_children = ast.get_children(node);
                let impl_name = ast
                    .get_node_value(impl_children[0])
                    .get_atomic_concept()
                    .get_value();
                refer_type_on_node(
                    impl_children[1],
                    ast,
                    &format!("{}:{}", scope, impl_name),
                    table_container,
                )
            }

            CompositeConcept::FuncDefList
            | CompositeConcept::StructMemberDecl
            | CompositeConcept::StructMemberDeclList => {
                for child in ast.get_children(node) {
                    refer_type_on_node(child, ast, scope, table_container)?;
                }
                Ok("".to_string())
            }
            CompositeConcept::FParam => Ok("".to_string()),
            CompositeConcept::Type => {
                let type_children = ast.get_children(node);
                refer_type_on_node(type_children[0], ast, scope, table_container)
            }
            CompositeConcept::FParams => Ok("".to_string()),
            CompositeConcept::VarDecl => Ok("".to_string()),
            CompositeConcept::FuncDecl => Ok("".to_string()),
            CompositeConcept::InheritsList => Ok("".to_string()),
        },
    }
}

fn check_func_def(table_container: &HashMap<String, SymbolTable>) {
    // for each table, each function should have link
    for (table_name, table) in table_container {
        for entry in table.get_all_entries() {
            if matches!(entry.kind, SymbolKind::Function) {
                if entry.link.is_none() {
                    SemanticError::report(
                        SemanticErrType::Error,
                        &format!(
                            "function {}:{} is declared but not defined",
                            table_name, entry.name
                        ),
                    );
                }
            }
        }
    }
}

// return the table name that contains the member
fn search_inherited_class_from_member(
    member_name: &str,
    func_param_type: Option<String>,
    table_name: &str,
    table_container: &HashMap<String, SymbolTable>,
) -> Option<String> {
    let table = table_container.get(table_name).unwrap();
    match func_param_type {
        None => {
            // not looking for a function. Search by name only
            let entries_in_table = table.get_all_entries_by_name(member_name);
            if entries_in_table.is_empty() {
                // member not in the table. Check inherits
                let inherits_entries = table.get_all_entries_by_kind(SymbolKind::Inherits);

                for inherits in inherits_entries {
                    if let Some(target_table_name) = search_inherited_class_from_member(
                        member_name,
                        None,
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
        Some(ref key_param_type) => {
            let entries = table.get_all_entries_by_name(member_name);
            for entry in entries {
                let entry_type = entry.symbol_type.get_name();
                let param_type = entry_type.split(":").last().unwrap();
                if key_param_type.eq(param_type) {
                    return Some(table_name.to_string());
                }
            }

            // function of given param type is not found in this table. check inherits
            let inherits_entries = table.get_all_entries_by_kind(SymbolKind::Inherits);
            for inherits in inherits_entries {
                if let Some(target_table_name) = search_inherited_class_from_member(
                    member_name,
                    func_param_type.clone(),
                    &inherits.link.unwrap(),
                    table_container,
                ) {
                    return Some(target_table_name);
                }
            }
            None
        }
    }
}
