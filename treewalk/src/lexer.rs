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
    pub line: usize
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            tok_type: typ,
            lexeme: lexeme.clone(),
            literal: None,
            line: line
        }
    }

    pub fn string_literal(s: String, line: usize) -> Token {
        Token {
            tok_type: TokenType::String,
            lexeme: s.clone(),
            literal: Some(Literal::String(s.clone())),
            line: line
        }
    }

    pub fn eof(line: usize) -> Token {
        Token {
            tok_type: TokenType::Eof,
            lexeme: String::from(""),
            literal: None,
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

pub struct Scanner {
    source: String,
    source_chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.clone(),
            source_chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: vec!()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof(self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            _ => ()
        }
    }

    fn advance(&mut self) -> char {
        let res = self.source_chars[self.current];
        self.current += 1;
        res
    }

    fn add_token(&mut self, typ: TokenType) {
        let lexeme = String::from(self.source.get(self.start..self.current).
                                  expect("this should never happen 2"));
        self.tokens.push(Token::new(typ, lexeme, self.line));
    }
}
