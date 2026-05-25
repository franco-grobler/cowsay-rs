//! Expressions

use crate::ast::token::Token;

/// Expressions.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// Binary expression.
    Binary {
        /// Left operand.
        left: Box<Expr>,
        /// Operator to apply.
        operator: Token,
        /// Right operand.
        right: Box<Expr>,
    },
    /// Grouping expression.
    Grouping(Box<Expr>),
    /// Literal expression.
    Literal(Literal),
    /// Unary expression.
    Unary(Token, Box<Expr>),
}

/// The AST-specific Literal
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    /// Numeric literal
    Number(f64),
    /// String literal
    String(String),
    /// Boolean literal
    Boolean(bool),
    /// None/Nil/undef literal.
    Nil,
}
