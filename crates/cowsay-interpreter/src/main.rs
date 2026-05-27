//! Run interpreter on a file.

use std::env;
use std::fs;
use std::process;

use cowsay_interpreter::evaluator::eval::Evaluator;
use cowsay_interpreter::lexer::scanner::Scanner;
use cowsay_interpreter::parser;

fn main() {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // args[0] is the executable name, so the first actual argument is args[1]
    match args.len() {
        // Too many arguments
        len if len > 2 => {
            eprintln!("Usage: rlox [script]");
            process::exit(64);
        }
        // Exactly one argument (the file path)
        2 => {
            run_file(&args[1]);
        }
        // No arguments (start the REPL)
        _ => {
            run_prompt();
        }
    }
}

fn run_file(path: &str) {
    // Read the file context into a String
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file '{path}': {error}");
            process::exit(74);
        }
    };

    run(&source);
}

fn run_prompt() {
    println!("Starting REPL...");
    // REPL loop logic goes here
}

fn run(source: &str) {
    // Initialize the scanner with the source code
    let mut scanner = Scanner::new(source);

    // Kick off the scanning phase
    scanner.scan_tokens();

    let tokens = scanner.tokens();
    let mut parser = parser::Parser::new(tokens.to_vec(), source);
    match parser.parse_expression() {
        Ok(ast) => {
            // 3. Evaluate
            let mut evaluator = Evaluator::new();
            match evaluator.evaluate(&ast) {
                Ok(result) => println!("Result: {result}"),
                Err(e) => eprintln!("{e}"),
            }
        }
        Err(e) => eprintln!("Syntax Error: {e}"),
    }
}
