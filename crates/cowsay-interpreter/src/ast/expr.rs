//! Expressions

use crate::ast::token::{Literal, Token};

/// Expressions.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    /// Binary expression.
    Binary {
        /// Left operand.
        left: Box<Expr<'a>>,
        /// Binary operator.
        operator: Token,
        /// Right operand.
        right: Box<Expr<'a>>,
    },
    /// Grouping expression.
    Grouping {
        /// Inside expression.
        expression: Box<Expr<'a>>,
    },
    /// Literal expression.
    Literal {
        /// Raw value.
        value: Literal<'a>,
    },
    /// Unary expression.
    Unary {
        /// Unary operator.
        operator: Token,
        /// Right operand.
        right: Box<Expr<'a>>,
    },
}
