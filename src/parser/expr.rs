use crate::scanner::token::Token;
use std::fmt;

pub trait ExprVisitor<T> {
    fn visit_expr(&mut self, expr: &mut Expr) -> T {
        match expr {
            Expr::Grouping(x) => self.visit_grouping(x),
            Expr::Binary(x) => self.visit_binary(x),
            Expr::Literal(x) => self.visit_literal(x),
            Expr::Unary(x) => self.visit_unary(x),
            Expr::Ternary(x) => self.visit_ternary(x),
            Expr::Comma(x) => self.visit_comma(x),
        }
    }
    fn visit_grouping(&self, grouping: &mut Grouping) -> T;
    fn visit_binary(&self, binary: &mut Binary) -> T;
    fn visit_literal(&self, literal: &mut Literal) -> T;
    fn visit_unary(&self, unary: &mut Unary) -> T;
    fn visit_ternary(&self, ternary: &mut Ternary) -> T;
    fn visit_comma(&self, comma: &mut Comma) -> T;
}

#[derive(Debug)]
pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Literal(Literal),
    Unary(Unary),
    Ternary(Ternary),
    Comma(Comma),
}

impl Expr {
    pub fn accept<T>(&mut self, mut visitor: Box<dyn ExprVisitor<T>>) -> T {
        visitor.visit_expr(self)
    }
}

// Expressions

// In all honesty True and False probably should be in a separate enum, then put in Literal as Bool(Bool).
// They aren't supposed to be separate types.
// TODO
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

impl Literal {
    pub fn is_truthy(expr: &Literal) -> bool {
        use Literal::*;

        match expr {
            False | Null => false,
            _ => true,
        }
    }

    pub fn to_bool(expr: &Literal) -> Literal {
        use Literal::*;

        match Literal::is_truthy(expr) {
            true => True,
            false => False,
        }
    }
    pub fn negate(expr: &Literal) -> Literal {
        use Literal::*;

        match !Literal::is_truthy(expr) {
            true => True,
            false => False,
        }
    }
    pub fn type_name(expr: &Literal) -> String {
        // Do not use this to compare types, only for display 
        use Literal::*;

        match expr {
            Number(_) => "Number".to_string(),
            String(_) => "String".to_string(),
            True | False => "Bool".to_string(), 
            Null => "Null".to_string(),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Literal::*;

        write!(
            f,
            "{}",
            match &self {
                Number(x) => x.to_string(),
                String(x) => x.to_string(),
                True => "True".to_string(),
                False => "False".to_string(),
                Null => "Null".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Ternary {
    pub condition: Box<Expr>,
    pub if_true: Box<Expr>,
    pub if_false: Box<Expr>,
}

#[derive(Debug)]
pub struct Comma {
    pub expr: Box<Expr>,
    pub next: Box<Expr>,
}

// Operators
#[derive(Debug)]
#[allow(dead_code)]
enum Operator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
#[allow(dead_code)]
enum UnaryOperator {
    Negative,
    Not,
}
