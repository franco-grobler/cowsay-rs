//! Expressions

use crate::ast::token::Token;

/// Expressions.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Binary expression.
    Binary {
        /// Left operand.
        left: Box<Expression>,
        /// Operator to apply.
        operator: Token,
        /// Right operand.
        right: Box<Expression>,
    },
    /// Grouping expression.
    Grouping(Box<Expression>),
    /// Literal expression.
    Literal(Literal),
    /// Unary expression.
    Unary {
        /// Operator to apply to expression
        operator: Token,
        /// Expression receiving operator
        expression: Box<Expression>,
    },
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
