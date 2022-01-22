pub mod token {
    pub enum TokenType {
        Id,
        Integer,
        Float,
        String,
    }

    pub struct Token {
        pub(crate) token_type: TokenType,
        pub(crate) lexeme: String,
        pub(crate) location: (i32, i32),
    }
}
