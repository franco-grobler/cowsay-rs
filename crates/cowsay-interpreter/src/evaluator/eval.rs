use crate::ast::expressions::{Expression, Literal as AstLiteral};
use crate::ast::token::Type;
use crate::evaluator::utils::{evaluate_binary, is_truthy};
use crate::result::{RuntimeError, Value};

/// Expression evaluator.
#[derive(Debug)]
pub struct Evaluator;

impl Evaluator {
    /// Create a new [`Evaluator`].
    pub const fn new() -> Self {
        Self
    }

    /// The core evaluation method. It takes an AST node and reduces it to a runtime Value.
    #[allow(clippy::only_used_in_recursion)]
    pub fn evaluate(
        &mut self,
        expr: &Expression,
    ) -> Result<Value, RuntimeError> {
        match expr {
            // 1. Literals: Convert the compile-time AST literal into a runtime Value
            Expression::Literal(ast_literal) => {
                let value = match ast_literal {
                    AstLiteral::Number(f) => Value::Number(*f),
                    AstLiteral::String(s) => Value::String(s.clone()), // Or Rc::clone if using Rc
                    AstLiteral::Boolean(b) => Value::Boolean(*b),
                    AstLiteral::Nil => Value::Nil,
                };
                Ok(value)
            }

            // 2. Grouping: Simply unwrap the parentheses and evaluate what's inside
            Expression::Grouping(inner_expr) => self.evaluate(inner_expr),

            // 3. Unary: Evaluate the right side first, then apply the operator (- or !)
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

            // 4. Binary: Evaluate left and right, then apply the math/logic
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                evaluate_binary(left_val, operator, right_val)
            }
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
