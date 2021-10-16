use super::expr::{self, *};
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

#[derive(Clone, Copy)]
pub struct AstPrinter;

impl expr::Visitor<String> for AstPrinter {
    fn visit_grouping(&self, expr: &mut Grouping) -> String {
        self.parenthesize("group", vec![&mut expr.expression])
    }

    fn visit_binary(&self, expr: &mut Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&mut expr.left, &mut expr.right])
    }

    fn visit_literal(&self, expr: &mut Literal) -> String {
        match expr {
            Literal::Number(x) => x.to_string(),
            Literal::String(x) => x.to_string(),
            Literal::True => "true".to_string(),
            Literal::False => "false".to_string(),
            Literal::Null => "null".to_string(),
        }
    }

    fn visit_unary(&self, expr: &mut Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&mut expr.expression])
    }

    fn visit_ternary(&self, expr: &mut Ternary) -> String {
        self.parenthesize(
            "ternary",
            vec![&mut expr.condition, &mut expr.if_true, &mut expr.if_false],
        )
    }

    fn visit_comma(&self, expr: &mut Comma) -> String {
        self.parenthesize("comma", vec![&mut expr.expr, &mut expr.next])
    }
}

impl AstPrinter {
    pub fn print(&self, mut expr: Expr) -> String {
        expr.accept(Box::new(*self))
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&mut Box<Expr>>) -> String {
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
    let expr = Expr::Binary(Binary {
        left: Box::new(Expr::Unary(Unary {
            operator: Token {
                token: TokenType::Minus,
                lexeme: "-".to_string(),
                line: 1,
                column: 1,
            },
            expression: Box::new(Expr::Literal(Literal::Number(123.0))),
        })),
        operator: Token {
            token: TokenType::Star,
            lexeme: "*".to_string(),
            line: 1,
            column: 1,
        },
        right: Box::new(Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal::Number(45.67))),
        })),
    });
    println!("{}", AstPrinter {}.print(expr));
}
