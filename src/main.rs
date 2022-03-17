extern crate core;

mod lexical;
mod semantic;
mod syntactic;

use crate::semantic::ast::{generate_symbol_tables, AbstractSyntaxTree};
use crate::syntactic::parser::Parser;
use lexical::lexer::Lexer;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut lexer: Lexer = Lexer::new();
    let mut parser = Parser::new();
    let path = "resource/ast/test";
    if let Ok(src) = fs::read_to_string(path.to_string() + ".src") {
        lexer.read_source(&src);
        let (_, ast) = parser.parse(lexer.get_tokens()).unwrap();
        let mut outast_file = File::create(path.to_string() + ".outast").unwrap();
        outast_file.write_all(format!("{}", &ast).as_bytes());
        let tables = generate_symbol_tables(&ast as &AbstractSyntaxTree);
        for (_, table) in tables.iter() {
            println!("{}", table);
        }
        // let mut outast_file = File::create(path.to_string() + ".outast").unwrap();
        // outast_file.write_all(format!("{}", ast).as_bytes());
    } else {
        panic!("Cannot find source code");
    }
}
