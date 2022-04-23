use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Number(i64),
    String(String),
    Identifier(String)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: i64
}

impl Token {
    pub fn string_literal(s: String, line: i64) -> Token {
        Token {
            tok_type: TokenType::String,
            lexeme: s.clone(),
            literal: Some(Literal::String(s.clone())),
            line: line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            None => write!(f, "{:?} {}", self.tok_type, self.lexeme),
            Some(l) => write!(f, "{:?} {} {:?}", self.tok_type, self.lexeme, l)
        }
    }
}
