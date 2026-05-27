use crate::{
    ast::{expressions::Expression, statements::Statement, token::Type},
    result::RuntimeError,
};

use super::core::Parser;

impl Parser<'_> {
    /// Checks if we are declaring a variable, otherwise parses a normal statement
    fn declaration(&mut self) -> Result<Statement, RuntimeError> {
        if self.match_token(&[Type::DollarSign]) {
            return self.let_declaration();
        }
        self.statement()
    }

    /// Parses variables: `let x = 5;`
    fn let_declaration(&mut self) -> Result<Statement, RuntimeError> {
        // 1. We must find an identifier (variable name) next
        let name = if self.match_token(&[Type::Identifier]) {
            self.previous()
        } else {
            return Err(self.add_error(
                "Syntax Error: Expected variable name after 'let'.".to_string(),
                self.peek().span,
            ));
        };

        // 2. Check if there is an '=' sign
        let initializer: Option<Expression> =
            if self.match_token(&[Type::Equal]) {
                Some(self.parse_expression()?)
            } else {
                None
            };

        // 3. We MUST find a semicolon at the end
        self.consume(
            Type::Semicolon,
            "Expected ';' after variable declaration.".to_string(),
        )?;

        Ok(Statement::Variable { name, initializer })
    }

    /// Parses normal statements (print, blocks, or expression statements)
    fn statement(&mut self) -> Result<Statement, RuntimeError> {
        if self.match_token(&[Type::KeywordPrint]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Statement, RuntimeError> {
        let value = self.parse_expression()?;
        self.consume(Type::Semicolon, "Expected ';' after value.".to_string())?;
        Ok(Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement, RuntimeError> {
        let expr = self.parse_expression()?;
        self.consume(
            Type::Semicolon,
            "Expected ';' after expression.".to_string(),
        )?;
        Ok(Statement::Expression(expr))
    }
}
