use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

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
    Identifier(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            tok_type: typ,
            lexeme: lexeme.clone(),
            literal: None,
            line: line,
        }
    }

    pub fn string_literal(s: String, line: usize) -> Token {
        Token {
            tok_type: TokenType::String,
            lexeme: s.clone(),
            literal: Some(Literal::String(s.clone())),
            line: line,
        }
    }

    pub fn eof(line: usize) -> Token {
        Token {
            tok_type: TokenType::Eof,
            lexeme: String::from(""),
            literal: None,
            line: line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            None => write!(f, "{:?} {}", self.tok_type, self.lexeme),
            Some(l) => write!(f, "{:?} {} {:?}", self.tok_type, self.lexeme, l),
        }
    }
}

pub struct Scanner {
    source: String,
    source_chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>,
    pub had_error: bool,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            source_chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            had_error: false,
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
        // this should work because scan_token is only called after checking is_at_end
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '/' => self.comment_or_slash(),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_alternatives('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_alternatives('=', TokenType::EqualEqual, TokenType::Equal),
            '>' => self.add_alternatives('=', TokenType::GreaterEqual, TokenType::Greater),
            '<' => self.add_alternatives('=', TokenType::LessEqual, TokenType::Less),
            c if c.is_whitespace() => self.process_whitespace(c),
            _ => self.error(format!("Unrecognized character: {}", c)),
        }
    }

    fn advance(&mut self) -> char {
        let res = self.source_chars[self.current];
        self.current += 1;
        res
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source_chars[self.current])
        }
    }

    fn add_token(&mut self, typ: TokenType) {
        let lexeme = String::from(
            self.source
                .get(self.start..self.current)
                .expect("this should never happen 2"),
        );
        self.tokens.push(Token::new(typ, lexeme, self.line));
    }

    fn error(&mut self, message: String) {
        println!("Error in line {}: {}", self.line, message);
        self.had_error = true;
    }

    fn match_next(&mut self, c: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source_chars[self.current] != c {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_alternatives(&mut self, next: char, typ_match: TokenType, typ_not_match: TokenType) {
        let does_match = self.match_next(next);
        self.add_token(if does_match { typ_match } else { typ_not_match });
    }

    fn comment_or_slash(&mut self) {
        if self.match_next('/') {
            while let Some(c) = self.peek() {
                if c == '\n' {
                    break;
                } else {
                    self.advance();
                }
            }
            // let mut c = self.advance();
            // while c != '\n' && !self.is_at_end() {
            //     c = self.advance();
            // }
        } else {
            self.add_token(TokenType::Slash);
        }
    }

    fn process_whitespace(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
        }
    }
}

// tests
#[test]
fn test_operators() {
    let mut scanner = Scanner::new("(/*){ ;+\t -}!({.,.!=<>====!})");

    scanner.scan_tokens();

    let types = vec![
        TokenType::LeftParen,
        TokenType::Slash,
        TokenType::Star,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::Semicolon,
        TokenType::Plus,
        TokenType::Minus,
        TokenType::RightBrace,
        TokenType::Bang,
        TokenType::LeftParen,
        TokenType::LeftBrace,
        TokenType::Dot,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::BangEqual,
        TokenType::Less,
        TokenType::GreaterEqual,
        TokenType::EqualEqual,
        TokenType::Equal,
        TokenType::Bang,
        TokenType::RightBrace,
        TokenType::RightParen,
        TokenType::Eof,
    ];

    let mut typ_it = types.iter();
    for tok in scanner.tokens {
        let typ = typ_it.next().expect("q?");
        assert_eq!(tok.tok_type, *typ);
    }
}
