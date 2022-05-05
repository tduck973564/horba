use crate::error::{self, LogLevel};
use crate::parser::expr::*;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;
use std::mem::discriminant;

pub mod ast_printer;
pub mod expr;
pub mod interpreter;
pub mod runtime_error;

struct ParseError;

pub struct Parser {
    source: String,
    tokens: Vec<Token>,
    pub current: u32,
}

impl Parser {
    pub fn new(source: String, tokens: Vec<Token>) -> Parser {
        Parser {
            source,
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        if self.is_at_end() {
            return None;
        }
        match self.expression() {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.ternary();

        while self.cmp(&[TokenType::Comma]) {
            let next = self.ternary();
            expr = Ok(Expr::Comma(Comma {
                expr: Box::new(expr?),
                next: Box::new(next?),
            }));
        }

        expr
    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut condition = self.equality();

        if self.cmp(&[TokenType::Question]) {
            let if_true = self.expression()?;

            self.consume(
                TokenType::Colon,
                "Expect ':' after then branch of conditional expression.",
            )?;

            let if_false = self.expression()?;

            condition = Ok(Expr::Ternary(Ternary {
                condition: Box::new(condition?),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
            }))
        }

        condition
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();

        while self.cmp(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.peek(-1).clone();
            let right = self.comparison();
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator: operator.clone(),
                right: Box::new(right?),
            }));
        }

        expr
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();

        while self.cmp(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.peek(-1).clone();
            let right = self.term();
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator: operator.clone(),
                right: Box::new(right?),
            }));
        }

        expr
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();

        while self.cmp(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.peek(-1).clone();
            let right = self.factor();
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator: operator.clone(),
                right: Box::new(right?),
            }))
        }

        expr
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();

        while self.cmp(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.peek(-1).clone();
            println!("{}", operator.lexeme);
            let right = self.unary();
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator: operator.clone(),
                right: Box::new(right?),
            }))
        }

        expr
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.cmp(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.peek(-1).clone();
            let expression = self.unary();
            return Ok(Expr::Unary(Unary {
                operator,
                expression: Box::new(expression?),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match &self.advance().token {
            TokenType::False => Ok(Expr::Literal(Literal::False)),
            TokenType::True => Ok(Expr::Literal(Literal::True)),
            TokenType::Null => Ok(Expr::Literal(Literal::Null)),
            TokenType::Number(x) => Ok(Expr::Literal(Literal::Number(*x))),
            TokenType::String(x) => Ok(Expr::Literal(Literal::String(x.clone()))),
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
                Ok(Expr::Grouping(Grouping {
                    expression: Box::new(expr?),
                }))
            }
            _ => Err(self.error(self.peek(0), "Expect expression.")),
        }
    }

    // Helpers
    fn cmp(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        discriminant(&self.peek(0).token) == discriminant(token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.peek(-1)
    }

    fn is_at_end(&self) -> bool {
        self.current as usize == self.tokens.len() - 1
    }

    fn peek(&self, offset: i32) -> &Token {
        self.tokens
            .get((self.current as i32 + offset) as usize)
            .unwrap()
    }

    fn consume(&mut self, token: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&token) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(0), message))
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        if token.token != TokenType::Eof {
            error::report(
                token.line,
                token.column,
                LogLevel::Error,
                &format!("at '{}'", token.lexeme),
                message,
                &self.source,
            );
        } else {
            error::report(
                token.line,
                token.column,
                LogLevel::Error,
                "at end",
                message,
                &self.source,
            );
        }

        ParseError {}
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.peek(-1).token == TokenType::Semicolon {
                return;
            }

            match self.peek(0).token {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,

                _ => {}
            }

            self.advance();
        }
    }
}
