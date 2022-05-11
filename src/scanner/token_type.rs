// TODO: Get string representations of each TokenType in here, not in the scanner.
// HashMaps aren't as efficient, and this could be achieved with fmt::Display much more idiomatically.
// actually no fmt::display isn't a great idea, typemap crate maybe?
use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Colon,
    Slash,
    Star,
    Question,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    _Self,
    True,
    Let,
    While,

    Eof,
}
