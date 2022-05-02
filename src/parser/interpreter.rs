// Ignore this comment; this is done
// This is garbage, why are we returning a language representation
// Rewrite this to return literal struct


use super::expr::{Expr, Visitor};
use crate::parser::expr::{Binary, Comma, Grouping, Literal, Ternary, Unary};
use crate::scanner::token_type::TokenType;

#[derive(Clone)]
pub(crate) struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &mut Expr) -> Literal {
        expr.accept::<Literal>(Box::new(self.clone()))
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_grouping(&self, grouping: &mut Grouping) -> Literal {
        self.evaluate(grouping.expression.as_mut())
    }

    fn visit_binary(&self, binary: &mut Binary) -> Literal {
        let left = self.evaluate(binary.left.as_mut());
        let right = self.evaluate(binary.right.as_mut());

        return match binary.operator.token {
            TokenType::Minus => Literal::Number(
                *(left.get().downcast::<f64>().unwrap()) - *(right.get().downcast::<f64>().unwrap())
            ),
            TokenType::Plus => Literal::Number(
                *(left.get().downcast::<f64>().unwrap()) + *(right.get().downcast::<f64>().unwrap())
            ),
            TokenType::Slash => Literal::Number(
                *(left.get().downcast::<f64>().unwrap()) / *(right.get().downcast::<f64>().unwrap())
            ),
            TokenType::Star => Literal::Number(
                *(left.get().downcast::<f64>().unwrap()) * *(right.get().downcast::<f64>().unwrap())
            ),
            TokenType::Greater =>
                if *(left.get().downcast::<f64>().unwrap()) > *(right.get().downcast::<f64>().unwrap()) {
                    Literal::True 
                } else { 
                    Literal::False 
                },
            TokenType::GreaterEqual => 
                if *(left.get().downcast::<f64>().unwrap()) >= *(right.get().downcast::<f64>().unwrap()) {
                    Literal::True 
                } else { 
                    Literal::False 
                },
            TokenType::Less => 
                if *(left.get().downcast::<f64>().unwrap()) < *(right.get().downcast::<f64>().unwrap()) {
                    Literal::True 
                } else { 
                    Literal::False 
                },
            TokenType::LessEqual => 
                if *(left.get().downcast::<f64>().unwrap()) <= *(right.get().downcast::<f64>().unwrap()) {
                    Literal::True 
                } else { 
                    Literal::False 
                },
            TokenType::BangEqual => 
                if left != right { Literal::True }
                else { Literal::False }
            TokenType::EqualEqual => 
                if left == right { Literal::True }
                else { Literal::False }

            _ => {
                // TODO: Runtime errors
            }
        }
    }

    fn visit_literal(&self, literal: &mut Literal) -> Literal {
        literal.clone()
    }

    fn visit_unary(&self, unary: &mut Unary) -> Literal {
        let right = self.evaluate(unary.expression.as_mut());

        match unary.operator.token {
            TokenType::Minus => Literal::Number(-*(right.get().downcast::<f64>().unwrap())),
            TokenType::Bang => Literal::negate(
                *(right.get().downcast::<Literal>().unwrap())
            ),
            _ => Literal::Null,
        }
    }

    fn visit_ternary(&self, ternary: &mut Ternary) -> Literal {
        todo!()
    }

    fn visit_comma(&self, comma: &mut Comma) -> Literal {
        todo!()
    }
}

