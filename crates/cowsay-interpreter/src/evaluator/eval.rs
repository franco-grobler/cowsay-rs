use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::expressions::{Expression, Literal as AstLiteral};
use crate::ast::statements::Statement;
use crate::ast::token::Type;
use crate::evaluator::environment::Environment;
use crate::evaluator::utils::{evaluate_binary, is_truthy};
use crate::result::{RuntimeError, Value};

/// Expression evaluator.
#[derive(Debug)]
pub struct Evaluator {
    /// Scoped variables
    pub environment: Rc<RefCell<Environment>>,
}

impl Evaluator {
    /// Create a new [`Evaluator`].
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }
    /// Executes a statement. Notice it returns () on success.
    #[allow(unused_variables)]
    pub fn execute(&mut self, stmt: &Statement) -> Result<(), RuntimeError> {
        match stmt {
            Statement::Expression(expr) => {
                // We evaluate it, but throw the resulting Value away!
                self.evaluate(expr)?;
                Ok(())
            }
            Statement::Print(expr) => {
                // Evaluate the inner expression, then print it
                let value = self.evaluate(expr)?;
                println!("{value}");
                Ok(())
            }
            Statement::Variable {
                name,
                name_token,
                initializer,
            } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Nil,
                };
                self.environment.borrow_mut().define(name.clone(), value);
                Ok(())
            }
        }
    }

    /// The core evaluation method. It takes an AST node and reduces it to a runtime Value.
    #[allow(clippy::only_used_in_recursion)]
    pub fn evaluate(
        &mut self,
        expr: &Expression,
    ) -> Result<Value, RuntimeError> {
        match expr {
            Expression::Literal(ast_literal) => {
                let value = match ast_literal {
                    AstLiteral::Number(f) => Value::Number(*f),
                    AstLiteral::String(s) => Value::String(s.clone()),
                    AstLiteral::Boolean(b) => Value::Boolean(*b),
                    AstLiteral::Nil => Value::Nil,
                };
                Ok(value)
            }

            Expression::Grouping(inner_expr) => self.evaluate(inner_expr),

            Expression::Unary {
                operator,
                expression,
            } => {
                let right_val = self.evaluate(expression)?;

                match operator.typ {
                    Type::Minus => match right_val {
                        Value::Number(f) => Ok(Value::Number(-f)),
                        _ => Err(RuntimeError::TypeMismatch {
                            expected: "Number",
                            found: [right_val.clone(), right_val], // FIX: this is wrong
                            span: operator.span,
                        }),
                    },
                    Type::Bang => {
                        // Assuming you have a '!' or 'not' operator
                        // You will need a helper to determine if a value is "truthy"
                        Ok(Value::Boolean(!is_truthy(&right_val)))
                    }
                    _ => Err(RuntimeError::ExecutionError {
                        message: format!(
                            "Unknown unary operator {:?}",
                            operator.typ
                        ),
                        span: operator.span,
                    }),
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                evaluate_binary(left_val, operator, right_val)
            }

            Expression::Variable { name, token } => {
                let value = self.environment.borrow().get(name, token)?;
                Ok(value)
            }

            Expression::InterpolatedString {
                parts,
                is_heredoc: _,
            } => {
                let mut string_builder = String::new();
                for part in parts {
                    let value = self.evaluate(part)?;
                    string_builder.push_str(&value.to_string());
                }
                Ok(Value::String(string_builder))
            }
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
