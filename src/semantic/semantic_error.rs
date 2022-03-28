use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Write;

pub struct SemanticError {
    error_type: SemanticErrType,
    message: String,
}

impl SemanticError {
    pub fn report_error(message: &str) -> Self {
        Self::report(SemanticErrType::Error, message)
    }
    pub fn report(error_type: SemanticErrType, message: &str) -> Self {
        let e = Self {
            error_type,
            message: message.to_string(),
        };
        let mut out_errors = File::create("resource/semantics/outsemanticerrors").unwrap();
        out_errors.write_all(format!("{}\n", e).as_bytes());
        println!("{}", e);
        e
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

impl Debug for SemanticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Error for SemanticError {}

pub enum SemanticErrType {
    Error,
    Warning,
}
