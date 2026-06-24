//! Expressions

use std::fmt::Display;

use crate::{ast::token::Token, number::Number};

/// Expressions.
#[derive(Debug, Eq, PartialEq, Clone)]
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
    /// Variable lookup expression
    Variable {
        /// Variable name
        name: String,
        ///  Error reporting
        token: Token,
    },
    /// Interpolated string
    InterpolatedString {
        /// Parts of the string as expressions
        parts: Vec<Expression>,
        /// True if heredoc
        is_heredoc: bool,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "Expression::Binary -> {left}, {operator}, {right}")
            }
            Self::Grouping(g) => {
                write!(f, "Expression::Grouping -> {g}")
            }
            Self::Literal(l) => {
                write!(f, "Expression::Literal -> {l}")
            }
            Self::Unary {
                operator,
                expression,
            } => {
                write!(f, "Expression::Unary -> {operator}, {expression}")
            }
            Self::Variable { name, token } => {
                write!(f, "Expression::Variable -> {name}={token}")
            }
            Self::InterpolatedString { parts, is_heredoc } => {
                write!(
                    f,
                    "Expression::InterpolatedString -> {parts:?}{}",
                    if *is_heredoc { " as heredoc" } else { "" }
                )
            }
        }
    }
}

/// The AST-specific Literal
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Literal {
    /// Numeric literal
    Number(Number),
    /// String literal
    String(String),
    /// Boolean literal
    Boolean(bool),
    /// None/Nil/undef literal.
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
