use super::expr::{self, Binary, Expr, Grouping, Literal, Unary};
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

#[derive(Clone, Copy)]
struct AstPrinter;

impl expr::Visitor<String> for AstPrinter {
    fn visit_grouping_expr(&self, expr: Box<Grouping<String>>) -> String {
        self.parenthesize("group", vec![expr.expression])
    }

    fn visit_binary_expr(&self, expr: Box<Binary<String>>) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![expr.left, expr.right])
    }

    fn visit_literal_expr(&self, expr: Box<Literal>) -> String {
        match *expr {
            Literal::Number(x) => x.to_string(),
            Literal::String(x) => x,
            Literal::True => "true".to_string(),
            Literal::False => "false".to_string(),
            Literal::Null => "null".to_string(),
        }
    }

    fn visit_unary_expr(&self, expr: Box<Unary<String>>) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![expr.expression])
    }
}

impl AstPrinter {
    pub fn print(&self, expr: Box<dyn Expr<String>>) -> String {
        expr.accept(Box::new(*self))
    }
    fn parenthesize(&self, name: &str, exprs: Vec<Box<dyn Expr<String>>>) -> String {
        let mut string = String::new();

        string.push('(');
        string.push_str(name);
        for expr in exprs {
            let visitor_string = expr.accept(Box::new(*self));
            string.push(' ');
            string.push_str(&visitor_string);
        }
        string.push(')');
        string
    }
}

pub fn ast_test() {
    let expr = Binary {
        left: Box::new(Unary {
            operator: Token {
                token: TokenType::Minus,
                lexeme: "-".to_string(),
                line: 1,
            },
            expression: Box::new(Literal::Number(123.0)),
        }),
        operator: Token {
            token: TokenType::Star,
            lexeme: "*".to_string(),
            line: 1,
        },
        right: Box::new(Grouping {
            expression: Box::new(Literal::Number(45.67)),
        }),
    };
    println!("{}", AstPrinter {}.print(Box::new(expr)));
}
