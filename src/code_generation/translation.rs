use crate::code_generation::register::RegisterPool;
use crate::semantic::concept::{AtomicConceptType, CompositeConcept, Concept};
use crate::semantic::symbol_table::SymbolTable;
use crate::syntactic::tree::NodeId;
use crate::AbstractSyntaxTree;
use std::collections::HashMap;

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
        stack_offset: 0,
    };
    translator.translate(root, "global");
    format!("{}{}", translator.exec_code, translator.data_code)
}

pub struct Translator {
    ast: AbstractSyntaxTree,
    table_container: HashMap<String, SymbolTable>,
    exec_code: String,
    data_code: String,
    register_pool: RegisterPool,
    stack_offset: u32,
}
impl Translator {
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
        {
            self.write_data_code("buf", "res", "20 % reserve a buffer used by intstr");
        }
        let write_children = self.ast.get_children(node);
        let elem_to_write = self.ast.get_node_value(write_children[0]);
        match elem_to_write {
            Concept::AtomicConcept(ac) => match ac.get_atomic_concept_type() {
                AtomicConceptType::Id => {}
                AtomicConceptType::FloatLit => {}
                AtomicConceptType::IntLit => {
                    let reg = self.register_pool.get_register();
                    let int_val = ac.get_value();
                    self.write_exec_code("", "% load integer to be print into param reg", "");
                    self.write_exec_code("", "addi", &format!("r{}, r0, {}", reg, int_val));
                    self.write_exec_code("", "sw", &format!("-8(r14), r{}", reg));
                    self.write_exec_code("", "% load the buffer pointer into param reg", "");
                    self.write_exec_code("", "addi", &format!("r{}, r0, {}", reg, "buf"));
                    self.write_exec_code("", "sw", &format!("-12(r14), r{}", reg));
                    self.write_exec_code("", "% call intstr to convert int to str", "");
                    self.write_exec_code("", "jl", "r15, intstr");
                    self.write_exec_code("", "% load the result into param reg", "");
                    self.write_exec_code("", "sw", "-8(r14), r13");
                    self.write_exec_code("", "jl", "r15, putstr");
                }
                _ => panic!("Unhandled ac in translate_write(): {}", ac),
            },
            Concept::CompositeConcept(cc) => {
                match cc {
                    CompositeConcept::Dot => {}
                    CompositeConcept::Var => {
                        /*
                        If we have a variable in 'write':
                        Then look it up in the table and get its offset;
                            load the value into a temp register
                            store the value into -8(r14)
                        */
                        todo!();
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
                    CompositeConcept::AddExpr => {}
                    CompositeConcept::MultExpr => {}
                    CompositeConcept::NotExpr => {}
                    CompositeConcept::SignedExpr => {}
                    _ => panic!("Unhandled cc in translate_write(): {}", cc),
                }
            }
        }
        // 1. load the value to right into register
    }

    fn translate_assign(&mut self, node: NodeId, table_name: &str) {
        let assign_children = self.ast.get_children(node);
        // TODO: assume lhs is an id
        let lhs = self
            .ast
            .get_node_value(assign_children[0])
            .get_atomic_concept()
            .get_value();
        let lhs_entry = self
            .table_container
            .get(table_name)
            .unwrap()
            .get_entry_by_name(&lhs)
            .expect(&format!(
                "Cannot find symbol {} in table {}",
                lhs, table_name
            ));

        // TODO: assume rhs is an literal
        let rhs_concept = self.ast.get_node_value(assign_children[1]);
        match rhs_concept {
            Concept::AtomicConcept(ac) => match ac.get_atomic_concept_type() {
                AtomicConceptType::Id => {}
                AtomicConceptType::FloatLit => {}
                AtomicConceptType::IntLit => {
                    let rhs_value = ac.get_value();
                    let reg = self.register_pool.get_register();
                    // TODO: assume positive number
                    self.write_exec_code("", "addi", &format!("r{}, r0, {}", reg, rhs_value));
                    self.write_exec_code("", "sw", &format!("-{}(r1), r{}", lhs_entry.offset, reg));
                    self.register_pool.give_back(reg);
                }
                _ => panic!("Unexpected assignment rhs: {}", rhs_concept),
            },
            Concept::CompositeConcept(_) => {
                todo!("expr on assignment rhs");
                todo!("var on assignment rhs");
                todo!("func call on assignment rhs");
            }
        }
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
            self.write_exec_code("", "addi", "r1, r0, topaddr    % frame pointer");
            self.translate(children[3], &format!("{}:{}", table_name, "main"));
            self.write_exec_code("", "hlt", "% =====end of program====");
        }
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
}
