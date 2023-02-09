use std::collections::HashMap;

use crate::{
    ast::AstNode,
    function::{Function, FunctionGroup, FunctionParameters},
    source_range::SourceRange,
    value::Value,
};

/// A table keeping track of variables and functions in some current icelang
/// scope
#[derive(Debug, Clone)]
pub struct SymbolTable<'source> {
    functions: HashMap<String, FunctionGroup<'source>>,
    variables: HashMap<String, Value>,
}

impl<'source> SymbolTable<'source> {
    /// Constructs a new SymbolTable
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    /// Declares a new variable with the given value, returning None if the
    /// variable already exists
    ///
    /// # Panics
    /// - If the variable is already defined
    pub fn declare_variable(&mut self, identifier: String, value: Value) {
        assert!(!self.variables.contains_key(&identifier));

        self.variables.insert(identifier, value);
    }

    /// Looks up a variable in the symbol table, returning None if the variable
    /// isn't defined
    pub fn lookup_variable(&self, identifier: &str) -> Option<&Value> {
        self.variables.get(identifier)
    }

    /// Declares a new function with the given parameters and body, or adds an
    /// overload to an existing function group
    ///
    /// # Panics
    /// - If the function is already defined and an overload with the same
    ///   parameter arity already exists
    pub fn declare_function(
        &mut self,
        identifier: String,
        parameters: FunctionParameters<'source>,
        body: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) {
        match self.functions.get_mut(&identifier) {
            Some(function_group) => {
                function_group.add_overload(Function::new(parameters, body, pos))
            }
            None => {
                let mut new_function_group = FunctionGroup::new();
                new_function_group.add_overload(Function::new(parameters, body, pos));
                self.functions.insert(identifier, new_function_group);
            }
        }
    }

    /// Accesses a function in the symbol table, returning None if the function
    /// isn't defined
    pub fn lookup_function(&self, identifier: &str) -> Option<&FunctionGroup<'source>> {
        self.functions.get(identifier)
    }

    /// Assigns a new value to an already existing variable
    ///
    /// # Panics
    /// - If the variable isn't already defined
    pub fn reassign_variable(&mut self, identifier: &str, new_value: Value) {
        assert!(self.lookup_variable(identifier).is_some());

        *self.variables.get_mut(identifier).unwrap() = new_value;
    }
}

impl Default for SymbolTable<'_> {
    fn default() -> Self {
        Self::new()
    }
}
