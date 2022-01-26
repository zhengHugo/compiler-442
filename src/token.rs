use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum TokenType {
    Id,
    Integer,
    Float,
    Str,

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
