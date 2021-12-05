// This is garbage, why are we returning a language representation
// Rewrite this to return literal struct

use super::expr::{Expr, Visitor};
use crate::parser::expr::{Binary, Comma, Grouping, Literal, Ternary, Unary};
use crate::scanner::token_type::TokenType;
use std::any::{Any, TypeId};

// Rewrite this bit
/*
#[derive(Clone)]
pub(crate) struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &mut Expr) -> Literal {
        expr.accept::<Literal>(Box::new(self.clone()))
    }

    fn is_truthy(expr: Literal) -> bool {
        !matches!(expr, Literal::Null | Literal::False)
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_grouping(&self, grouping: &mut Grouping) -> Literal {
        self.evaluate(grouping.expression.as_mut())
    }

    fn visit_binary(&self, binary: &mut Binary) -> Literal {
        let left = self.evaluate(binary.left.as_mut());
        let right = self.evaluate(binary.right.as_mut());

        match binary.operator.token {
            TokenType::Minus => {
                Box::new(*(left.downcast::<f64>().unwrap()) - *(right.downcast::<f64>().unwrap()))
            }
        }
    }

    fn visit_literal(&self, literal: &mut Literal) -> Literal {
        Box::new(literal.clone())
    }

    fn visit_unary(&self, unary: &mut Unary) -> Literal {
        let right = self.evaluate(unary.expression.as_mut());

        match unary.operator.token {
            TokenType::Minus => return Box::new(-*(right.downcast::<f64>().unwrap())),
            TokenType::Bang => Box::new(!Interpreter::is_truthy(
                *(right.downcast::<Literal>().unwrap()),
            )),
            _ => Box::new(()),
        }
    }

    fn visit_ternary(&self, ternary: &mut Ternary) -> Literal {
        todo!()
    }

    fn visit_comma(&self, comma: &mut Comma) -> Literal {
        todo!()
    }
}
*/
