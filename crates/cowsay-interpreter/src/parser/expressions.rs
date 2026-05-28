use super::core::Parser;
use crate::{
    ast::{
        expressions::{Expression, Literal},
        token::Type,
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
                return Ok(Expression::Literal(Literal::Number(Number(val))));
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
            format!("Syntax Error: Unexpected token {raw_text}"),
            token.span,
        );
        self.synchronize();
        Err(err)
    }
}
