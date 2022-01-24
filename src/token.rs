use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum TokenType {
    Id,
    Integer,
    Float,
    Str,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) location: (i32, i32),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.token_type, self.lexeme)
    }
}

// impl Clone for Token {
//     fn clone(&self) -> Self {
//         Token {
//             token_type: self.token_type.,
//             lexeme: self.lexeme.clone(),
//             location: self.location,
//         }
//     }
// }
