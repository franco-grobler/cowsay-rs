use super::core::Parser;
use crate::{
    ast::{
        expressions::{Expression, Literal},
        token::{Token, Type},
    },
    number::Number,
    result::RuntimeError,
};

impl Parser<'_> {
    /// Express equalities.
    pub(super) fn equality(&mut self) -> Result<Expression, RuntimeError> {
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
        if self.match_token(&[Type::Variable]) {
            let token = self.previous();
            let name =
                self.source[token.span.start + 1..token.span.end].to_string();
            return Ok(Expression::Variable { name, token });
        }

        if self.match_token(&[
            Type::LiteralNumber,
            Type::LiteralString,
            Type::LiteralStringInterpolationStart,
            Type::Redirect,
        ]) {
            let token = self.previous();
            let raw_text = &self.source[token.span.start..token.span.end];

            if token.typ == Type::LiteralNumber {
                return self.match_number(token, raw_text);
            } else if token.typ == Type::LiteralStringInterpolationStart {
                return self.match_interpolated_string(token);
            } else if token.typ == Type::Redirect {
                return self.match_heredoc();
            }

            return Ok(Expression::Literal(Literal::String(
                raw_text.to_string(),
            )));
        }

        if self.match_token(&[Type::ParenthesisLeft]) {
            let expr = self.equality()?;

            self.consume(
                Type::ParenthesisRight,
                "Expected ')' after expression.".to_string(),
            )?;

            return Ok(Expression::Grouping(Box::new(expr)));
        }

        let token = self.peek();
        let raw_text = &self.source[token.span.start..token.span.end];
        let err = self.add_error(
            format!(
                "Syntax Error: Unexpected token {raw_text} at {} of type {:?}",
                self.current, token.typ
            ),
            token.span,
        );
        self.synchronize();
        Err(err)
    }

    fn match_number(
        &mut self,
        token: Token,
        raw_text: &str,
    ) -> Result<Expression, RuntimeError> {
        let val: f64 = if let Ok(x) = raw_text.parse() {
            x
        } else {
            let err = self.add_error(
                format!("Could not parse as numeric value: {raw_text}."),
                token.span,
            );
            return Err(err);
        };
        Ok(Expression::Literal(Literal::Number(Number(val))))
    }

    fn match_interpolated_string(
        &mut self,
        token: Token,
    ) -> Result<Expression, RuntimeError> {
        let mut parts: Vec<Expression> = Vec::new();
        while !self.is_at_end() {
            let expr = match self.advance().typ {
                Type::LiteralStringInterpolationEnd => break,
                _ => self.primary(),
            };
            parts.push(expr?);
        }
        if self.is_at_end() {
            let err = self.add_error(
                "Could not find end of interpolated string.".to_string(),
                token.span,
            );
            return Err(err);
        }
        Ok(Expression::InterpolatedString {
            parts,
            is_heredoc: false,
        })
    }

    fn match_heredoc(&mut self) -> Result<Expression, RuntimeError> {
        let mut parts: Vec<Expression> = Vec::new();
        self.consume(
            Type::Identifier,
            "Expected identifier after heredoc.".to_string(),
        )?;
        self.consume(
            Type::Semicolon,
            "Expected semicolon after heredoc identifier.".to_string(),
        )?;
        loop {
            if self.is_at_end() || self.match_token(&[Type::Identifier]) {
                break;
            }
            let expr = self.primary()?;
            parts.push(expr);
        }
        self.consume(
            Type::RedirectEnd,
            "Expected semicolon after heredoc identifier.".to_string(),
        )?;

        Ok(Expression::InterpolatedString {
            parts,
            is_heredoc: true,
        })
    }
}
