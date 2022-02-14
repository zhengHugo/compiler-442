mod lexical;
mod syntactic;

use crate::syntactic::parser::Parser;
use lexical::lexer::Lexer;
use std::fs;

fn main() {
    let mut lexer: Lexer = Lexer::new();
    let parser = Parser::new();
    if let Ok(src) = fs::read_to_string("resource/syntax/bubblesort.src") {
        lexer.read_source(&src);
        parser.parse(lexer.get_tokens());
    }
}
