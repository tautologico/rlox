use std::env;
use std::io;
use std::io::Write;
use std::fs::read_to_string;

mod lexer;
mod ast;
mod parser;
mod interpreter;

use lexer::Scanner;
use parser::Parser;

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
        match repl() {
            Ok(_) => println!("Ok..."),
            Err(_) => println!("There was some error")
        }
    }
}

fn repl() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("{}", buffer);

    let mut parser = Parser::new(&buffer);
    let expr = parser.parse();
    println!("AST: {}", expr);

    Ok(())
}

fn run(contents: &str) {
    let mut scanner = Scanner::new(contents);

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

fn process_file(fname: &str) {
    match read_to_string(fname) {
        Ok(s) => run(&s),
        Err(e) => println!("Error opening file: {}", e),
    }
}

#[allow(dead_code)]
fn parser_test_1() {
    let mut parser = Parser::new("3 + 7 * (48 - 6)");
    //let mut parser = Parser::new("42");

    let expr = parser.parse();

    println!("AST: {}", expr);
}

#[allow(dead_code)]
fn scanner_test_1() {
    let mut scanner =
        Scanner::new("(/*){ ; +\t -}!// this is a comment\n({.,.!=<>====!!})\nif x == 23");

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
