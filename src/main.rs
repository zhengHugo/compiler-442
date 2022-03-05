extern crate core;

mod lexical;
mod semantic;
mod syntactic;

use crate::syntactic::parser::Parser;
use lexical::lexer::Lexer;
use std::fs;
use std::fs::File;

fn main() {
    let mut lexer: Lexer = Lexer::new();
    let mut parser = Parser::new();
    if let Ok(src) = fs::read_to_string("resource/ast/bubblesort.src") {
        lexer.read_source(&src);
        parser.parse(lexer.get_tokens());
    } else {
        panic!("Cannot find source code");
    }
}
