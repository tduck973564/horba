use crate::scanner::token::Token;

pub trait Visitor<T> {
    fn visit(&mut self, expr: &mut Expr) -> T {
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
    pub fn accept<T>(&mut self, mut visitor: Box<dyn Visitor<T>>) -> T {
        visitor.visit(self)
    }
}

// Expressions
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
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

#[derive(Debug)]
// Operators
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

enum UnaryOperator {
    Negative,
    Not,
}
