//! Parser implementation.

use crate::ast::token::{Span, Token, Type};
use crate::result::RuntimeError;

/// Construct grammar from the lexer.
///
/// * `tokens`: Lexer tokens.
/// * `source`: Raw source code to extract identifiers/numbers.
/// * `current`: Cursor.
/// * `errors`: Collected errors.
#[derive(Debug)]
pub struct Parser<'a> {
    pub(super) tokens: Vec<Token>,
    pub(super) source: &'a str,
    pub(super) current: usize,
    pub(super) errors: Vec<RuntimeError>,
}

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`].
    pub const fn new(tokens: Vec<Token>, source: &'a str) -> Self {
        Self {
            tokens,
            source,
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Look at the current token without consuming it.
    pub(super) fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| self.tokens.last().unwrap())
    }

    /// Checks if the cursor is at the end of the token stream.
    pub(super) fn is_at_end(&self) -> bool {
        self.peek().typ == Type::EndOfFile
    }

    /// Consumes and returns the current token.
    pub(super) fn advance(&mut self) -> Token {
        loop {
            if !self.is_at_end() {
                self.current += 1; // This will now compile perfectly!
            }

            // This is now an owned copy, so it releases `self` immediately
            let token = self.previous();

            if token.typ != Type::Error {
                return token;
            }
        }
    }

    /// Returns the most recently consumed token.
    pub(super) fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    /// Consume the current token if it matches a specific type.
    pub(super) fn match_token(&mut self, types: &[Type]) -> bool {
        for &typ in types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Check the current token's type without consuming it.
    pub(super) fn check(&self, typ: Type) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().typ == typ
    }

    /// Get collected errors.
    pub fn errors(&self) -> &[RuntimeError] {
        &self.errors
    }

    /// Add errors to tracked errors.
    ///
    /// * `message`: Error description.
    /// * `span`: Error location.
    pub(super) fn add_error(
        &mut self,
        message: String,
        span: Span,
    ) -> RuntimeError {
        let err = RuntimeError::ParsingError { message, span };
        self.errors.push(err.clone());
        err
    }

    pub(super) fn consume(
        &mut self,
        typ: Type,
        message: String,
    ) -> Result<Token, RuntimeError> {
        if self.check(typ) {
            return Ok(self.advance());
        }
        let err = self.add_error(message, self.peek().span);
        Err(err)
    }

    /// Skip until end of statement.
    /// Used to recover from errors.
    pub(super) fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().typ == Type::Semicolon {
                return;
            }
            match self.peek().typ {
                Type::StatementPrint
                | Type::LoopWhile
                | Type::LoopFor
                | Type::Return
                | Type::Function => return,
                _ => {}
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            expressions::{Expression, Literal},
            token::{Span, Token, Type},
        },
        parser::core::Parser,
    };

    #[test]
    fn can_read_comparison() {
        let source = "1 < 2";
        let tokens = vec![
            Token::new(Type::LiteralNumber, Span::new(0, 1)),
            Token::new(Type::LessThan, Span::new(2, 3)),
            Token::new(Type::LiteralNumber, Span::new(4, 5)),
            Token::new(Type::EndOfFile, Span::new(5, 5)),
        ];
        let mut parser = Parser::new(tokens, source);

        assert_eq!(
            parser.parse_expression().unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(1.0))),
                operator: Token::new(Type::LessThan, Span::new(2, 3)),
                right: Box::new(Expression::Literal(Literal::Number(2.0))),
            }
        );
    }
}
