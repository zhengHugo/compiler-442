use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct LexicalError {
    pub(crate) invalid_lexeme: String,
    pub(crate) loc: (i32, i32),
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
            "Line {},{}: {} cannot be recognized as a token",
            self.loc.0, self.loc.1, self.invalid_lexeme
        )
    }
}

impl Error for LexicalError {}
