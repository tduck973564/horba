use crate::scanner::token::Token;

pub(crate) trait Visitor<T> {
    fn visit_grouping_expr(&self, expr: Box<Grouping<T>>) -> T;
    fn visit_binary_expr(&self, expr: Box<Binary<T>>) -> T;
    fn visit_literal_expr(&self, expr: Box<Literal>) -> T;
    fn visit_unary_expr(&self, expr: Box<Unary<T>>) -> T;
}

pub(crate) trait Expr<T> {
    fn accept(self, visitor: Box<dyn Visitor<T>>) -> T;
}

// Expressions
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

impl<T> Expr<T> for Literal {
    fn accept(self, visitor: Box<dyn Visitor<T>>) -> T {
        visitor.visit_literal_expr(Box::new(self))
    }
}

pub struct Grouping<T> {
    pub(crate) expression: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Grouping<T> {
    fn accept(self, visitor: Box<dyn Visitor<T>>) -> T {
        visitor.visit_grouping_expr(Box::new(self))
    }
}

pub struct Unary<T> {
    pub(crate) operator: Token,
    pub(crate) expression: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Unary<T> {
    fn accept(self, visitor: Box<dyn Visitor<T>>) -> T {
        visitor.visit_unary_expr(Box::new(self))
    }
}

pub struct Binary<T> {
    pub(crate) left: Box<dyn Expr<T>>,
    pub(crate) operator: Token,
    pub(crate) right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Binary<T> {
    fn accept(self, visitor: Box<dyn Visitor<T>>) -> T {
        visitor.visit_binary_expr(Box::new(self))
    }
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
