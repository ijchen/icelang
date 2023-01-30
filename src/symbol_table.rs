use std::collections::HashMap;

use crate::value::Value;

/// A table keeping track of variables and functions in some current icelang
/// scope
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    variables: HashMap<String, Value>,
}

impl SymbolTable {
    /// Constructs a new SymbolTable
    pub fn new() -> Self {
        Default::default()
    }

    /// Declares a new variable with the given value, returning None if the
    /// variable already exists
    pub fn declare_variable(&mut self, identifier: String, value: Value) -> Option<()> {
        if self.variables.contains_key(&identifier) {
            return None;
        }

        self.variables.insert(identifier, value);
        Some(())
    }

    /// Accesses a variable in the symbol table, returning None if the variable
    /// doesn't exist
    pub fn access_variable(&self, identifier: &str) -> Option<&Value> {
        self.variables.get(identifier)
    }

    /// Returns whether or not a variable is defined
    pub fn is_defined(&self, identifier: &str) -> bool {
        self.variables.contains_key(identifier)
    }

    /// Assigns a new value to an already existing variable
    ///
    /// # Panics
    /// - If the variable isn't already defined
    pub fn reassign(&mut self, identifier: &str, new_value: Value) {
        assert!(self.is_defined(identifier));

        *self.variables.get_mut(identifier).unwrap() = new_value;
    }
}
