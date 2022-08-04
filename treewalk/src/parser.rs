use crate::lexer::Scanner;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::lexer::Value;
use crate::ast::Expr;

pub struct Parser {
    scanner: Scanner,
    current: usize
}

impl Parser {
    pub fn new(source: &str) -> Parser {
        Parser {
            current: 0,
            scanner: Scanner::new(source)
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.scanner.scan_tokens();
        // TODO: return an option; process the result of parse_expression
        // (Result<Expr, ParseError>) and return accordingly
        self.parse_expression()
    }

    pub fn parse_expression(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();
        let eq_ops = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_token_types(&eq_ops) {
            let op_type = self.previous().tok_type;
            let right = self.parse_comparison();
            expr = Expr::binary_from_token(op_type, expr, right);
        }
        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();
        let comparison_ops = vec![TokenType::Greater, TokenType::GreaterEqual,
                                  TokenType::Less, TokenType::LessEqual];
        while self.match_token_types(&comparison_ops) {
            let op_type = self.previous().tok_type;
            let right = self.parse_term();
            expr = Expr::binary_from_token(op_type, expr, right);
        }
        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        let term_ops = vec![TokenType::Plus, TokenType::Minus];
        while self.match_token_types(&term_ops) {
            let op_type = self.previous().tok_type;
            let right = self.parse_factor();
            expr = Expr::binary_from_token(op_type, expr, right);
        }
        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();
        let factor_ops = vec![TokenType::Slash, TokenType::Star];
        while self.match_token_types(&factor_ops) {
            let op_type = self.previous().tok_type;
            let right = self.parse_unary();
            expr = Expr::binary_from_token(op_type, expr, right);
        }
        expr
    }

    fn parse_unary(&mut self) -> Expr {
        let unary_ops = vec![TokenType::Bang, TokenType::Minus];
        if self.match_token_types(&unary_ops) {
            let op_type = self.previous().tok_type;
            let right = self.parse_unary();
            return Expr::unary_from_token(op_type, right);
        }

        // if it's not a unary operator, it's a primary
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        if self.match_token_types(&vec![TokenType::False]) {
            return Expr::false_literal();
        }

        if self.match_token_types(&vec![TokenType::True]) {
            return Expr::true_literal();
        }

        if self.match_token_types(&vec![TokenType::Nil]) {
            return Expr::nil_literal();
        }

        if self.match_token_types(&vec![TokenType::Number, TokenType::String]) {
            let e = match &self.previous().value {
                Some(Value::Number(i)) => Expr::number_literal(*i),
                Some(Value::String(s)) => Expr::string_literal(s),
                _ => panic!("Invalid value for token, should never happen!")
            };
            return e;
        }

        if self.match_token_types(&vec![TokenType::LeftParen]) {
            let expr = self.parse_expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            return Expr::group(expr);
        }

        // TODO: report error for unexpected token
        panic!("Expected expression");
    }

    fn consume(&mut self, typ: TokenType, msg: &str) {
        if self.check(typ) {
            self.advance();   // TODO original code returns the token from advance
            return;
        }

        // if the next token does not have the required type, raise an error
        // TODO: should properly report the error, not panic
        panic!("{}", msg);
    }

    fn match_token_types(&mut self, types: &Vec<TokenType>) -> bool {
        for typ in types {
            if self.check(*typ) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.scanner.tokens.len() ||
            self.scanner.tokens[self.current].is_eof()
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            // return last token (assuming it is EOF)
            &self.scanner.tokens[self.scanner.tokens.len() - 1]
        } else {
            &self.scanner.tokens[self.current]
        }
    }

    fn check(&self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().tok_type == typ
        }
    }

    fn previous(&self) -> &Token {
        &self.scanner.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // advance in the token stream until finding a synchronization point
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().tok_type == TokenType::Semicolon {
                return;
            }

            match self.peek().tok_type {
                TokenType::Class | TokenType::For | TokenType::Fun |
                TokenType::If | TokenType::Print | TokenType::Return |
                TokenType::Var | TokenType::While => return,
                _ => ()
            }

            self.advance();
        }
    }
}


// tests

#[test]
fn test_constant() {
    let mut parser = Parser::new("42");

    assert_eq!(parser.parse(), Expr::number_literal(42.0));
}

#[test]
fn test_simple_expression_1() {
    use crate::ast::BinOp;

    let mut parser = Parser::new("3 + 7 * (48 - 6)");

    let expected = Expr::binary(BinOp::Plus,
                                Expr::number_literal(3.0),
                                Expr::binary(BinOp::Mult,
                                             Expr::number_literal(7.0),
                                             Expr::group(
                                                 Expr::binary(BinOp::Minus,
                                                              Expr::number_literal(48.0),
                                                              Expr::number_literal(6.0)))));

    assert_eq!(parser.parse(), expected);
}
