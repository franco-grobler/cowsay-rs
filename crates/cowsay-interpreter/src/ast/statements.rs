//! Statement evaluation

use crate::ast::expressions::Expression;
use crate::ast::token::Token;

/// Statements
#[derive(Debug, Clone)]
pub enum Statement {
    /// An expression followed by a semicolon (e.g., `5 + 5;`)
    Expression(Expression),

    /// A print statement (e.g., `print "Hello";`)
    Print(Expression),

    /// Variable declaration.
    Variable {
        /// Identifier token.
        name: Token,
        /// Initial value.
        initializer: Option<Expression>,
    },
}
