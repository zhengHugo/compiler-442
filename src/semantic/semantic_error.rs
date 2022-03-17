use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct SemanticError {
    error_type: SemanticErrType,
    message: String,
}

impl SemanticError {
    pub fn report(error_type: SemanticErrType, message: String) {
        println!(
            "{}",
            Self {
                error_type,
                message
            }
        )
    }
}

impl Debug for SemanticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "semantic {}: {}",
            match self.error_type {
                SemanticErrType::Error => "error",
                SemanticErrType::Warning => "warning",
            },
            self.message
        )
    }
}

impl Error for SemanticError {}

pub enum SemanticErrType {
    Error,
    Warning,
}
