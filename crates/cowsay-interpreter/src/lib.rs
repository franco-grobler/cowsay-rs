//! Cowfile interpreter

/// Abstract Syntax Tree (AST) module
pub mod ast;
/// Evaluate expression
pub mod evaluator;
/// Lexer
pub mod lexer;
/// Parser
pub mod parser;
/// Runtime values and errors.
pub mod result;
/// Standard Library
pub mod stdlib;

/// Add two numbers
///
/// * `left`: first number to add
/// * `right`: second number to add
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
