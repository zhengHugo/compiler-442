use crate::token::InvalidTokenType;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct LexicalError {
    pub(crate) error_type: InvalidTokenType,
    pub(crate) invalid_lexeme: String,
    pub(crate) loc: (u32, u32),
}

impl Debug for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {},{}: {} cannot be recognized as a token",
            self.loc.0, self.loc.1, self.invalid_lexeme
        )
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lexical error: {}: \"{}\": line {}.",
            match self.error_type.clone() {
                InvalidTokenType::InvalidNumber => "Invalid number",
                InvalidTokenType::UnterminatedBlockCmt => "Unterminated block comment",
                InvalidTokenType::InvalidChar => "Invalid character",
                InvalidTokenType::Other => "Invalid token",
            },
            self.invalid_lexeme,
            self.loc.0
        )
    }
}

impl Error for LexicalError {}
