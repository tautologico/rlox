use std::fmt;

use crate::lexer::TokenType;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "\"{}\"", &s),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    Minus,
    Not,
}

impl UnOp {
    fn from_token_type(toktyp: TokenType) -> Option<UnOp> {
        match toktyp {
            TokenType::Minus => Some(UnOp::Minus),
            TokenType::Bang => Some(UnOp::Not),
            _ => None
        }
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Minus => write!(f, "neg"),  // neg is unambiguous in relation to -
            UnOp::Not => write!(f, "not")     // !
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Equal,
    NotEqual,
    Lt,
    LtEqual,
    Gt,
    GtEqual,
    Plus,
    Minus,
    Mult,
    Div,
}

impl BinOp {
    fn from_token_type(toktyp: TokenType) -> Option<BinOp> {
        match toktyp {
            TokenType::EqualEqual => Some(BinOp::Equal),
            TokenType::BangEqual => Some(BinOp::NotEqual),
            TokenType::Less => Some(BinOp::Lt),
            TokenType::LessEqual => Some(BinOp::LtEqual),
            TokenType::Greater => Some(BinOp::Gt),
            TokenType::GreaterEqual => Some(BinOp::GtEqual),
            TokenType::Plus => Some(BinOp::Plus),
            TokenType::Minus => Some(BinOp::Minus),
            TokenType::Slash => Some(BinOp::Div),
            TokenType::Star => Some(BinOp::Mult),
            _ => None
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Equal => write!(f, "=="),
            BinOp::NotEqual => write!(f, "!="),
            BinOp::Lt => write!(f, "<"),
            BinOp::LtEqual => write!(f, "<="),
            BinOp::Gt => write!(f, ">"),
            BinOp::GtEqual => write!(f, ">="),
            BinOp::Plus => write!(f, "+"),
            BinOp::Minus => write!(f, "-"),
            BinOp::Mult => write!(f, "*"),
            BinOp::Div => write!(f, "/")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
}

impl Expr {
    pub fn number_literal(n: f64) -> Expr {
        Expr::Literal(Literal::Number(n))
    }

    pub fn string_literal(s: &str) -> Expr {
        Expr::Literal(Literal::String(s.to_string()))
    }

    pub fn true_literal() -> Expr {
        Expr::Literal(Literal::True)
    }

    pub fn false_literal() -> Expr {
        Expr::Literal(Literal::False)
    }

    pub fn nil_literal() -> Expr {
        Expr::Literal(Literal::Nil)
    }

    pub fn group(e: Expr) -> Expr {
        Expr::Grouping(Box::new(e))
    }

    pub fn binary(op: BinOp, e1: Expr, e2: Expr) -> Expr {
        Expr::Binary(op, Box::new(e1), Box::new(e2))
    }

    pub fn binary_from_token(op_tok: TokenType, e1: Expr, e2: Expr) -> Expr {
        let op = match BinOp::from_token_type(op_tok) {
            Some(bop) => bop,
            None => panic!("Unexpected token type for binary operator!")
        };
        Expr::binary(op, e1, e2)
    }

    pub fn unary(op: UnOp, e: Expr) -> Expr {
        Expr::Unary(op, Box::new(e))
    }

    pub fn unary_from_token(op_tok: TokenType, e: Expr) -> Expr {
        let op = match UnOp::from_token_type(op_tok) {
            Some(uop) => uop,
            None => panic!("Unexpected token type for unary operator!")
        };
        Expr::unary(op, e)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(l) => write!(f, "{}", l),
            Expr::Unary(op, exp) => write!(f, "({} {})", op, exp),
            Expr::Binary(op, e1, e2) => write!(f, "({} {} {})", op, e1, e2),
            Expr::Grouping(e) => write!(f, "(group {})", e)
        }
    }
}


// tests

#[test]
fn test_ast_display() {
    let lit_num1 = Literal::Number(4.0);
    let lit_num2 = Literal::Number(123.45);
    let lit_str = Literal::String("this is a string".to_string());

    assert_eq!(format!("{}", lit_num1), "4");
    assert_eq!(format!("{}", lit_num2), "123.45");
    assert_eq!(format!("{}", lit_str), "\"this is a string\"");

    // build a larger expression
    let exp = Expr::binary(BinOp::Mult,
                           Expr::unary(UnOp::Minus, Expr::number_literal(123.0)),
                           Expr::group(Expr::number_literal(45.67)));

    assert_eq!(format!("{}", exp), "(* (neg 123) (group 45.67))");
}
