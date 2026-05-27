use crate::{
    ast::token::{Token, Type},
    result::{RuntimeError, Value},
};

/// Helper method for binary operations to keep the main match block clean
pub(super) fn evaluate_binary(
    left: Value,
    operator: &Token,
    right: Value,
) -> Result<Value, RuntimeError> {
    match (left, operator.typ, right) {
        // --- Math ---
        (Value::Number(a), Type::Plus, Value::Number(b)) => {
            Ok(Value::Number(a + b))
        }
        (Value::Number(a), Type::Minus, Value::Number(b)) => {
            Ok(Value::Number(a - b))
        }
        (Value::Number(a), Type::Multiply, Value::Number(b)) => {
            Ok(Value::Number(a * b))
        }
        (Value::Number(a), Type::SlashForward, Value::Number(b)) => {
            if b == 0.0 {
                return Err(RuntimeError::DivisionByZero {
                    span: operator.span,
                });
            }
            Ok(Value::Number(a / b))
        }

        // --- String Concatenation ---
        (Value::String(a), Type::Plus, Value::String(b)) => {
            Ok(Value::String(a + &b))
        }

        // --- Equality ---
        // PartialEq handles all the logic because we derived it on Value!
        (a, Type::EqualEqual, b) => Ok(Value::Boolean(a == b)),
        (a, Type::KeywordNotEqual, b) => Ok(Value::Boolean(a != b)),

        // --- Errors ---
        (a, _, b) => Err(RuntimeError::TypeMismatch {
            expected: "Matching types for binary operation",
            found: [a, b],
            span: operator.span,
        }),
    }
}

/// Helper method to define what values are considered "true" in your language
pub(super) fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Boolean(b) => *b,
        // Most languages consider 0 to be false, and non-zero to be true
        Value::Number(i) => *i != 0.0,
        Value::String(s) => !s.is_empty(),
    }
}
