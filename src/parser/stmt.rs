use crate::scanner::token::Token;
use std::fmt;

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, stmt: &mut Stmt) -> T {
        match stmt {
            _ => todo!() // TODO
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
}

impl Stmt {
    pub fn accept<T>(&mut self, mut visitor: Box<dyn StmtVisitor<T>>) -> T {
        visitor.visit_stmt(self)
    }
}