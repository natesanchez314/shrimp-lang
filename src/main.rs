mod lexer;
mod token;

use std::{env, fs::{self, File}, io::{self, stdin, stdout, BufRead, StdoutLock, Write}, str};

use lexer::Lexer;

static mut HAD_ERROR: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: Shrimp [script]")
    } else if args.len() == 2 {
        println!("Run file!");
        let mut lock = stdout().lock();
        run_file(args.get(1).unwrap(), &mut lock);
    } else {
        repl();
    }
}

fn run_file(path: &str, lock: &mut StdoutLock) {
    let bytes = fs::read(path)
        .expect("Unable to open file!");
    run(str::from_utf8(&bytes).unwrap(), lock);
}

fn repl() {
    let mut lock = stdout().lock();
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
                run(&input, &mut lock);
            }
            _ => {}
        }
    }
}

fn run(src: &str, lock: &mut StdoutLock) {
    let mut lexer = Lexer::new(src);
    println!("Starting lexer...");
    let tokens = lexer.eval();
    println!("Finished lexing.");
    //writeln!(lock, "Tokens size: {}", tokens.len()).unwrap();
    println!("Tokens size: {:?}.", tokens.len());
    for token in tokens {
        println!("{}", token.to_string());
    }
}