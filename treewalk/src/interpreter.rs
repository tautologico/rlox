#[derive(Debug, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    Boolean(bool),
    String(String)
}

use crate::ast::Expr;
use crate::ast::Literal;
use crate::ast::UnOp;
use crate::ast::BinOp;

pub fn eval(exp: &Expr) -> Value {
    match exp {
        Expr::Literal(l) => eval_literal(l),
        Expr::Grouping(e) => eval(e),
        Expr::Unary(op, e) => eval_unary(op, e),
        Expr::Binary(op, e1, e2) => eval_binary(op, e1, e2),
        _ => Value::Nil
    }
}

fn eval_literal(literal: &Literal) -> Value {
    match literal {
        Literal::Nil => Value::Nil,
        Literal::True => Value::Boolean(true),
        Literal::False => Value::Boolean(false),
        Literal::Number(n) => Value::Number(*n),
        Literal::String(s) => Value::String(s.to_string())   // may optimize to a move later
    }
}

fn eval_unary(op: &UnOp, e: &Expr) -> Value {
    match op {
        UnOp::Minus => minus(&eval(e)),
        UnOp::Not => negate(&eval(e))
    }
}

fn eval_binary(op: &BinOp, e1: &Expr, e2: &Expr) -> Value {
    match op {
        BinOp::Plus => Value::Nil,
        BinOp::Minus => Value::Nil,
        BinOp::Mult => Value::Nil,
        BinOp::Div => Value::Nil,
        BinOp::Gt => Value::Nil,
        BinOp::GtEqual => Value::Nil,
        BinOp::Lt => Value::Nil,
        BinOp::LtEqual => Value::Nil,
        BinOp::Equal => Value::Nil,
        BinOp::NotEqual => Value::Nil
    }
}

fn minus(v: &Value) -> Value {
    match v {
        Value::Number(n) => Value::Number(- *n),
        _ => panic!("Tried to invert sign of a non-numeric value: {:?}", v)
    }
}

fn negate(v: &Value) -> Value {
    // false and nil are "falsey", everything else is truthy
    match v {
        Value::Boolean(false) => Value::Boolean(true),
        Value::Nil => Value::Boolean(true),
        _ => Value::Boolean(false)
    }
}
