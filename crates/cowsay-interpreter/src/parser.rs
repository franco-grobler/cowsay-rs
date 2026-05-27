//! Parser implementation.

use crate::{
    ast::{
        expressions::{Expression, Literal},
        token::{Span, Token, Type},
    },
    result::RuntimeError,
};

/// Construct grammar from the lexer.
///
/// * `tokens`: Lexer tokens.
/// * `source`: Raw source code to extract identifiers/numbers.
/// * `current`: Cursor.
/// * `errors`: Collected errors.
#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Vec<Token>,
    source: &'a str,
    current: usize,
    errors: Vec<RuntimeError>,
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
    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| self.tokens.last().unwrap())
    }

    /// Checks if the cursor is at the end of the token stream.
    fn is_at_end(&self) -> bool {
        self.peek().typ == Type::EndOfFile
    }

    /// Consumes and returns the current token.
    fn advance(&mut self) -> Token {
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
    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    /// Consume the current token if it matches a specific type.
    fn match_token(&mut self, types: &[Type]) -> bool {
        for &typ in types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Check the current token's type without consuming it.
    fn check(&self, typ: Type) -> bool {
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
    fn add_error(&mut self, message: String, span: Span) -> RuntimeError {
        let err = RuntimeError::ParsingError { message, span };
        self.errors.push(err.clone());
        err
    }

    /// The entry point for parsing an expression
    pub fn parse_expression(&mut self) -> Result<Expression, RuntimeError> {
        self.equality()
    }

    /// Express equalities.
    fn equality(&mut self) -> Result<Expression, RuntimeError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            Type::EqualEqual,
            Type::KeywordNotEqual,
            Type::LessThan,
            Type::MoreThan,
        ]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Express terms.
    fn term(&mut self) -> Result<Expression, RuntimeError> {
        let mut expr = self.factor()?;

        while self.match_token(&[Type::Plus, Type::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Express factors (multiplication)
    fn factor(&mut self) -> Result<Expression, RuntimeError> {
        let mut expr = self.primary()?;

        while self.match_token(&[Type::Multiply, Type::SlashForward]) {
            // Assuming you have Slash
            let operator = self.previous();
            let right = self.primary()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Express literals and groupings.
    fn primary(&mut self) -> Result<Expression, RuntimeError> {
        if self.match_token(&[Type::KeywordTrue]) {
            return Ok(Expression::Literal(Literal::Boolean(true)));
        }
        if self.match_token(&[Type::KeywordFalse]) {
            return Ok(Expression::Literal(Literal::Boolean(false)));
        }
        if self.match_token(&[Type::KeywordNil]) {
            return Ok(Expression::Literal(Literal::Nil));
        }

        if self.match_token(&[Type::LiteralNumber, Type::LiteralString]) {
            let token = self.previous();
            let raw_text = &self.source[token.span.start..token.span.end];

            if token.typ == Type::LiteralNumber {
                let val: f64 = if let Ok(x) = raw_text.parse() {
                    x
                } else {
                    let err = self.add_error(
                        format!(
                            "Could not parse as numeric value: {raw_text}."
                        ),
                        token.span,
                    );
                    return Err(err);
                };
                return Ok(Expression::Literal(Literal::Number(val)));
            }

            return Ok(Expression::Literal(Literal::String(
                raw_text.to_string(),
            )));
        }

        if self.match_token(&[Type::ParenthesisLeft]) {
            let expr = self.parse_expression()?;

            // MUST find a closing parenthesis, otherwise it's a syntax error
            if self.check(Type::ParenthesisRight) {
                self.advance();
            } else {
                let err = self.add_error(
                    "Expected ')' after expression.".to_string(),
                    self.peek().span,
                );
                return Err(err);
            }
            return Ok(Expression::Grouping(Box::new(expr)));
        }

        let token = self.peek();
        let raw_text = &self.source[token.span.start..token.span.end];
        let err = self.add_error(
            format!("Syntax Error: Unexpected token {raw_text}"),
            token.span,
        );
        self.synchronize();
        Err(err)
    }

    /// Skip until end of statement.
    /// Used to recover from errors.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().typ == Type::SemiColon {
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
        parser::Parser,
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
