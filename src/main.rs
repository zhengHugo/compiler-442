mod lexer_machine_impl;
mod lexical_error;
mod token;

extern crate core;

use crate::lexer_machine_impl::LexerStateMachineImpl;
use crate::lexer_machine_impl::State;
use crate::lexical_error::LexicalError;
use crate::token::Token;
use crate::token::TokenType;
use rust_fsm::{StateMachine, StateMachineImpl};
use std::fs;

struct Lexer {
    state_machine: StateMachine<LexerStateMachineImpl>,
    buffer: String,
    current_loc: (i32, i32),
    output_tokens: Vec<Token>,
    output_index: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            state_machine: StateMachine::new(),
            buffer: String::from(""),
            current_loc: (0, 0),
            output_tokens: vec![],
            output_index: 0,
        }
    }

    pub fn read_source(&mut self, source: &str) {
        // for (i, c) in source.chars().enumerate() {
        for c in source.chars() {
            self.update_loc(&c);
            match c {
                ' ' | '\t' | '\n' => {
                    if matches!(self.state_machine.state(), State::Str2) {
                        // is reading a string now. consume the space
                        self.state_machine.consume(&c).expect(&*format!(
                            "Transition impossible with ({:?}, {})",
                            State::Str2,
                            c
                        ));
                    } else if !(self.buffer.is_empty()) {
                        // if buffer has something in it, finalize a token
                        let output_token = self.finalize_token();
                        match output_token {
                            Ok(token) => {
                                self.output_tokens.push(token);
                            }
                            Err(e) => {
                                panic!("Lexical Error found!\n {}", e);
                            }
                        }
                    }
                }
                _ => {
                    self.next_char(&c);
                }
            }
        }
        // when loop ends, flush out what's in the buffer
        let output_token = self.finalize_token();
        match output_token {
            Ok(token) => {
                self.output_tokens.push(token);
            }
            Err(e) => {
                panic!("Lexical Error found!\n {}", e);
            }
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.output_index < self.output_tokens.len() {
            let result_token = self.output_tokens[self.output_index].clone();
            self.output_index += 1;
            return Some(result_token);
        }
        None
    }

    fn next_char(&mut self, input: &char) {
        let consumed_result = self.state_machine.consume(input);
        match consumed_result {
            Ok(_output) => {
                self.buffer.push(input.clone());
            }
            Err(_e) => {
                // if transition error happens,
                // 1. finalize the last token first
                let output_token = self.finalize_token();
                match output_token {
                    Ok(token) => {
                        self.output_tokens.push(token);
                    }
                    Err(e) => {
                        panic!("Lexical Error found!\n {}", e);
                    }
                }
                // 2. consume the current char
                self.next_char(input);
            }
        }
    }

    // try to get a token from the current state machine
    fn finalize_token(&mut self) -> Result<Token, LexicalError> {
        match LexerStateMachineImpl::state_to_token_type(&self.state_machine.state()) {
            Some(token_type) => {
                let result = Ok(Token {
                    token_type,
                    lexeme: self.buffer.clone(),
                    location: self.current_loc,
                });
                self.buffer.clear();
                self.state_machine = StateMachine::from_state(State::Start);
                result
            }
            None => {
                let result = Err(LexicalError {
                    invalid_lexeme: self.buffer.clone(),
                    loc: self.current_loc,
                });
                self.buffer.clear();
                self.state_machine = StateMachine::from_state(State::Start);
                result
            }
        }
    }

    fn update_loc(&mut self, c: &char) {
        match c {
            '\n' => {
                self.current_loc.0 += 1;
                self.current_loc.1 = 0;
            }
            _ => {
                self.current_loc.1 += 1;
            }
        }
    }
}

fn main() {
    let source: String =
        fs::read_to_string("sample.src").expect("Something went wrong reading the file");
    let mut lexer: Lexer = Lexer::new();
    let result = lexer.read_source(&source);
    println!("{:?}", result);
}
