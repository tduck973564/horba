use super::expr::{ Expr, Visitor };
use crate::scanner::token_type::TokenType;
use std::any::Any;
use crate::parser::expr::{Binary, Comma, Grouping, Literal, Ternary, Unary};
use std::convert::TryInto;

#[derive(Clone)]
struct Interpreter;

impl Interpreter {
    fn evaluate(&self, mut expr: Box<Expr>) -> Box<dyn Any> {
        expr.accept(Box::new((*self).clone()))
    }
}

impl Visitor<Box<dyn Any>> for Interpreter {
    fn visit_grouping(&self, grouping: &mut Grouping) -> Box<dyn Any> {
        self.evaluate(grouping.expression)
    }

    fn visit_binary(&self, binary: &mut Binary) -> Box<dyn Any> {
        todo!()
    }

    fn visit_literal(&self, literal: &mut Literal) -> Box<dyn Any> {
        Box::new(literal.clone())
    }

    fn visit_unary(&self, unary: &mut Unary) -> Box<dyn Any> {
        let right = self.evaluate(unary.expression);

        match unary.operator.token {
            TokenType::Minus => return Box::new(-*(right.downcast::<f64>().unwrap())),
            _ => (),
        };
        Box::new(()) // unreachable
    }

    fn visit_ternary(&self, ternary: &mut Ternary) -> Box<dyn Any> {
        todo!()
    }

    fn visit_comma(&self, comma: &mut Comma) -> Box<dyn Any> {
        todo!()
    }
}