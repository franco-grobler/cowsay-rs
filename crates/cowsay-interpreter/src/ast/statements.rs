//! Statement evaluation

use std::fmt::Display;

use crate::ast::expressions::Expression;
use crate::ast::token::Token;

/// Statements
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    /// An expression followed by a semicolon (e.g., `5 + 5;`)
    Expression(Expression),

    /// A print statement (e.g., `print "Hello";`)
    Print(Expression),

    /// Variable
    Variable {
        /// Parsed name
        name: String,
        /// Identifier token.
        name_token: Token,
        /// Initial value.
        initializer: Option<Expression>,
    },
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(e) => write!(f, "expression: {e}"),
            Self::Print(b) => write!(f, "print {b}"),
            Self::Variable {
                name,
                name_token,
                initializer,
            } => {
                write!(f, "var {name} = {initializer:?} at {name_token}")
            }
        }
    }
}
