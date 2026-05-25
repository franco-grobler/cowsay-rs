//! Handling Cowfile tokens

use std::hash::Hash;

/// Byte index of a lexeme.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Span {
    ///  Index of first character.
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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token {
    /// Token's type.
    pub typ: Type,
    /// Raw lexeme location read from source
    pub span: Span,
}

impl Token {
    /// Creates a new [`Token`].
    pub const fn new(typ: Type, span: Span) -> Self {
        Self { typ, span }
    }
}

/// Describes the type of a Token
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Variable name
    Identifier,

    // Operators
    /// EOF
    EndOfFile,
    /// \n
    NewLine,
    /// <<
    Redirect,
    /// <<-
    RedirectStrip,
    /// =
    Equal,
    /// =
    EqualEqual,
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
    /// .=
    DotEqual,
    /// <
    LessThan,
    /// >
    MoreThan,
    /// /
    SlashForward,
    /// \
    SlashBackward,

    // Keywords
    /// ne
    KeywordNotEqual,
    /// unless
    KeywordUnless,
    /// True
    KeywordTrue,
    /// False
    KeywordFalse,
    /// undef
    KeywordNil,

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
