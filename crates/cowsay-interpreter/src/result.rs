//! Runtime values and errors.

use std::result;

use crate::{ast::token::Span, number::Number};

/// Possible runtime results.
pub type Result<T> = result::Result<T, RuntimeError>;

/// Runtime values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// Nil/None/undef
    Nil,
    /// Literal boolean.
    Boolean(bool),
    /// Literal number.
    Number(Number),
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
#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// Error during execution
    ExecutionError {
        /// Execution message.
        message: String,
        /// Location
        span: Span,
    },
    /// Error during parsing
    ParsingError {
        /// Parsing message.
        message: String,
        /// Location
        span: Span,
    },
    /// Mixing types during evaluation.
    TypeMismatch {
        /// Expected type.
        expected: &'static str,
        /// Evaluated type.
        found: [Value; 2],
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
    /// Unknown sequence of characters.
    UndefinedControlSequence {
        /// Location
        span: Span,
    },
}

// To make it integrate perfectly with Rust, implement std::error::Error
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivisionByZero { .. } => {
                write!(f, "Math Error: Division by zero")
            }
            Self::ExecutionError { message, .. } => {
                write!(f, "Execution Error: {message}")
            }
            Self::ParsingError { message, .. } => {
                write!(f, "Parsing Error: {message}")
            }
            Self::TypeMismatch {
                expected, found, ..
            } => {
                write!(
                    f,
                    "Type Error: Expected {expected}, found {} and {}",
                    found[0], found[1],
                )
            }
            Self::UndefinedFunction { name, .. } => {
                write!(f, "Reference Error: Undefined function '{name}'")
            }
            Self::UndefinedVariable { name, .. } => {
                write!(f, "Reference Error: Undefined variable '{name}'")
            }
            Self::UndefinedControlSequence { .. } => {
                write!(f, "Undefined control sequence")
            }
        }
    }
}

impl std::error::Error for RuntimeError {}
