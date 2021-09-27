use crate::parser::expr::*;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;
use std::mem::discriminant;

pub mod ast_printer;
pub mod expr;

struct Parser {
    tokens: Vec<Token>,
    current: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.cmp(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.peek(-1);
            let right = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: (*operator).clone(),
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.cmp(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.peek(-1);
            let right = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            });
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.cmp(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.peek(-1);
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            })
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.cmp(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.peek(-1);
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            })
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.cmp(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.peek(-1);
            let expression = self.unary();
            return Expr::Unary(Unary {
                operator: operator.clone(),
                expression: Box::new(expression),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        match &self.peek(0).token {
            TokenType::False => Expr::Literal(Literal::False),
            TokenType::True => Expr::Literal(Literal::True),
            TokenType::Null => Expr::Literal(Literal::Null),
            TokenType::Number(x) => Expr::Literal(Literal::Number(*x)),
            TokenType::String(x) => Expr::Literal(Literal::String(x.clone())),
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expected ')' after expression.");
                Expr::Grouping(Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => panic!("Panicked at primary, something is wrong with the operator precedence"),
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
        self.peek(0).token == TokenType::Eof
    }

    fn peek(&self, offset: i32) -> &Token {
        self.tokens
            .get((self.current as i32 + offset) as usize)
            .unwrap()
    }
}
