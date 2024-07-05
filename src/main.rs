mod ast;
mod lexer;
//mod lexer_with_iter;
mod token;
mod shrimp_err;
mod parser;

use std::{env, fs, io::{stdin, stdout, Write}, str};

use lexer::Lexer;
//use lexer_with_iter::Lexer;
use token::{Token, TokenType};

static mut HAD_ERROR: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: Shrimp [script]")
    } else if args.len() == 2 {
        run_file(args.get(1).unwrap());
    } else {
        repl();
    }
}

fn run_file(path: &str) {
    let bytes = fs::read(path)
        .expect("Unable to open file!");
    run(str::from_utf8(&bytes).unwrap());
}

fn repl() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(input)) => {
                if input.trim() == "exit" {
                    break;
                } else if input.trim().is_empty() {
                    continue;
                }
                run(&input);
            }
            _ => {}
        }
    }
}

fn run(src: &str) {
    let mut lexer = Lexer::new(src.to_owned());
    let tokens = lexer.scan();
    for token in tokens {
        println!("{}", token.to_string());
    }
}