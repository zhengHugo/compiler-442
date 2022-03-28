use crate::code_generation::register::RegisterPool;
use crate::semantic::concept::{CompositeConcept, Concept};
use crate::semantic::symbol_table::SymbolTable;
use crate::syntactic::tree::NodeId;
use crate::AbstractSyntaxTree;
use std::collections::HashMap;

pub fn generate_moon_code(
    ast: AbstractSyntaxTree,
    table_container: HashMap<String, SymbolTable>,
) -> String {
    let mut stack_offset: u32 = 0;
    let root = ast.get_root();
    let mut translator: Translator = Translator {
        ast,
        table_container,
        output_code: String::new(),
        register_pool: RegisterPool::new(),
        stack_offset: 0,
    };
    translator.translate(root, "global");
    translator.output_code
}

pub struct Translator {
    ast: AbstractSyntaxTree,
    table_container: HashMap<String, SymbolTable>,
    output_code: String,
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
                CompositeConcept::Prog => self.translate_prog(node, "global"),
                _ => panic!("Unhandled composite concept {}", cc),
            },
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
            self.write_code("", "entry", "% =====program entry=====");
            self.write_code("", "align", "% following instructions align");
            self.translate(children[3], &format!("{}:{}", table_name, "main"));
        }
    }

    fn grow_stack(&mut self, size: u32) {
        self.stack_offset += size;
        self.write_code("", "subi", &format!("R1, R1, {}", size));
    }

    fn write_code(&mut self, tag: &str, op: &str, remain: &str) {
        self.output_code
            .push_str(&format!("{:<10} {:<10} {}\n", tag, op, remain))
    }
}
