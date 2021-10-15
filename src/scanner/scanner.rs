use super::token::Token;
use super::token_type::TokenType;
use crate::error;

use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    column: u32,
}

fn is_ident_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

lazy_static! {
    static ref KEYWORDS_TABLE: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fn", TokenType::Fn);
        m.insert("if", TokenType::If);
        m.insert("null", TokenType::Null);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("self", TokenType::_Self);
        m.insert("true", TokenType::True);
        m.insert("let", TokenType::Let);
        m.insert("while", TokenType::While);
        m.insert("inherits", TokenType::Inherits);
        m
    };
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            self.line,
            self.column,
        ));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.cmp('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.cmp('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.cmp('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.cmp('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => match self.peek(0) {
                '/' => {
                    self.advance();
                    while self.peek(0) != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                '*' => {
                    self.advance();
                    self.block_comment();
                }
                _ => {
                    self.advance();
                    self.add_token(TokenType::Slash);
                }
            },
            '"' => self.string(),
            '\n' => self.new_line(),
            x if x.is_whitespace() => (),
            x if x.is_digit(10) => self.number(),
            x if is_ident_char(x) => self.identifier(),
            x => {
                error::error(
                    self.line,
                    self.column,
                    &format!("Unexpected character: {}", x),
                );
            }
        }
    }

    fn string(&mut self) {
        while self.peek(0) != '"' && !self.is_at_end() {
            if self.peek(0) == '\n' {
                self.new_line()
            }
            self.advance();
        }

        if self.is_at_end() {
            error::error(self.line, self.column, "Unterminated string.");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(value));
    }

    fn number(&mut self) {
        while self.peek(0).is_digit(10) {
            self.advance();
        }

        if self.peek(0) == '.' && self.peek(1).is_digit(10) {
            self.advance();
            while self.peek(0).is_digit(10) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number(
            str::parse(&self.source[self.start..self.current]).unwrap(),
        ))
    }

    fn identifier(&mut self) {
        while is_ident_char(self.peek(0)) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token = match KEYWORDS_TABLE.get(text) {
            Some(x) => x,
            None => &TokenType::Identifier,
        };
        self.add_token(token.clone());
    }

    fn block_comment(&mut self) {
        while !(self.peek(0) == '*' && self.peek(1) == '/' && self.is_at_end()) {
            if self.peek(0) == '\n' {
                self.new_line();
            }
            self.advance();
        }
        self.advance();
        self.advance();
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let out = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        out
    }

    fn add_token(&mut self, token: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token, text.to_string(), self.line, self.column));
    }

    fn peek(&self, lookahead: usize) -> char {
        self.source
            .chars()
            .nth(self.current + lookahead)
            .unwrap_or('\0')
    }

    fn cmp(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.peek(0) != expected) {
            return false;
        }
        self.current += 1;
        true
    }
    fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}
