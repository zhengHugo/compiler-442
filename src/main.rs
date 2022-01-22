mod token;

extern crate core;

use crate::token::token::Token;
use crate::token::token::TokenType;
use rust_fsm::{state_machine, StateMachine, StateMachineImpl};
use std::fs;

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
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            state_machine: StateMachine::new(),
            buffer: String::from(""),
            current_loc: (0, 0),
        }
    }

    pub fn run(&mut self, source: &str) -> Vec<Token> {
        let mut result: Vec<Token> = vec![];
        for (i, c) in source.chars().enumerate() {
            self.update_loc(&c);
            match c {
                ' ' | '\t' | '\n' => {
                    if !(self.buffer.is_empty()) {
                        match self.state_machine.state() {
                            // getting a space when reading a string. Continue
                            State::Str2 => {
                                self.state_machine.consume(&c).expect("Invalid input char!")
                            }

                            // otherwise, get output from the state machine and reset the machine
                            State::Id2 => result.push(Token {
                                token_type: TokenType::Id,
                                lexeme: self.buffer.clone(),
                                location: self.current_loc,
                            }),
                            State::Str3 => result.push(Token {
                                token_type: TokenType::String,
                                lexeme: self.buffer.clone(),
                                location: self.current_loc,
                            }),
                            State::Int12 | State::Int13 => result.push(Token {
                                token_type: TokenType::Integer,
                                lexeme: self.buffer.clone(),
                                location: self.current_loc,
                            }),
                            State::Frac13
                            | State::Frac14
                            | State::Int22
                            | State::Int23
                            | State::Int32
                            | State::Int33 => result.push(Token {
                                token_type: TokenType::Float,
                                lexeme: self.buffer.clone(),
                                location: self.current_loc,
                            }),
                            _ => panic!("Invalid token error"), // TODO
                        };
                    } else {
                    }
                }
                _ => {
                    // TODO: read normal characters
                    let output = self.state_machine.consume(&c).expect("MESSAGE TODO");
                }
            }
        }
        result
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

    fn consume(&mut self, c: &char) {}
}

struct LexerStateMachineImpl {
    buffer: String,
}

impl StateMachineImpl for LexerStateMachineImpl {
    type Input = char;
    type State = State;
    type Output = Token;

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
            (_, _) => None,
        }
    }

    fn output(state: &Self::State, input: &Self::Input) -> Option<Self::Output> {
        todo!()
    }
}

fn main() {
    static SOURCE: String =
        fs::read_to_string("sample.src").expect("Something went wrong reading the file");
    let lexer: Lexer = Lexer::new();
}
