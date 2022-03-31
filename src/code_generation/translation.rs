use crate::code_generation::register_pool::RegisterPool;
use crate::code_generation::temp_var_name_pool::TempVarNamePool;
use crate::semantic::concept::{AtomicConceptType, CompositeConcept, Concept};
use crate::semantic::semantic_util::var_node_utils;
use crate::semantic::symbol_table::{SymbolKind, SymbolTable, SymbolTableEntry, SymbolType};
use crate::syntactic::tree::NodeId;
use crate::AbstractSyntaxTree;
use std::collections::HashMap;

pub struct Translator {
    ast: AbstractSyntaxTree,
    table_container: HashMap<String, SymbolTable>,
    exec_code: String,
    data_code: String,
    register_pool: RegisterPool,
    temp_var_name_pool: TempVarNamePool,
    stack_offset: u32,
}
impl Translator {
    // translate statements
    pub fn translate(&mut self, node: NodeId, table_name: &str) {
        let node_value = self.ast.get_node_value(node);
        match node_value {
            Concept::AtomicConcept(ac) => {
                panic!("Unhandled atomic concept {}", ac)
            }
            Concept::CompositeConcept(cc) => match cc {
                CompositeConcept::Assign => self.translate_assign(node, table_name),
                CompositeConcept::VarDecl => self.translate_var_decl(node, table_name),
                CompositeConcept::FuncDef => self.translate_func_def(node, table_name),
                CompositeConcept::FuncBody => self.translate_func_body(node, table_name),
                CompositeConcept::Write => self.translate_write(node, table_name),
                CompositeConcept::Prog => self.translate_prog(node, "global"),
                _ => panic!("Unhandled composite concept {}", cc),
            },
        }
    }

    fn translate_write(&mut self, node: NodeId, table_name: &str) {
        // 1. store the value to write into -8(r14)
        // 2. prepare a buffer to convert it into a string
        // 3. call 'intstr' to perform the conversion.
        // 4. the  pointer to the result string is in r13
        // 5. call 'putstr' to write the string
        let write_children = self.ast.get_children(node);
        let elem_to_write = self.ast.get_node_value(write_children[0]);
        let reg = self.register_pool.get_register();
        match elem_to_write {
            Concept::AtomicConcept(ac) => match ac.get_atomic_concept_type() {
                AtomicConceptType::IntLit => {
                    let int_val = ac.get_value();
                    self.write_exec_code("", "% load integer to print into param reg", "");
                    self.write_exec_code("", "addi", &format!("r{}, r0, {}", reg, int_val));
                    self.write_exec_code("", "sw", &format!("-8(r14), r{}", reg));
                }
                _ => panic!("Unhandled ac in translate_write(): {}", ac),
            },
            Concept::CompositeConcept(cc) => {
                match cc {
                    CompositeConcept::Dot => {}
                    CompositeConcept::Var => {
                        let var_name = self.translate_with_result(write_children[0], table_name);
                        let table = self.table_container.get(table_name).unwrap();
                        let entry_offset = table.get_entry_by_name(&var_name).unwrap().offset;

                        self.write_exec_code("", "% load var to print into param reg", "");
                        self.write_exec_code(
                            "",
                            "lw",
                            &format!("r{}, -{}(r12)", reg, entry_offset),
                        );
                        self.write_exec_code("", "sw", &format!("-8(r14), r{}", reg));

                        /*
                        If we have a variable in 'write':
                        Then look it up in the table and get its offset;
                            load the value into a temp register
                            store the value into -8(r14)
                        */
                    }
                    CompositeConcept::FuncCall => {
                        /*
                        If we have a function call in 'write':
                        */
                        todo!();
                    }
                    CompositeConcept::RelExpr => {
                        /*
                        If we have an expression in 'write':
                        Then translate the code that compute the expr
                            where to put this result? specify in translate_expr function;
                            load the value into a temp register;
                            store the value into -8(r14);
                        */
                        todo!();
                    }
                    // CompositeConcept::AddExpr => {}
                    // CompositeConcept::MultExpr => {}
                    // CompositeConcept::NotExpr => {}
                    // CompositeConcept::SignedExpr => {}
                    _ => panic!("Unhandled cc in translate_write(): {}", cc),
                }
            }
        }
        self.write_exec_code("", "% load the buffer pointer into param reg", "");
        self.write_exec_code("", "addi", &format!("r{}, r0, {}", reg, "buf"));
        self.write_exec_code("", "sw", &format!("-12(r14), r{}", reg));
        self.write_exec_code("", "% call intstr to convert int to str", "");
        self.write_exec_code("", "jl", "r15, intstr");
        self.write_exec_code("", "% load the result into param reg", "");
        self.write_exec_code("", "sw", "-8(r14), r13");
        self.write_exec_code("", "jl", "r15, putstr");
        self.register_pool.give_back(reg);
    }

    fn translate_assign(&mut self, node: NodeId, table_name: &str) {
        let assign_children = self.ast.get_children(node);

        // Compute rhs and load into reg
        // After an assignment, stack size should not change
        // Store the stack size before compute the rhs so that we can
        // drop the temp values
        let init_stack_offset = self.stack_offset;
        let rhs_temp_var_name = self.translate_with_result(assign_children[1], table_name);
        let table = self.table_container.get(table_name).unwrap().clone();
        let rhs_offset = table
            .get_entry_by_name(&rhs_temp_var_name)
            .expect(&format!("Cannot find entry {}", rhs_temp_var_name))
            .offset;
        let reg = self.register_pool.get_register();
        self.write_exec_code("", "lw", &format!("r{}, -{}(r12)", reg, rhs_offset));

        // TODO: assume lhs is an id
        // get the address of lhs
        let lhs_concept = self.ast.get_node_value(assign_children[0]);
        let lhs_offset: u32;
        match lhs_concept {
            Concept::AtomicConcept(ac) => {
                if matches!(ac.get_atomic_concept_type(), AtomicConceptType::Id) {
                    let lhs = ac.get_value();
                    let lhs_entry = self
                        .table_container
                        .get(table_name)
                        .unwrap()
                        .get_entry_by_name(&lhs)
                        .expect(&format!(
                            "Cannot find symbol {} in table {}",
                            lhs, table_name
                        ));
                    lhs_offset = lhs_entry.offset;
                    self.write_exec_code("", "sw", &format!("-{}(r12), r{}", lhs_offset, reg));
                }
            }
            Concept::CompositeConcept(cc) => {
                match cc {
                    CompositeConcept::Var => {
                        // get lhs offset from a var
                        let reg_target_address =
                            self.get_var_array_elem_address(assign_children[0], table_name);
                        self.write_exec_code(
                            "",
                            "sw",
                            &format!("0(r{}), r{}", reg_target_address, reg),
                        );
                    }
                    _ => panic!("{}", cc),
                }
            }
        }

        self.register_pool.give_back(reg);
        self.stack_offset = init_stack_offset;
        self.write_exec_code("", "subi", &format!("r14, r12, {}", self.stack_offset));
    }

    fn translate_prog(&mut self, prog_node: NodeId, table_name: &str) {
        let prog_children = self.ast.get_children(prog_node);
        for child in prog_children {
            self.translate(child, table_name);
        }
    }

    fn translate_var_decl(&mut self, var_decl_node: NodeId, table_name: &str) {
        // compute offset and write into table
        let var_decl_children = self.ast.get_children(var_decl_node);
        let var_name = self
            .ast
            .get_node_value(var_decl_children[0])
            .get_atomic_concept()
            .get_value();

        // set size and offset
        let var_size: u32;
        {
            let table = self
                .table_container
                .get_mut(table_name)
                .expect(&format!("Cannot find table {}", table_name));
            let var_entry = table.get_mut_entry_by_name(&var_name).expect(&format!(
                "cannot find entry {} in table {}",
                var_name, table_name
            ));
            var_size = var_entry.size;
            var_entry.offset = self.stack_offset + var_size;
        }
        self.grow_stack(var_size);
    }

    fn translate_func_body(&mut self, func_body_node: NodeId, table_name: &str) {
        let stmt_children = self.ast.get_children(func_body_node);
        for child in stmt_children {
            self.translate(child, table_name);
        }
    }

    fn translate_func_def(&mut self, func_def_node: NodeId, table_name: &str) {
        let children = self.ast.get_children(func_def_node);
        let caller = self.ast.get_node_value(children[0]);
        if caller.is_atomic() && caller.get_atomic_concept().get_value().eq("main") {
            // handle main function: program start
            self.write_exec_code("", "entry", "% =====program entry=====");
            self.write_exec_code("", "align", "% following instructions align");
            self.write_exec_code("", "addi", "r14, r0, topaddr    % stack pointer");
            self.write_exec_code("", "addi", "r12, r0, topaddr    % frame pointer");
            self.translate(children[3], &format!("{}:{}", table_name, "main"));
            self.write_exec_code("", "hlt", "% =====end of program====");

            // reserve a buffer for write
            self.write_data_code("buf", "res", "32 % reserve a buffer used by intstr");
        }
    }

    // translate statements with return value: returns the temp_var_name stored in this table
    // returns a temp var name
    pub fn translate_with_result(&mut self, node: NodeId, table_name: &str) -> String {
        let node_value = self.ast.get_node_value(node);
        match node_value {
            Concept::AtomicConcept(ac) => match ac.get_atomic_concept_type() {
                AtomicConceptType::IntLit => {
                    self.translate_with_result_int_lit(ac.get_value(), table_name)
                }
                AtomicConceptType::Id => ac.get_value(),
                // AtomicConceptType::FloatLit => {}
                // AtomicConceptType::RelOp => {}
                // AtomicConceptType::MultiOp => {}
                // AtomicConceptType::AddOp => {}
                // AtomicConceptType::Sign => {}
                _ => panic!("Unhandled ac in translate_with_result: {}", ac),
            },
            Concept::CompositeConcept(cc) => match cc {
                CompositeConcept::AddExpr
                | CompositeConcept::MultExpr
                | CompositeConcept::RelExpr => {
                    self.translate_with_result_binary_expr(node, table_name)
                }
                // CompositeConcept::Dot => {}
                CompositeConcept::Var => self.translate_with_result_var(node, table_name),
                // CompositeConcept::FuncCall => {}
                // CompositeConcept::NotExpr => {}
                // CompositeConcept::SignedExpr => {}
                // CompositeConcept::Type => {}
                _ => panic!("Unhandled cc in translate_with_result: {}", cc),
            },
        }
    }

    fn translate_with_result_int_lit(&mut self, val: String, table_name: &str) -> String {
        let reg = self.register_pool.get_register();
        let mut table = self.table_container.get(table_name).unwrap().clone();
        let (temp_var_name, _) = self.allocate_temp_var(4, table);
        // load into register
        self.write_exec_code("", "addi", &format!("r{}, r0, {}", reg, val));
        // allocate a temp var and store it
        self.write_exec_code("", "sw", &format!("-{}(r12), r{}", self.stack_offset, reg));
        self.register_pool.give_back(reg);
        temp_var_name
    }

    fn translate_with_result_var(&mut self, var_node: NodeId, table_name: &str) -> String {
        let var_children = self.ast.get_children(var_node);

        let index_nodes = self.ast.get_children(var_children[1]);
        if index_nodes.is_empty() {
            self.translate_with_result(var_children[0], table_name)
        } else {
            let base_size = var_node_utils::get_base_size(
                var_node,
                table_name,
                &self.ast,
                &self.table_container,
            );
            let table = self.table_container.get(table_name).unwrap();
            let (temp_var_name, temp_var_offset) = self.allocate_temp_var(base_size, table.clone());
            let reg_target_addr = self.get_var_array_elem_address(var_node, table_name);

            let reg_temp_var = self.register_pool.get_register();
            // reg_temp_var <- Mem[reg_target_addr]
            // temp_var <- reg_temp_var
            self.write_exec_code(
                "",
                "lw",
                &format!("r{}, 0(r{})", reg_temp_var, reg_target_addr),
            );
            self.write_exec_code(
                "",
                "sw",
                &format!("-{}(r12), r{}", temp_var_offset, reg_temp_var),
            );

            self.register_pool.give_back(reg_target_addr);
            temp_var_name
        }
    }

    fn translate_with_result_binary_expr(&mut self, node: NodeId, table_name: &str) -> String {
        let reg1 = self.register_pool.get_register();
        let reg2 = self.register_pool.get_register();
        let reg3 = self.register_pool.get_register();
        let expr_children = self.ast.get_children(node);
        let operand1_name = self.translate_with_result(expr_children[0], table_name);
        let operand2_name = self.translate_with_result(expr_children[2], table_name);
        let operator = self
            .ast
            .get_node_value(expr_children[1])
            .get_atomic_concept()
            .get_value();

        let mut table = self
            .table_container
            .get(table_name)
            .expect(&format!("Cannot find table {}", table_name))
            .clone();
        let operand1_offset = table.get_entry_by_name(&operand1_name).unwrap().offset;
        let operand2_offset = table.get_entry_by_name(&operand2_name).unwrap().offset;

        // grow stack for a temp var
        // TODO: only integer expression is supported
        let (temp_var_name, _) = self.allocate_temp_var(4, table);

        self.write_exec_code("", "lw", &format!("r{}, -{}(r12)", reg1, operand1_offset));
        self.write_exec_code("", "lw", &format!("r{}, -{}(r12)", reg2, operand2_offset));
        match operator.as_str() {
            "+" => self.write_exec_code("", "add", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "-" => self.write_exec_code("", "sub", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "or" => self.write_exec_code("", "or", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "*" => self.write_exec_code("", "mul", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "/" => self.write_exec_code("", "div", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "and" => self.write_exec_code("", "and", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "==" => self.write_exec_code("", "ceq", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            ">=" => self.write_exec_code("", "cge", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            ">" => self.write_exec_code("", "cgt", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "<=" => self.write_exec_code("", "cle", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "<" => self.write_exec_code("", "clt", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            "<>" => self.write_exec_code("", "cne", &format!("r{}, r{}, r{}", reg3, reg1, reg2)),
            _ => panic!("Unhandled binary operator {}", operator),
        }
        self.write_exec_code("", "sw", &format!("-{}(r12), r{}", self.stack_offset, reg3));
        self.register_pool.give_back(reg3);
        self.register_pool.give_back(reg2);
        self.register_pool.give_back(reg1);
        temp_var_name
    }

    fn grow_stack(&mut self, size: u32) {
        self.stack_offset += size;
        self.write_exec_code("", "subi", &format!("r14, r14, {}", size));
    }

    fn write_exec_code(&mut self, tag: &str, op: &str, remain: &str) {
        self.exec_code
            .push_str(&format!("{:<10} {:<10} {}\n", tag, op, remain));
    }

    fn write_data_code(&mut self, tag: &str, op: &str, remain: &str) {
        self.data_code
            .push_str(&format!("{:<10} {:<10} {}\n", tag, op, remain));
    }

    fn allocate_temp_var(&mut self, size: u32, mut table: SymbolTable) -> (String, u32) {
        let temp_var_name = self
            .temp_var_name_pool
            .get_next_unique_name_in_table(&table);
        table.insert(SymbolTableEntry {
            name: temp_var_name.clone(),
            kind: SymbolKind::Variable,
            symbol_type: SymbolType::new("integer"),
            link: None,
            size,
            offset: self.stack_offset + size,
        });
        self.grow_stack(size);
        self.table_container.insert(table.get_table_name(), table);
        (temp_var_name, self.stack_offset)
    }

    fn load_var_to_reg(&mut self, var_name: String, reg: i32, table_name: &str) {}

    // generate code to get var array access offset
    // returns a register that stores the address of the array element
    fn get_var_array_elem_address(&mut self, var_node: NodeId, table_name: &str) -> i32 {
        // TODO: assume base_var is a id, not a dot
        let base_size =
            var_node_utils::get_base_size(var_node, table_name, &self.ast, &self.table_container);

        // TODO: assume one-dimensional array
        // get base offset
        let var_entry =
            var_node_utils::get_entry(var_node, table_name, &self.ast, &self.table_container);
        let base_offset = var_entry.offset;
        // reg_base_offset <- base_offset
        let reg_base_offset = self.register_pool.get_register();
        self.write_exec_code(
            "",
            "addi",
            &format!("r{}, r0, {}", reg_base_offset, base_offset),
        );

        // reg_offset <- index
        let reg_offset = self.register_pool.get_register();
        let var_children = self.ast.get_children(var_node);
        let indices_nodes = self.ast.get_children(var_children[1]);
        let index_var_name = self.translate_with_result(indices_nodes[0], table_name);
        let table = self.table_container.get(table_name).unwrap();
        let index_var_entry = table.get_entry_by_name(&index_var_name).unwrap();
        let index_var_offset = index_var_entry.offset;
        self.write_exec_code(
            "",
            "lw",
            &format!("r{}, -{}(r12)", reg_offset, index_var_offset),
        );
        // reg_offset <- reg_offset * base_size
        self.write_exec_code(
            "",
            "muli",
            &format!("r{}, r{}, {}", reg_offset, reg_offset, base_size),
        );

        // reg_offset <- reg_base_offset - reg_offset
        self.write_exec_code(
            "",
            "sub",
            &format!("r{}, r{}, r{}", reg_offset, reg_base_offset, reg_offset),
        );

        // reg_target_addr = r12 - reg_offset
        let reg_target_addr = self.register_pool.get_register();
        self.write_exec_code(
            "",
            "sub",
            &format!("r{}, r12, r{}", reg_target_addr, reg_offset),
        );
        self.register_pool.give_back(reg_base_offset);
        self.register_pool.give_back(reg_offset);
        reg_target_addr
    }
}

pub fn generate_moon_code(
    ast: AbstractSyntaxTree,
    table_container: HashMap<String, SymbolTable>,
) -> String {
    let root = ast.get_root();
    let mut translator: Translator = Translator {
        ast,
        table_container,
        exec_code: String::new(),
        data_code: String::new(),
        register_pool: RegisterPool::new(),
        temp_var_name_pool: TempVarNamePool::new(),
        stack_offset: 0,
    };
    translator.translate(root, "global");
    format!("{}{}", translator.exec_code, translator.data_code)
}
