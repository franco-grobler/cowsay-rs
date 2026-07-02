use crate::{ast::token::Span, result::RuntimeError};

/// Diagnostic severity
#[derive(Debug, Clone)]
pub enum Severity {
    /// Someone fucked up
    Error,
    /// Not great
    Warning,
    /// Could be better
    Advice,
}

/// Diagnostic labels
#[derive(Debug, Clone)]
pub struct Label {
    /// Location
    span: Span,
    /// Text at location
    text: String,
}

/// Error diagnostics.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Error severity.
    pub severity: Severity,
    /// Error code
    pub code: Option<String>,
    /// Error message.
    pub message: String,
    /// Error location
    pub span: Span,
    /// Error label
    pub labels: Vec<Label>,
    /// Error notes
    pub notes: Vec<String>,
}

impl From<RuntimeError> for Diagnostic {
    fn from(value: RuntimeError) -> Self {
        todo!()
    }
}
