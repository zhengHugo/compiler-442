use crate::lexical::lexer_machine_impl::{LexerStateMachineImpl, State};
use crate::lexical::lexical_error::LexicalError;
use crate::lexical::token::{InvalidTokenType, Token, TokenType, ValidTokenType};
use rust_fsm::StateMachine;

pub struct Lexer {
    state_machine: StateMachine<LexerStateMachineImpl>,
    buffer: String,
    start_loc: (u32, u32),
    current_loc: (u32, u32),
    output_tokens: Vec<Token>,
    output_index: usize,

    // for (nested block comment)
    block_depth: usize,
    possibly_entering_block: bool, // encounter a '/', waiting for '*'
    possibly_exiting_block: bool,  // encounter a '*', waiting for '/'
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            state_machine: StateMachine::new(),
            buffer: String::from(""),
            start_loc: (1, 0),
            current_loc: (1, 0),
            output_tokens: vec![],
            output_index: 0,

            // handle block comments
            block_depth: 0,
            possibly_entering_block: false,
            possibly_exiting_block: false,
        }
    }

    /// input a source file and generate tokens
    pub fn read_source(&mut self, source: &str) {
        // for (i, c) in source.chars().enumerate() {
        for c in source.chars() {
            self.update_loc(&c);

            // handle block comment
            if self.block_depth > 0 {
                // we are in a block comment
                self.buffer.push(c);
                if self.possibly_entering_block {
                    // we just consumed a '/'
                    self.possibly_entering_block = false;
                    match c {
                        '*' => {
                            // enter a block
                            self.block_depth += 1
                        }
                        _ => {}
                    }
                } else if self.possibly_exiting_block {
                    self.possibly_exiting_block = false;
                    match c {
                        '/' => {
                            // exit a block
                            self.block_depth -= 1;
                            if self.block_depth == 0 {
                                // going out of block comment
                                let token = Token {
                                    token_type: TokenType::ValidTokenType(ValidTokenType::BlockCmt),
                                    lexeme: self.buffer.clone(),
                                    location: self.start_loc,
                                };
                                self.buffer.clear();
                                self.state_machine = StateMachine::new();
                                self.handle_finalized_token(token);
                            }
                        }
                        _ => {}
                    }
                } else {
                    match c {
                        '/' => {
                            self.possibly_entering_block = true;
                        }
                        '*' => {
                            self.possibly_exiting_block = true;
                        }
                        _ => {}
                    }
                }
                continue;
            }
            match (self.state_machine.state(), c) {
                (_, '\n' | '\r') => {
                    // handle line breaks as token boundaries
                    if !self.buffer.is_empty() {
                        // if buffer has something in it, finalize a token
                        let token_result = self.finalize_token(Some(&c));
                        self.handle_finalized_token(token_result);
                    }
                }
                (State::Str2 | State::InlineCmt, ' ' | '\t') => {
                    // is reading a string or in a inline comment. consume the space
                    self.next_char(&c);
                }
                (_, ' ' | '\t') => {
                    // handle spaces as token boundaries
                    if !(self.buffer.is_empty()) {
                        // if buffer has something in it, finalize a token
                        let token_result = self.finalize_token(Some(&c));
                        self.handle_finalized_token(token_result);
                    }
                }
                (State::Div, '*') => {
                    // go into block comment
                    self.next_char(&c);
                    self.block_depth += 1;
                }
                (_, _) => {
                    self.next_char(&c);
                }
            }
        }
        // when loop ends, flush out what's in the buffer
        if !(self.buffer.is_empty()) {
            let token_result = self.finalize_token(None);
            self.handle_finalized_token(token_result);
        }
    }

    /// Returns the next token from output tokens
    pub fn next_token(&mut self) -> Option<Token> {
        if self.output_index < self.output_tokens.len() {
            let result_token = self.output_tokens[self.output_index].clone();
            self.output_index += 1;
            return Some(result_token);
        }
        None
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.output_tokens.clone()
    }

    // push a token into the result token vector, possibly giving a lexical error
    fn handle_finalized_token(&mut self, token: Token) -> Option<LexicalError> {
        match token.token_type {
            TokenType::ValidTokenType(_) => {
                println!("{}", token);
                self.output_tokens.push(token);
                None
            }
            TokenType::InvalidTokenType(ref invalid_type) => {
                let e = LexicalError {
                    error_type: invalid_type.clone(),
                    invalid_lexeme: token.lexeme.clone(),
                    loc: token.location,
                };
                println!("{}", token);
                self.output_tokens.push(token);
                println!("{}", e);
                Some(e)
            }
        }
    }

    // input the next char into the lexical
    fn next_char(&mut self, input: &char) {
        let consumed_result = self.state_machine.consume(input);
        match consumed_result {
            Ok(_output) => {
                // transition success
                self.buffer.push(input.clone());
            }
            Err(_e) => {
                // if transition error happens,
                // 1. finalize the last token
                let token_result = self.finalize_token(Some(input));
                let some_error = self.handle_finalized_token(token_result);
                // 2. if it is not the first character causing the error, consume the current char
                match some_error {
                    Some(e) if e.error_type == InvalidTokenType::InvalidChar => {}
                    _ => self.next_char(input),
                }
            }
        }
    }

    /// Return a token from the lexical in current state, and reset the state machine
    fn finalize_token(&mut self, input: Option<&char>) -> Token {
        if self.block_depth > 0 {
            // try to finalize an unterminated block comment
            let token = Token {
                token_type: TokenType::InvalidTokenType(InvalidTokenType::UnterminatedBlockCmt),
                lexeme: self.buffer.clone(),
                location: self.start_loc,
            };
            self.buffer.clear();
            self.state_machine = StateMachine::new();
            return token;
        }
        match LexerStateMachineImpl::state_to_token_type(&self.state_machine.state()) {
            TokenType::ValidTokenType(valid_token_type) => {
                let result = Token {
                    token_type: TokenType::ValidTokenType(valid_token_type),
                    lexeme: self.buffer.clone(),
                    location: self.start_loc,
                };
                self.buffer.clear();
                self.state_machine = StateMachine::from_state(State::Start);
                result
            }
            TokenType::InvalidTokenType(invalid_token_type) => {
                let result = match invalid_token_type {
                    InvalidTokenType::InvalidChar => Token {
                        token_type: TokenType::InvalidTokenType(InvalidTokenType::InvalidChar),
                        lexeme: input
                            .expect("Try to create a InvalidChar error but missing input char")
                            .clone()
                            .to_string(),
                        location: self.start_loc,
                    },
                    _ => Token {
                        token_type: TokenType::InvalidTokenType(invalid_token_type),
                        lexeme: self.buffer.clone(),
                        location: self.start_loc,
                    },
                };
                self.buffer.clear();
                self.state_machine = StateMachine::from_state(State::Start);
                result
            }
        }
    }

    fn update_loc(&mut self, c: &char) {
        if matches!(self.state_machine.state(), State::Start) {
            // starting with a new token
            self.start_loc = self.current_loc;
        }
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
