//! Parser implementation.

use crate::ast::statements::Statement;
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

    /// The entry point for parsing an AST.
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Ok(stmt) = self.declaration() {
                statements.push(stmt);
            } else {
                self.synchronize();
            }
        }
        statements
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
            if self.is_at_end() {
                break;
            }
            self.current += 1;

            let token = self.previous();
            if token.typ != Type::Error {
                return token;
            }
        }
        *self.peek()
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
        println!("Error encountered: {message}");
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
            statements::Statement,
            token::{Span, Token, Type},
        },
        number::Number,
        parser::core::Parser,
    };

    #[test]
    fn can_read_comparison() {
        let source = "1 < 2;";
        let tokens = vec![
            Token::new(Type::LiteralNumber, Span::new(0, 1)),
            Token::new(Type::LessThan, Span::new(2, 3)),
            Token::new(Type::LiteralNumber, Span::new(4, 5)),
            Token::new(Type::Semicolon, Span::new(6, 7)),
            Token::new(Type::EndOfFile, Span::new(7, 7)),
        ];
        let mut parser = Parser::new(tokens, source);

        assert_eq!(
            parser.parse(),
            vec![Statement::Expression(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(Number(
                    1.0
                )))),
                operator: Token::new(Type::LessThan, Span::new(2, 3)),
                right: Box::new(Expression::Literal(Literal::Number(Number(
                    2.0
                )))),
            })]
        );
    }

    #[test]
    fn can_read_print_statements() {
        let source = r#"print "hello";"#;
        let tokens = vec![
            Token::new(Type::KeywordPrint, Span::new(0, 5)),
            Token::new(Type::LiteralString, Span::new(6, 13)),
            Token::new(Type::Semicolon, Span::new(13, 14)),
            Token::new(Type::EndOfFile, Span::new(14, 14)),
        ];
        let mut parser = Parser::new(tokens, source);

        assert_eq!(
            parser.parse(),
            vec![Statement::Print(Expression::Literal(Literal::String(
                "\"hello\"".to_string()
            )))]
        );
    }

    #[test]
    fn can_read_variable_devlaration() {
        let source = r#"$var="hello";"#;
        let tokens = vec![
            Token::new(Type::Variable, Span::new(0, 4)),
            Token::new(Type::Equal, Span::new(4, 5)),
            Token::new(Type::LiteralString, Span::new(5, 12)),
            Token::new(Type::Semicolon, Span::new(12, 13)),
            Token::new(Type::EndOfFile, Span::new(13, 13)),
        ];
        let mut parser = Parser::new(tokens, source);

        assert_eq!(
            parser.parse(),
            vec![Statement::Variable {
                name: "var".to_string(),
                name_token: Token::new(Type::Variable, Span::new(0, 4)),
                initializer: Some(Expression::Literal(Literal::String(
                    "\"hello\"".to_string()
                )))
            }]
        );
    }
}
