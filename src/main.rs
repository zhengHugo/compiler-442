mod lexer;
mod lexer_machine_impl;
mod lexical_error;
mod token;

extern crate core;

use crate::lexer::Lexer;
use crate::lexer_machine_impl::LexerStateMachineImpl;
use crate::lexer_machine_impl::State;
use crate::lexical_error::LexicalError;
use crate::token::Token;
use std::fs;

fn main() {
    let input_files = ["lexnegativegrading", "lexpositivegrading"];
    for input_file in input_files {
        let source: String = fs::read_to_string(input_file.to_owned() + ".src")
            .expect("Something went wrong reading the file");
        let mut lexer: Lexer = Lexer::new();
        lexer.read_source(&source);
        loop {
            let token = lexer.next_token();
            if token.is_none() {
                break;
            }
            println!("{}", token.unwrap());
        }
    }
}
