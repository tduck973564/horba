use crate::error;
use crate::scanner::token::Token;
use std::fmt;

pub struct RuntimeError {
    pub token: Token,
    pub log_level: error::LogLevel,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for RuntimeError {
    fn report(&self, source: &str) {
        error::report(
            self.token.line,
            self.token.column,
            self.log_level,
            "",
            &self.message,
            source,
        )
    }
}
