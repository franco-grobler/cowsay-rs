//! Run interpreter on a file.

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

use cowsay_interpreter::evaluator::eval::Evaluator;
use cowsay_interpreter::lexer::scanner::Scanner;
use cowsay_interpreter::number::Number;
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

    let mut evaluator = Evaluator::new();
    run(&source, &mut evaluator);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut evaluator = Evaluator::new();
    let mut buffer = String::new();

    println!("Starting Lox REPL. Type Ctrl+D to exit.");

    loop {
        print!("> ");

        if let Err(e) = stdout.flush() {
            eprintln!("Failed to flush stdout: {e}");
            break;
        }

        buffer.clear();

        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                println!("\nExiting REPL...");
                break;
            }
            Ok(_) => {
                run(buffer.trim(), &mut evaluator);
            }
            Err(e) => {
                eprintln!("Error reading input: {e}");
                break;
            }
        }
    }
}

fn run(source: &str, evaluator: &mut Evaluator) {
    // Initialize the scanner with the source code
    let mut scanner = Scanner::new(source);

    // Kick off the scanning phase
    scanner.scan_tokens();

    let tokens = scanner.tokens();
    let mut parser = parser::core::Parser::new(tokens.to_vec(), source);
    let statements = parser.parse();

    evaluator.environment.borrow_mut().define(
        "t".to_string(),
        cowsay_interpreter::result::Value::Number(Number(0.0)),
    );
    evaluator.environment.borrow_mut().define(
        "eyes".to_string(),
        cowsay_interpreter::result::Value::String("oo".to_string()),
    );
    evaluator.environment.borrow_mut().define(
        "thoughts".to_string(),
        cowsay_interpreter::result::Value::String("o".to_string()),
    );
    evaluator.environment.borrow_mut().define(
        "tongue".to_string(),
        cowsay_interpreter::result::Value::String("U".to_string()),
    );

    for statement in &statements {
        let _ = evaluator.execute(statement);
    }

    let cow_frame = evaluator
        .environment
        .borrow_mut()
        .get("the_cow", None)
        .unwrap();

    // Print the new frame
    println!("{cow_frame}");
}
