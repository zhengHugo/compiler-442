mod lexical_error;
mod token;

extern crate core;

use crate::lexical_error::LexicalError;
use crate::token::Token;
use crate::token::TokenType;
use rust_fsm::{StateMachine, StateMachineImpl};
use std::fs;

#[derive(Debug)]
enum State {
    Start,
    Id2,
    Str2,
    Str3,
    Int12,
    Int13,
    Int21,
    Int22,
    Int23,
    Int31,
    Int32,
    Int33,
    Frac12,
    Frac13,
    Frac14,
    Frac15,
}

struct Lexer {
    state_machine: StateMachine<LexerStateMachineImpl>,
    buffer: String,
    current_loc: (i32, i32),
    output_tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            state_machine: StateMachine::new(),
            buffer: String::from(""),
            current_loc: (0, 0),
            output_tokens: vec![],
        }
    }

    pub fn take(&mut self, source: &str) -> Vec<Token> {
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
        self.output_tokens.to_vec()
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

struct LexerStateMachineImpl {}

impl LexerStateMachineImpl {
    fn state_to_token_type(
        state: &<LexerStateMachineImpl as StateMachineImpl>::State,
    ) -> Option<<LexerStateMachineImpl as StateMachineImpl>::Output> {
        match state {
            State::Id2 => Some(TokenType::Id),
            State::Str3 => Some(TokenType::Str),
            State::Int12 | State::Int13 => Some(TokenType::Integer),
            State::Frac13
            | State::Frac14
            | State::Int22
            | State::Int23
            | State::Int32
            | State::Int33 => Some(TokenType::Float),
            _ => None,
        }
    }
}

impl StateMachineImpl for LexerStateMachineImpl {
    type Input = char;
    type State = State;
    type Output = TokenType;

    const INITIAL_STATE: Self::State = State::Start;

    fn transition(state: &Self::State, input: &Self::Input) -> Option<Self::State> {
        match (state, input) {
            (State::Start, '0') => Some(State::Int12),
            (State::Start, '1'..='9') => Some(State::Int13),
            (State::Start, 'A'..='Z' | 'a'..='z') => Some(State::Id2),
            (State::Start, '"') => Some(State::Str2),
            (State::Id2, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_') => Some(State::Id2),
            (State::Str2, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | ' ') => Some(State::Str2),
            (State::Str2, '"') => Some(State::Str3),
            (State::Int12 | State::Int13, '.') => Some(State::Frac12),
            (State::Int13, '0'..='9') => Some(State::Int13),
            (State::Frac12, '0') => Some(State::Frac14),
            (State::Frac12, '1'..='9') => Some(State::Frac13),
            (State::Frac13, '0') => Some(State::Frac15),
            (State::Frac13, '1'..='9') => Some(State::Frac13),
            (State::Frac14, '0') => Some(State::Frac15),
            (State::Frac14, '1'..='9') => Some(State::Frac14),
            (State::Frac15, '0') => Some(State::Frac15),
            (State::Frac15, '1'..='9') => Some(State::Frac14),
            (State::Frac13 | State::Frac14, 'e') => Some(State::Int21),
            (State::Int21, '0') => Some(State::Int22),
            (State::Int21, '+' | '-') => Some(State::Int31),
            (State::Int21, '1'..='9') => Some(State::Int23),
            (State::Int23, '0'..='9') => Some(State::Int23),
            (State::Int31, '0') => Some(State::Int32),
            (State::Int31, '1'..='9') => Some(State::Int33),
            (State::Int33, '0'..='9') => Some(State::Int33),
            _ => None,
        }
    }

    fn output(state: &Self::State, input: &Self::Input) -> Option<Self::Output> {
        let next_state = Self::transition(state, input).expect("Unhandled transition error");
        LexerStateMachineImpl::state_to_token_type(&next_state)
        // match (state, input) {
        //     (State::Start, '0') => Some(TokenType::Integer),
        //     (State::Start, '1'..='9') => Some(TokenType::Integer),
        //     (State::Start, 'A'..='Z' | 'a'..='z') => Some(TokenType::Id),
        //     (State::Id2, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_') => Some(TokenType::Id),
        //     (State::Str2, '"') => Some(TokenType::Str),
        //     (State::Int13, '0'..='9') => Some(TokenType::Integer),
        //     (State::Frac12, '0') => Some(TokenType::Float),
        //     (State::Frac12, '1'..='9') => Some(TokenType::Float),
        //     (State::Frac13, '1'..='9') => Some(TokenType::Float),
        //     (State::Frac14, '1'..='9') => Some(TokenType::Float),
        //     (State::Frac15, '1'..='9') => Some(TokenType::Float),
        //     (State::Int21, '0') => Some(TokenType::Float),
        //     (State::Int21, '1'..='9') => Some(TokenType::Float),
        //     (State::Int23, '0'..='9') => Some(TokenType::Float),
        //     (State::Int31, '0') => Some(TokenType::Float),
        //     (State::Int31, '1'..='9') => Some(TokenType::Float),
        //     (State::Int33, '0'..='9') => Some(TokenType::Float),
        //     _ => None,
        // }
    }
}

fn main() {
    let source: String =
        fs::read_to_string("sample.src").expect("Something went wrong reading the file");
    let mut lexer: Lexer = Lexer::new();
    let result = lexer.take(&source);
    println!("{:?}", result);
}
