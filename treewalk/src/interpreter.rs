#[derive(Debug, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    Boolean(bool),
    String(String)
}

use crate::ast::Expr;

pub fn eval(exp: &Expr) -> Value {
    Nil
}
