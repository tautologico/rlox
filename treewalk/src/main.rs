use std::env;
use std::fs::read_to_string;

mod lexer;

use lexer::Literal;
use lexer::Scanner;
use lexer::Token;
use lexer::TokenType;

fn main() {
    println!("Lox interpreter");
    let args: Vec<String> = env::args().skip(1).collect();
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

    let mut scanner = Scanner::new("(/*){ ; +\t -}!// this is a comment\n({.,.!=<>====!!})");

    scanner.scan_tokens();

    for tok in scanner.tokens {
        println!("Next token: {}", tok);
    }

    if scanner.had_error {
        println!("*** Errors occurred during lexing.");
    } else {
        println!("*** No lexical errors detected.")
    }
}

fn run(contents: &str) {
    println!("{}", contents);
}

fn process_file(fname: &str) {
    match read_to_string(fname) {
        Ok(s) => run(&s),
        Err(e) => println!("Error opening file: {}", e),
    }
}
