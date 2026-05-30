use crate::ast::token::Token;
use crate::result::{RuntimeError, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    /// The local variables in this specific scope
    values: HashMap<String, Value>,
    /// The parent scope
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    /// Creates the global environment (no parent)
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    /// Creates a new local scope tied to a parent environment
    pub fn new_enclosed(enclosing: Rc<RefCell<Self>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    /// Creates a new variable in the current scope.
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    /// Retrieves a variable, checking parent scopes if necessary.
    pub fn get(
        &self,
        name: &str,
        token: &Token,
    ) -> Result<Value, RuntimeError> {
        // First, check local scope
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }

        // If not found, recursively check the parent scope
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name, token);
        }

        // If we hit the top and it's not there, it's an error!
        Err(RuntimeError::UndefinedVariable {
            name: name.to_string(),
            span: token.span,
        })
    }

    /// Modifies an existing variable.
    pub fn assign(
        &mut self,
        name: &str,
        value: Value,
        token: &Token,
    ) -> Result<(), RuntimeError> {
        // If it exists in local scope, update it
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return Ok(());
        }

        // If not, try to update it in the parent scope
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value, token);
        }

        Err(RuntimeError::UndefinedVariable {
            name: name.to_string(),
            span: token.span,
        })
    }
}
