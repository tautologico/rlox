use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
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
            lexeme: s.clone(), // TODO: the lexeme should include quotes
            literal: Some(Literal::String(s.clone())),
            line: line,
        }
    }

    pub fn number_literal(val: f64, lex: &str, line: usize) -> Token {
        Token {
            tok_type: TokenType::Number,
            lexeme: lex.to_string(),
            literal: Some(Literal::Number(val)),
            line: line,
        }
    }

    pub fn identifier(id: &str, line: usize) -> Token {
        Token {
            tok_type: TokenType::Identifier,
            lexeme: id.to_string(),
            literal: Some(Literal::Identifier(id.to_string())),
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
    reserved_words: HashMap<String, TokenType>,
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
            reserved_words: Scanner::build_reserved_word_map(),
        }
    }

    fn build_reserved_word_map() -> HashMap<String, TokenType> {
        HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("fun".to_string(), TokenType::Fun),
            ("for".to_string(), TokenType::For),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ])
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
        //let c = self.advance();

        match self.advance() {
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
            '"' => self.string(),
            c if c.is_digit(10) => self.number(),
            c if c.is_whitespace() => self.process_whitespace(c),
            c if c.is_alphabetic() => self.identifier(),
            c => self.error(format!("Unrecognized character: {}", c)),
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
                    // leave the newline in the stream to count lines
                    break;
                } else {
                    self.advance();
                }
            }
        } else {
            self.add_token(TokenType::Slash);
        }
    }

    fn process_whitespace(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
        }
    }

    fn string(&mut self) {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            } else {
                if c == '\n' {
                    self.line += 1;
                }
                self.advance();
            }
        }

        if self.is_at_end() {
            self.error("Unterminated string literal".to_string());
            return;
        }

        self.advance(); // consume the closing double quote

        let value = String::from(
            self.source
                .get(self.start + 1..self.current - 1)
                .expect("this should never happen 3"),
        );
        self.tokens.push(Token::string_literal(value, self.line));
    }

    fn advance_digits(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_digit(10) {
                break;
            }
            self.advance();
        }
    }

    fn current_lexeme(&self) -> String {
        String::from(
            self.source
                .get(self.start..self.current)
                .expect("there should be a string in this range"),
        )
    }

    fn number(&mut self) {
        self.advance_digits();

        // a dot after a number literal may be used as a method call
        // on the number, so we should only consume the dot if there
        // are more digits after it
        if let Some(c) = self.peek() {
            if c == '.' && self.peek_next_is_digit() {
                self.advance(); // consume the dot

                // get the fractional part
                self.advance_digits();
            }
        }

        let str_value = self.current_lexeme();
        let val: f64 = str_value.parse().unwrap();

        self.tokens
            .push(Token::number_literal(val, &str_value, self.line));
    }

    fn peek_next_is_digit(&self) -> bool {
        if self.current + 2 >= self.source.len() {
            false
        } else {
            let c = self.source_chars[self.current + 2];
            if c.is_digit(10) {
                true
            } else {
                false
            }
        }
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_alphabetic() {
                break;
            }
            self.advance();
        }

        let ident = self.current_lexeme();

        // check if it is a reserved word
        match self.reserved_words.get(&ident) {
            None => self.tokens.push(Token::identifier(&ident, self.line)),
            Some(&toktyp) => self.add_token(toktyp),
        }
    }
}

// tests
#[test]
fn test_operators() {
    let mut scanner = Scanner::new("(/*){ ;+\t -}!({.,.!=<>====!})");

    scanner.scan_tokens();

    assert!(!scanner.had_error);

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

#[test]
fn test_string_literal_1() {
    let mut scanner = Scanner::new("\"abscondmal\"");

    scanner.scan_tokens();

    assert!(!scanner.had_error);

    let mut tok_it = scanner.tokens.iter();

    let str_tok = tok_it
        .next()
        .expect("There should be a string token in the stream");

    assert_eq!(str_tok.tok_type, TokenType::String);
    assert_eq!(str_tok.lexeme, "abscondmal");
    assert_eq!(
        str_tok.literal,
        Some(Literal::String("abscondmal".to_string()))
    );
}

#[test]
fn test_number_literal_1() {
    let mut scanner = Scanner::new("1234 + 37.52");

    scanner.scan_tokens();

    assert!(!scanner.had_error);

    let mut tok_it = scanner.tokens.iter();

    let num_tok_1 = tok_it
        .next()
        .expect("There should be a number token in the stream");

    assert_eq!(num_tok_1.tok_type, TokenType::Number);
    assert_eq!(num_tok_1.lexeme, "1234");
    assert_eq!(num_tok_1.literal, Some(Literal::Number(1234.0)));

    let op_tok = tok_it
        .next()
        .expect("There should be a Plus token in the stream");

    assert_eq!(op_tok.tok_type, TokenType::Plus);

    let num_tok_2 = tok_it
        .next()
        .expect("There should be a number token in the stream");

    assert_eq!(num_tok_2.tok_type, TokenType::Number);
    assert_eq!(num_tok_2.lexeme, "37.52");
    assert_eq!(num_tok_2.literal, Some(Literal::Number(37.52)));
}

#[test]
fn test_keywords_1() {
    let mut scanner = Scanner::new("class for lunch");

    scanner.scan_tokens();

    assert!(!scanner.had_error);

    let mut tok_it = scanner.tokens.iter();

    let kw_tok_1 = tok_it
        .next()
        .expect("There should be a keyword token in the stream");

    assert_eq!(kw_tok_1.tok_type, TokenType::Class);

    let kw_tok_2 = tok_it
        .next()
        .expect("There should be a keyword token in the stream");

    assert_eq!(kw_tok_2.tok_type, TokenType::For);

    let id_tok = tok_it
        .next()
        .expect("There should be an identifier in the stream");

    assert_eq!(id_tok.tok_type, TokenType::Identifier);
    assert_eq!(
        id_tok.literal,
        Some(Literal::Identifier("lunch".to_string()))
    );
}

#[test]
fn test_keywords_2() {
    let mut scanner = Scanner::new("and for if while class return else false print true");

    scanner.scan_tokens();

    assert!(!scanner.had_error);

    let types = vec![
        TokenType::And,
        TokenType::For,
        TokenType::If,
        TokenType::While,
        TokenType::Class,
        TokenType::Return,
        TokenType::Else,
        TokenType::False,
        TokenType::Print,
        TokenType::True,
        TokenType::Eof,
    ];

    let mut typ_it = types.iter();
    for tok in scanner.tokens {
        let typ = typ_it.next().expect("A token was expected");
        assert_eq!(tok.tok_type, *typ);
    }
}

#[test]
fn test_identifiers_1() {
    let mut scanner = Scanner::new("x = y + 37;");

    scanner.scan_tokens();

    assert!(!scanner.had_error);

    let mut tok_it = scanner.tokens.iter();

    let id_tok_1 = tok_it
        .next()
        .expect("There should be an identifier in the stream");

    assert_eq!(id_tok_1.tok_type, TokenType::Identifier);
    assert_eq!(id_tok_1.literal, Some(Literal::Identifier("x".to_string())));

    let eq_tok = tok_it
        .next()
        .expect("There should be an equals token in the stream");

    assert_eq!(eq_tok.tok_type, TokenType::Equal);

    let id_tok_2 = tok_it
        .next()
        .expect("There should be an identifier in the stream");

    assert_eq!(id_tok_2.tok_type, TokenType::Identifier);
    assert_eq!(id_tok_2.literal, Some(Literal::Identifier("y".to_string())));

    let plus_tok = tok_it
        .next()
        .expect("There should be a plus token in the stream");

    assert_eq!(plus_tok.tok_type, TokenType::Plus);

    let num_tok_1 = tok_it
        .next()
        .expect("There should be a number token in the stream");

    assert_eq!(num_tok_1.tok_type, TokenType::Number);
    assert_eq!(num_tok_1.lexeme, "37");
    assert_eq!(num_tok_1.literal, Some(Literal::Number(37.0)));
}
