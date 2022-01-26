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
    let source: String = fs::read_to_string("lexpositivegrading.src")
        .expect("Something went wrong reading the file");
    let mut lexer: Lexer = Lexer::new();
    lexer.read_source(&source);
    let mut token = lexer.next_token();
    while token.is_some() {
        // println!("{}", token.expect("Not a token!"));
        token = lexer.next_token();
    }
}
