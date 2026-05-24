//! Handling Cowfile tokens

use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
/// Byte index of a lexeme.
///
/// * `start`: Index of first character.
/// * `end`: Index of final character.
pub struct Span {
    /// Index of first character.
    pub start: usize,
    /// Index of final character.
    pub end: usize,
}

impl Span {
    /// Create a new Span.
    ///
    /// * `start`: Start byte index.
    /// * `end`: End byte index.
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// A Token read from source.
///
/// A Token describes the lexeme read from a source.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    /// This token's type
    pub typ: Type,
    /// The raw lexeme location read from source
    pub span: Span,
}

impl Token {
    /// Creates a new [`Token`].
    pub const fn new(typ: Type, span: Span) -> Self {
        Self { typ, span }
    }
}

/// Describes a literal string or number value
#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    /// No value literal.
    Nil,
    /// Boolean literal.
    Boolean(bool),
    /// Floating point literal.
    Number(f64),
    /// String literal. Use clone-on-write (cow) for escape sequences.
    String(std::borrow::Cow<'a, str>),
}

/// Describes the type of a Token
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Variable name
    Identifier,

    // Operators
    /// EOF
    EndOfFile,
    /// <<
    Redirect,
    /// <<-
    RedirectStrip,
    /// =
    Equal,
    /// .=
    EqualDot,
    /// -
    Minus,
    /// x
    Multiply,
    /// (
    ParenthesisLeft,
    /// )
    ParenthesisRight,
    /// +
    Plus,

    // Keywords
    /// ne
    KeywordNotEqual,
    /// unless
    KeywordUnless,
    /// True
    KeywordTrue,
    /// False
    KeywordFalse,

    // Literals
    /// Boolean
    LiteralBoolean,
    /// Number
    LiteralNumber,
    /// String
    LiteralString,

    /// Errors
    Error,
}
