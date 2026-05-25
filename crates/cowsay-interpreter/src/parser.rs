//! Parser implementation.
//! TODO: Split this into a module, with a sub-module for the syntax errors.

use crate::ast::{
    expr::{Expr, Literal},
    token::{Token, Type},
};

/// Construct grammar.
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
    errors: Vec<String>,
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
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// The entry point for parsing an expression
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    /// Express equalities.
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(&[Type::EqualEqual, Type::KeywordNotEqual]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Express terms.
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[Type::Plus, Type::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Express factors (multiplication)
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        while self.match_token(&[Type::Multiply, Type::SlashForward]) {
            // Assuming you have Slash
            let operator = self.previous();
            let right = self.primary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Express literals and groupings.
    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[Type::KeywordTrue]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.match_token(&[Type::KeywordFalse]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.match_token(&[Type::KeywordNil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_token(&[Type::LiteralNumber, Type::LiteralString]) {
            let token = self.previous();
            let raw_text = &self.source[token.span.start..token.span.end];

            if token.typ == Type::LiteralNumber {
                let val: f64 = raw_text.parse().unwrap();
                return Ok(Expr::Literal(Literal::Number(val)));
            }

            return Ok(Expr::Literal(Literal::String(raw_text.to_string())));
        }

        if self.match_token(&[Type::ParenthesisLeft]) {
            let expr = self.parse_expression()?;

            // MUST find a closing parenthesis, otherwise it's a syntax error
            if self.check(Type::ParenthesisRight) {
                self.advance();
            } else {
                return Err(
                    "Syntax Error: Expected ')' after expression.".to_string()
                );
            }
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let token = self.peek();
        Err(format!("Syntax Error: Unexpected token '{:?}'", token.typ))
    }
}
