//! Runtime values and errors.

use std::result;

use crate::ast::token::Span;

/// Possible runtime results.
pub type Result = result::Result<Value, RuntimeError>;

/// Runtime values.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Nil/None/undef
    Nil,
    /// Literal boolean.
    Boolean(bool),
    /// Literal number.
    Number(f64),
    /// Literal string.
    String(String),
}

// You can also implement helpful Display traits here
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

/// Errors encountered during runtime.
#[derive(Debug)]
pub enum RuntimeError {
    /// Mixing types during evaluation.
    TypeMismatch {
        /// Expected type.
        expected: &'static str,
        /// Evaluated type.
        found: Value,
        /// Location
        span: Span,
    },
    /// Dividing by zero
    DivisionByZero {
        /// Location
        span: Span,
    },
    /// Calling undefined function
    UndefinedFunction {
        /// Function name.
        name: String,
        /// Location
        span: Span,
    },
    /// Calling undefined variable
    UndefinedVariable {
        /// Variable name.
        name: String,
        /// Location
        span: Span,
    },
}

// To make it integrate perfectly with Rust, implement std::error::Error
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeMismatch {
                expected, found, ..
            } => {
                write!(f, "Type Error: Expected {expected}, found {found}")
            }
            Self::DivisionByZero { .. } => {
                write!(f, "Math Error: Division by zero")
            }
            Self::UndefinedFunction { name, .. } => {
                write!(f, "Reference Error: Undefined function '{name}'")
            }
            Self::UndefinedVariable { name, .. } => {
                write!(f, "Reference Error: Undefined variable '{name}'")
            }
        }
    }
}

impl std::error::Error for RuntimeError {}
