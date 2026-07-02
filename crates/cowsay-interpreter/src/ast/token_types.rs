//! Token types emitted by the Perl lexer.

/// Variant identifying a single lexical token.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Types {
    // Sigils
    /// $
    SigilScalar,
    /// @
    SigilArray,
    /// %
    SigilHash,
    /// &
    SigilCode,
    /// * (typeglob)
    SigilType,

    // Literals & identifiers
    /// Identifier, e.g. `foo`, `main::bar`.
    Identifier,
    /// Unquoted bareword; meaning depends on context.
    Bareword,
    /// Integer literal.
    Int,
    /// Floating-point literal.
    Float,
    /// Quoted string, e.g. "hello", 'world'.
    StringLiteral,
    /// Pattern with modifiers, e.g. /foo/ig.
    Regex,

    // Declarations & scoping
    /// my
    KwMy,
    /// our
    KwOur,
    /// local
    KwLocal,
    /// state
    KwState,

    // Control flow
    /// if
    KwIf,
    /// unless
    KwUnless,
    /// else
    KwElse,
    /// elsif
    KwElsif,
    /// while
    KwWhile,
    /// until
    KwUntil,
    /// for
    KwFor,
    /// foreach
    KwForeach,
    /// given
    KwGiven,
    /// when
    KwWhen,
    /// default
    KwDefault,

    // Loop control
    /// next
    KwNext,
    /// last
    KwLast,
    /// redo
    KwRedo,
    /// continue
    KwContinue,

    // Subroutines & packages
    /// sub
    KwSub,
    /// return
    KwReturn,
    /// package
    KwPackage,
    /// use
    KwUse,
    /// require
    KwRequire,
    /// no
    KwNo,

    // Operators
    /// =
    Assign,

    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// **
    Exp,

    /// &
    BitAnd,
    /// |
    BitOr,
    /// ^
    BitXor,
    /// ~
    BitNot,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,

    /// . (concatenation)
    Concat,
    /// x (repetition)
    Repetition,

    /// ==
    Eq,
    /// !=
    Ne,
    /// <
    Lt,
    /// <=
    Le,
    /// >
    Gt,
    /// >=
    Ge,
    /// <=>
    Cmp,
    /// &&
    And,
    /// ||
    Or,
    /// !
    Not,

    /// and
    LowAnd,
    /// or
    LowOr,
    /// not
    LowNot,
    /// xor
    LowXor,

    /// eq
    StrEq,
    /// ne
    StrNe,
    /// lt
    StrLt,
    /// le
    StrLe,
    /// gt
    StrGt,
    /// ge
    StrGe,
    /// cmp
    StrCmp,

    /// =~
    Match,
    /// !~
    NotMatch,

    /// ..
    Range,
    /// ...
    FlipFlop,
    /// ->
    Arrow,
    /// ::
    DoubleColon,

    /// ?
    Question,
    /// :
    Colon,

    // Punctuation & grouping
    /// ,
    Comma,
    /// =>
    FatComma,
    /// ;
    SemiColon,

    /// (
    LParen,
    /// )
    RParen,
    /// {
    LBrace,
    /// }
    RBrace,
    /// [
    LBracket,
    /// ]
    RBracket,

    // Special
    /// End of input.
    Eof,
    /// Lexing error.
    Error,
}
