use crate::{
    ast::{expressions::Expression, statements::Statement, token::Type},
    result::RuntimeError,
};

use super::core::Parser;

impl Parser<'_> {
    /// Check for variable declaring, otherwise parse a normal statement
    pub(super) fn declaration(&mut self) -> Result<Statement, RuntimeError> {
        if self.match_token(&[Type::Variable]) {
            return self.let_declaration();
        }
        self.statement()
    }

    /// Parse variables
    fn let_declaration(&mut self) -> Result<Statement, RuntimeError> {
        let name_token = if self.match_token(&[Type::Variable]) {
            *self.peek()
        } else {
            return Err(self.add_error(
                "Syntax Error: Expected variable name after '$'.".to_string(),
                self.peek().span,
            ));
        };

        let initializer: Option<Expression> =
            if self.match_token(&[Type::Equal]) {
                Some(self.equality()?)
            } else {
                None
            };

        self.consume(
            Type::Semicolon,
            "Expected ';' after variable declaration.".to_string(),
        )?;
        let name = &self.source[name_token.span.start + 1..name_token.span.end]
            .to_string();

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
