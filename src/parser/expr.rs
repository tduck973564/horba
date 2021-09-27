use crate::scanner::token::Token;

pub(crate) trait Visitor<T> {
    fn visit(&mut self, expr: &Expr) -> T {
        match expr {
            Expr::Grouping(x) => self.visit_grouping(x),
            Expr::Binary(x) => self.visit_binary(x),
            Expr::Literal(x) => self.visit_literal(x),
            Expr::Unary(x) => self.visit_unary(x),
        }
    }
    fn visit_grouping(&self, grouping: &Grouping) -> T;
    fn visit_binary(&self, binary: &Binary) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
    fn visit_unary(&self, unary: &Unary) -> T;
}

pub(crate) enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Literal(Literal),
    Unary(Unary),
}

impl Expr {
    pub(crate) fn accept<T>(&mut self, mut visitor: Box<dyn Visitor<T>>) -> T {
        visitor.visit(self)
    }
}

// Expressions
pub(crate) enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

pub struct Grouping {
    pub(crate) expression: Box<Expr>,
}

pub struct Unary {
    pub(crate) operator: Token,
    pub(crate) expression: Box<Expr>,
}

pub struct Binary {
    pub(crate) left: Box<Expr>,
    pub(crate) operator: Token,
    pub(crate) right: Box<Expr>,
}

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
