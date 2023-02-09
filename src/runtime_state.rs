//! Contains code related to `RuntimeState`s, which represent the entire state
//! of an icelang program during execution.

use std::fmt::Display;

use crate::{
    ast::AstNode,
    call_stack::CallStack,
    function::{FunctionGroup, FunctionParameters},
    icelang_std_lib::{IcelangFmt, IcelangFmtArgs},
    source_range::SourceRange,
    value::Value,
};

/// Represents the entire state of an icelang program during execution
#[derive(Clone, Debug)]
pub struct RuntimeState<'source> {
    most_recent_value: Value,
    call_stack: CallStack<'source>,
}

impl<'source> RuntimeState<'source> {
    /// Constructs a new default RuntimeState
    pub fn new() -> Self {
        Self {
            most_recent_value: Value::Null,
            call_stack: CallStack::new(),
        }
    }

    /// Pushes a new stack frame to the call stack
    pub fn push_stack_frame(&mut self) {
        self.call_stack.push_stack_frame();
    }

    /// Pops a stack frame from the call stack
    ///
    /// # Panics
    /// - If the call stack is empty
    pub fn pop_stack_frame(&mut self) {
        self.call_stack.pop_stack_frame();
    }

    /// Returns the most recent value from an expression
    pub fn most_recent_value(&self) -> &Value {
        &self.most_recent_value
    }

    /// Updates the most recent value from an expression
    pub fn update_most_recent_value(&mut self, value: Value) {
        self.most_recent_value = value;
    }

    /// Declares a new variable and assigns the given value
    ///
    /// # Panics
    /// - If the variable is already defined
    pub fn declare_variable(&mut self, identifier: String, value: Value) {
        assert!(self.call_stack.lookup_local_variable(&identifier).is_none());

        self.call_stack.declare_variable(identifier, value);
    }

    /// Declares a function (or overloads a function)
    ///
    /// # Panics
    /// - If the function is already defined (including arity)
    pub fn declare_function(
        &mut self,
        identifier: String,
        parameters: FunctionParameters<'source>,
        body: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) {
        self.call_stack
            .declare_function(identifier, parameters, body, pos);
    }

    /// Looks up a variable in the runtime state
    pub fn lookup_variable(&self, identifier: &str) -> Option<&Value> {
        self.call_stack.lookup_variable(identifier)
    }

    /// Looks up a variable in the runtime state, only checking the most local
    /// scope
    pub fn lookup_local_variable(&self, identifier: &str) -> Option<&Value> {
        self.call_stack.lookup_local_variable(identifier)
    }

    /// Looks up a function in the runtime state
    pub fn lookup_function(&self, identifier: &str) -> Option<&FunctionGroup<'source>> {
        self.call_stack.lookup_function(identifier)
    }

    /// Looks up a function in the runtime state, only checking the most local
    /// scope
    pub fn lookup_local_function(&self, identifier: &str) -> Option<&FunctionGroup<'source>> {
        self.call_stack.lookup_local_function(identifier)
    }

    /// Assigns a new value to an already existing variable
    ///
    /// # Panics
    /// - If the variable isn't already defined
    pub fn reassign_variable(&mut self, identifier: &str, value: Value) {
        assert!(self.call_stack.lookup_variable(identifier).is_some());

        self.call_stack.reassign_variable(identifier, value)
    }
}

impl<'source> Default for RuntimeState<'source> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'source> Display for RuntimeState<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Most recent value: ")?;
        let fmt_args = IcelangFmtArgs { debug: true };
        self.most_recent_value.icelang_fmt(f, &fmt_args)?;
        Ok(())
    }
}
