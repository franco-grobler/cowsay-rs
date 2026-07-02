use crate::{
    ast::{expressions::Expression, statements::Statement, token::Type},
    result::RuntimeError,
};

use super::core::Parser;

impl Parser<'_> {
    /// Check for variable declaring, otherwise parse a normal statement
    pub(super) fn declaration(&mut self) -> Result<Statement, RuntimeError> {
        if self.match_token(&[Type::Variable]) {
            return self.variable_declaration();
        }
        self.statement()
    }

    /// Parse variables
    fn variable_declaration(&mut self) -> Result<Statement, RuntimeError> {
        let name_token = self.previous();
        let name = &self.source[name_token.span.start + 1..name_token.span.end]
            .to_string();

        let initializer: Option<Expression> =
            if self.match_token(&[Type::Equal]) {
                Some(self.equality()?)
            } else {
                None
            };

        let is_heredoc_init = matches!(
            initializer,
            Some(Expression::InterpolatedString {
                is_heredoc: true,
                ..
            })
        );
        if !is_heredoc_init {
            self.consume(
                Type::Semicolon,
                "Expected ';' after variable declaration.".to_string(),
            )?;
        }

        Ok(Statement::Variable {
            name: name.clone(),
            name_token,
            initializer,
        })
    }

    /// Parses normal statements (print, blocks, or expression statements)
    fn statement(&mut self) -> Result<Statement, RuntimeError> {
        if self.match_token(&[Type::KeywordPrint]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Statement, RuntimeError> {
        let value = self.equality()?;
        self.consume(Type::Semicolon, "Expected ';' after value.".to_string())?;
        Ok(Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement, RuntimeError> {
        let expr = self.equality()?;
        self.consume(
            Type::Semicolon,
            "Expected ';' after expression.".to_string(),
        )?;
        Ok(Statement::Expression(expr))
    }
}
