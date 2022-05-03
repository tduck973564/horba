// Ignore this comment; this is done
// This is garbage, why are we returning a language representation
// Rewrite this to return literal struct

use super::runtime_error::RuntimeError;
use super::expr::{Expr, Visitor};
use crate::parser::expr::{Binary, Comma, Grouping, Literal, Ternary, Unary};
use crate::scanner::token_type::TokenType;

#[derive(Clone)]
pub(crate) struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &mut Expr) -> Result<Literal, RuntimeError> {
        Ok(expr.accept::<Result<Literal, RuntimeError>>(Box::new(self.clone())))
    }
}

impl Visitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_grouping(&self, grouping: &mut Grouping) -> Result<Literal, RuntimeError> {
        self.evaluate(grouping.expression.as_mut())
    }

    fn visit_binary(&self, binary: &mut Binary) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(binary.left.as_mut());
        let right = self.evaluate(binary.right.as_mut());

        return match binary.operator.token {
            TokenType::Minus => Ok(Literal::Number(
                *(left.get().downcast::<f64>().unwrap()) - *(right.get().downcast::<f64>().unwrap())
            )),
            TokenType::Plus => Ok(Literal::Number(
                *(left.get().downcast::<f64>().unwrap()) + *(right.get().downcast::<f64>().unwrap())
            )),
            TokenType::Slash => Ok(Literal::Number(
                *(left.get().downcast::<f64>().unwrap()) / *(right.get().downcast::<f64>().unwrap())
            )),
            TokenType::Star => Ok(Literal::Number(
                *(left.get().downcast::<f64>().unwrap()) * *(right.get().downcast::<f64>().unwrap())
            )),
            TokenType::Greater =>
                if *(left.get().downcast::<f64>().unwrap()) > *(right.get().downcast::<f64>().unwrap()) {
                    Ok(Literal::True) 
                } else { 
                    Ok(Literal::False) 
                },
            TokenType::GreaterEqual => 
                if *(left.get().downcast::<f64>().unwrap()) >= *(right.get().downcast::<f64>().unwrap()) {
                    Ok(Literal::True) 
                } else { 
                    Ok(Literal::False) 
                },
            TokenType::Less => 
                if *(left.get().downcast::<f64>().unwrap()) < *(right.get().downcast::<f64>().unwrap()) {
                    Ok(Literal::True) 
                } else { 
                    Ok(Literal::False) 
                },
            TokenType::LessEqual => 
                if *(left.get().downcast::<f64>().unwrap()) <= *(right.get().downcast::<f64>().unwrap()) {
                    Ok(Literal::True) 
                } else { 
                    Ok(Literal::False) 
                },
            TokenType::BangEqual => 
                if left != right { Ok(Literal::True) }
                else { Ok(Literal::False) }
            TokenType::EqualEqual => 
                if left == right { Ok(Literal::True) }
                else { Ok(Literal::False) }

            _ => {
                // TODO: Runtime errors
            }
        }
    }

    fn visit_literal(&self, literal: &mut Literal) -> Result<Literal, RuntimeError> {
        Ok(literal.clone())
    }

    fn visit_unary(&self, unary: &mut Unary) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(unary.expression.as_mut());

        match unary.operator.token {
            TokenType::Minus => Ok(Literal::Number(-*(right.get().downcast::<f64>().unwrap()))),
            TokenType::Bang => Ok(Literal::negate(
                *(right.get().downcast::<Literal>().unwrap())
            )),
            _ => Ok(Literal::Null),
        }
    }

    fn visit_ternary(&self, ternary: &mut Ternary) -> Result<Literal, RuntimeError> {
        todo!()
    }

    fn visit_comma(&self, comma: &mut Comma) -> Result<Literal, RuntimeError> {
        todo!()
    }
}

