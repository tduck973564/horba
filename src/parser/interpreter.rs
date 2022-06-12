use super::expr::{Expr, ExprVisitor};
use super::runtime_error::RuntimeError;
use crate::error::Error;
use crate::error::LogLevel;
use crate::parser::expr::{Binary, Comma, Grouping, Literal, Ternary, Unary};
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

#[derive(Clone)]
pub struct Interpreter;

// This is how we get the enum into something Rust can do arithmetic on.
// Trust me, it's better this way.
// Long and verbose but not cursed.

fn expected_type_msg(expected: &str, got: &str) -> String {
    format!("Expected {} type, got {}.", expected, got)
}

struct LiteralWithToken(Literal, Token);

impl TryFrom<LiteralWithToken> for f64 {
    type Error = RuntimeError;

    fn try_from(value: LiteralWithToken) -> Result<Self, Self::Error> {
        use Literal::*;

        match value.0 {
            Number(x) => Ok(x),
            x => Err(RuntimeError {
                token: value.1,
                log_level: LogLevel::Error,
                message: expected_type_msg("Number", &Literal::type_name(&x)),
            }),
        }
    }
}

impl TryFrom<LiteralWithToken> for String {
    type Error = RuntimeError;

    fn try_from(value: LiteralWithToken) -> Result<Self, Self::Error> {
        use Literal::*;

        match value.0 {
            String(x) => Ok(x),
            x => Err(RuntimeError {
                token: value.1,
                log_level: LogLevel::Error,
                message: expected_type_msg("String", &Literal::type_name(&x)),
            }),
        }
    }
}

impl TryFrom<LiteralWithToken> for bool {
    type Error = RuntimeError;

    fn try_from(value: LiteralWithToken) -> Result<Self, Self::Error> {
        use Literal::*;

        match value.0 {
            True => Ok(true),
            False => Ok(false),
            x => Err(RuntimeError {
                token: value.1,
                log_level: LogLevel::Error,
                message: expected_type_msg("Bool", &Literal::type_name(&x)),
            }),
        }
    }
}

impl Interpreter {
    pub fn interpret(&self, mut expr: Expr, source: &str) -> Result<(), ()> {
        let evaluated = self.evaluate(&mut expr);

        match evaluated {
            Ok(x) => {
                println!("{}", x);
                Ok(())
            }
            Err(e) => {
                e.report(source);
                Err(())
            }
        }
    }
    fn evaluate(&self, expr: &mut Expr) -> Result<Literal, RuntimeError> {
        expr.accept::<Result<Literal, RuntimeError>>(Box::new(self.clone()))
    }
}

impl ExprVisitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_grouping(&self, grouping: &mut Grouping) -> Result<Literal, RuntimeError> {
        self.evaluate(grouping.expression.as_mut())
    }

    fn visit_binary(&self, binary: &mut Binary) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(binary.left.as_mut())?;
        let right = self.evaluate(binary.right.as_mut())?;

        return match binary.operator.token {
            TokenType::Minus => Ok(Literal::Number(
                f64::try_from(LiteralWithToken(left, binary.operator.clone()))?
                    - f64::try_from(LiteralWithToken(right, binary.operator.clone()))?,
            )),
            TokenType::Plus => Ok(Literal::Number(
                f64::try_from(LiteralWithToken(left, binary.operator.clone()))?
                    + f64::try_from(LiteralWithToken(right, binary.operator.clone()))?,
            )),
            TokenType::Slash => Ok(Literal::Number(
                f64::try_from(LiteralWithToken(left, binary.operator.clone()))?
                    / f64::try_from(LiteralWithToken(right, binary.operator.clone()))?,
            )),
            TokenType::Star => Ok(Literal::Number(
                f64::try_from(LiteralWithToken(left, binary.operator.clone()))?
                    * f64::try_from(LiteralWithToken(right, binary.operator.clone()))?,
            )),
            TokenType::Greater => {
                if f64::try_from(LiteralWithToken(left, binary.operator.clone()))?
                    > f64::try_from(LiteralWithToken(right, binary.operator.clone()))?
                {
                    Ok(Literal::True)
                } else {
                    Ok(Literal::False)
                }
            }
            TokenType::GreaterEqual => {
                if f64::try_from(LiteralWithToken(left, binary.operator.clone()))?
                    >= f64::try_from(LiteralWithToken(right, binary.operator.clone()))?
                {
                    Ok(Literal::True)
                } else {
                    Ok(Literal::False)
                }
            }
            TokenType::Less => {
                if f64::try_from(LiteralWithToken(left, binary.operator.clone()))?
                    < f64::try_from(LiteralWithToken(right, binary.operator.clone()))?
                {
                    Ok(Literal::True)
                } else {
                    Ok(Literal::False)
                }
            }
            TokenType::LessEqual => {
                if f64::try_from(LiteralWithToken(left, binary.operator.clone()))?
                    <= f64::try_from(LiteralWithToken(right, binary.operator.clone()))?
                {
                    Ok(Literal::True)
                } else {
                    Ok(Literal::False)
                }
            }
            TokenType::BangEqual => {
                if left != right {
                    Ok(Literal::True)
                } else {
                    Ok(Literal::False)
                }
            }
            TokenType::EqualEqual => {
                if left == right {
                    Ok(Literal::True)
                } else {
                    Ok(Literal::False)
                }
            }

            _ => {
                return Err(RuntimeError {
                    token: binary.operator.clone(),
                    log_level: LogLevel::Error,
                    message: "Expected two numbers on each side of the operator.".to_string(),
                })
            }
        };
    }

    fn visit_literal(&self, literal: &mut Literal) -> Result<Literal, RuntimeError> {
        Ok(literal.clone())
    }

    fn visit_unary(&self, unary: &mut Unary) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(unary.expression.as_mut())?;

        match unary.operator.token {
            TokenType::Minus => Ok(Literal::Number(f64::try_from(LiteralWithToken(
                right,
                unary.operator.clone(),
            ))?)),
            TokenType::Bang => Ok(Literal::negate(&right)),
            _ => Ok(Literal::Null), // unreachable
        }
    }

    fn visit_ternary(&self, ternary: &mut Ternary) -> Result<Literal, RuntimeError> {
        let condition = self.evaluate(ternary.condition.as_mut())?;
        let if_true = self.evaluate(ternary.if_true.as_mut())?;
        let if_false = self.evaluate(ternary.if_false.as_mut())?;

        match Literal::is_truthy(&condition) {
            true => Ok(if_true),
            false => Ok(if_false),
        }
    }

    fn visit_comma(&self, comma: &mut Comma) -> Result<Literal, RuntimeError> {
        self.evaluate(comma.expr.as_mut())?;
        Ok(self.evaluate(comma.next.as_mut())?)
    }
}
