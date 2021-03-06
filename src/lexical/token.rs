use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ValidTokenType(ValidTokenType),
    InvalidTokenType(InvalidTokenType),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum ValidTokenType {
    Id,
    Integer,
    Float,
    Str,
    InlineCmt,
    BlockCmt,

    // operators
    Eq,
    NotEq,
    Lt,
    Gt,
    Leq,
    Geq,
    Plus,
    Minus,
    Mult,
    Div,
    Assign,
    Or,
    And,
    Not,
    OpenPar,
    ClosePar,
    OpenCuBr,
    CloseCuBr,
    OpenSqBr,
    CloseSqBr,
    Semi,
    Comma,
    Dot,
    Colon,
    ColonColon,
    Arrow,

    // keywords
    KwIf,
    KwThen,
    KwElse,
    KwInteger,
    KwFloat,
    KwVoid,
    KwPublic,
    KwPrivate,
    KwFunc,
    KwVar,
    KwStruct,
    KwWhile,
    KwRead,
    KwWrite,
    KwReturn,
    KwSelf,
    KwInherits,
    KwLet,
    KwImpl,
}

impl fmt::Display for ValidTokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidTokenType {
    InvalidNumber,
    InvalidChar,
    InvalidStr,
    UnterminatedBlockCmt,
    Other,
}

impl fmt::Display for InvalidTokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::ValidTokenType(valid) => valid.to_string(),
                TokenType::InvalidTokenType(invalid) => invalid.to_string(),
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) location: (u32, u32),
}

impl Token {
    pub fn get_valid_token_type(&self) -> Option<ValidTokenType> {
        match self.token_type.clone() {
            TokenType::ValidTokenType(valid_token_type) => Some(valid_token_type),
            TokenType::InvalidTokenType(_) => None,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}]",
            self.token_type, self.lexeme, self.location.0
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}]",
            self.token_type, self.lexeme, self.location.0
        )
    }
}
