trait Visitor<T> {
    fn visit_grouping_expr(&self, expr: &Grouping) -> T;
    fn visit_binary_expr(&self, expr: &Binary) -> T;
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_unary_expr(&self, expr: &Unary) -> T;
}

trait Accept<T> {
    fn accept(&self, visitor: Box<dyn Visitor<T>>);
}

enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
}

enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

impl<T> Accept<T> for Literal {
    fn accept(&self, visitor: Box<dyn Visitor<T>>) {
        visitor.visit_literal_expr(self);
    }
}

struct Grouping {
    expression: Box<Expr>,
}

impl<T> Accept<T> for Grouping {
    fn accept(&self, visitor: Box<dyn Visitor<T>>) {
        visitor.visit_grouping_expr(self);
    }
}

struct Unary {
    operator: UnaryOperator,
    expression: Box<Expr>,
}

impl<T> Accept<T> for Unary {
    fn accept(&self, visitor: Box<dyn Visitor<T>>) {
        visitor.visit_unary_expr(self);
    }
}

struct Binary {
    left: Box<Expr>,
    operator: Operator,
    right: Box<Expr>,
}

impl<T> Accept<T> for Binary {
    fn accept(&self, visitor: Box<dyn Visitor<T>>) {
        visitor.visit_binary_expr(self);
    }
}

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

pub enum Stmt {}
