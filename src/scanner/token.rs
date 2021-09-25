use super::token_type::TokenType;
use std::any::Any;
use std::fmt::{self, Display};

pub struct Token {
    token: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, line: u32) -> Token {
        Token {
            token,
            lexeme,
            line,
        }
    }

    pub fn get_literal(&self) -> Option<Box<dyn Any>> {
        match self.token.clone() {
            TokenType::Number(x) => Some(Box::new(x)),
            TokenType::String(x) => Some(Box::new(x)),
            _ => None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}\t{}", self.token, self.lexeme)
    }
}
