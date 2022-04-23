use std::env;
use std::fs::read_to_string;

mod lexer;

use lexer::Token;
use lexer::TokenType;
use lexer::Literal;

fn main() {
    println!("Lox interpreter");
    let args : Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        println!("Usage: rlox [filename]");
        std::process::exit(1);
    }
    if args.len() == 1 {
        println!("Processing file: {}", &args[0]);
        process_file(&args[0]);
    } else {
        println!("Opening the REPL...");
    }

    let t1 = Token {
        tok_type: TokenType::LeftParen,
        lexeme: String::from("("),
        literal: None,
        line: 11
    };

    println!("This is a token: {}", t1);

    let t2 = Token::string_literal(String::from("this is something"), 42);

    println!("This is another token: {}", t2);
}

fn run(contents: &str) {
    println!("{}", contents);
}

fn process_file(fname: &str) {
    match read_to_string(fname) {
        Ok(s) => run(&s),
        Err(e) => println!("Error opening file: {}", e)
    }
}
