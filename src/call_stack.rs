use crate::{
    ast::AstNode,
    function::{FunctionGroup, FunctionParameters},
    source_range::SourceRange,
    symbol_table::SymbolTable,
    value::Value,
};

/// An icelang callstack
#[derive(Debug, Clone)]
pub struct CallStack<'source> {
    base_frame: StackFrame<'source>,
    stack: Vec<StackFrame<'source>>,
}

impl<'source> CallStack<'source> {
    /// Constructs a new CallStack
    pub fn new(base_frame_display_name: String) -> Self {
        Self {
            base_frame: StackFrame::new(base_frame_display_name),
            stack: Vec::new(),
        }
    }

    /// Returns the display name of the current scope
    pub fn scope_display_name(&self) -> &str {
        if !self.stack.is_empty() {
            self.stack[self.stack.len() - 1].display_name()
        } else {
            self.base_frame.display_name()
        }
    }

    /// Pushes a new stack frame to the call stack
    pub fn push_stack_frame(&mut self, display_name: String) {
        self.stack.push(StackFrame::new(display_name));
    }

    /// Pops a stack frame from the call stack
    ///
    /// # Panics
    /// - If the call stack is empty
    pub fn pop_stack_frame(&mut self) {
        assert!(!self.stack.is_empty());

        self.stack.pop().unwrap();
    }

    /// Looks up a variable in the call stack
    pub fn lookup_variable(&self, identifier: &str) -> Option<&Value> {
        if !self.stack.is_empty() {
            if let Some(value) = self.stack[self.stack.len() - 1].lookup_variable(identifier) {
                return Some(value);
            }
        }

        self.base_frame.lookup_variable(identifier)
    }

    /// Looks up a variable in the call stack, only checking the most local
    /// scope
    pub fn lookup_local_variable(&self, identifier: &str) -> Option<&Value> {
        if !self.stack.is_empty() {
            self.stack[self.stack.len() - 1].lookup_local_variable(identifier)
        } else {
            self.base_frame.lookup_local_variable(identifier)
        }
    }

    /// Looks up a function in the call stack
    pub fn lookup_function(&self, identifier: &str) -> Option<&FunctionGroup<'source>> {
        if !self.stack.is_empty() {
            if let Some(value) = self.stack[self.stack.len() - 1].lookup_function(identifier) {
                return Some(value);
            }
        }

        self.base_frame.lookup_function(identifier)
    }

    /// Looks up a function in the call stack, only checking the most local
    /// scope
    pub fn lookup_local_function(&self, identifier: &str) -> Option<&FunctionGroup<'source>> {
        if !self.stack.is_empty() {
            self.stack[self.stack.len() - 1].lookup_local_function(identifier)
        } else {
            self.base_frame.lookup_local_function(identifier)
        }
    }

    /// Assigns a new value to an already existing variable
    ///
    /// # Panics
    /// - If the variable isn't already defined
    pub fn reassign_variable(&mut self, identifier: &str, value: Value) {
        if !self.stack.is_empty() {
            let last_index = self.stack.len() - 1;
            if self.stack[last_index].lookup_variable(identifier).is_some() {
                self.stack[last_index].reassign_variable(identifier, value);
                return;
            }
        }

        self.base_frame.reassign_variable(identifier, value);
    }

    /// Declares a new variable and assigns the given value
    ///
    /// # Panics
    /// - If the variable is already defined
    pub fn declare_variable(&mut self, identifier: String, value: Value) {
        if self.stack.is_empty() {
            self.base_frame.declare_variable(identifier, value);
        } else {
            let last_index = self.stack.len() - 1;
            self.stack[last_index].declare_variable(identifier, value);
        }
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
        if self.stack.is_empty() {
            self.base_frame
                .declare_function(identifier, parameters, body, pos);
        } else {
            let last_index = self.stack.len() - 1;
            self.stack[last_index].declare_function(identifier, parameters, body, pos);
        }
    }
}

/// An icelang stack frame
#[derive(Debug, Clone)]
pub struct StackFrame<'source> {
    display_name: String,
    local: SymbolTable<'source>,
    scopes: Vec<SymbolTable<'source>>,
}

impl<'source> StackFrame<'source> {
    /// Constructs a new StackFrame
    pub fn new(display_name: String) -> Self {
        Self {
            display_name,
            local: SymbolTable::new(),
            scopes: Vec::new(),
        }
    }

    /// Returns the display name of the stack frame
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Looks up a variable in the stack frame
    pub fn lookup_variable(&self, identifier: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.lookup_variable(identifier) {
                return Some(value);
            }
        }

        self.local.lookup_variable(identifier)
    }

    /// Looks up a variable in the stack frame, only checking the most local
    /// scope
    pub fn lookup_local_variable(&self, identifier: &str) -> Option<&Value> {
        if self.scopes.is_empty() {
            self.local.lookup_variable(identifier)
        } else {
            let last_index = self.scopes.len() - 1;
            self.scopes[last_index].lookup_variable(identifier)
        }
    }

    /// Looks up a function in the stack frame
    pub fn lookup_function(&self, identifier: &str) -> Option<&FunctionGroup<'source>> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.lookup_function(identifier) {
                return Some(value);
            }
        }

        self.local.lookup_function(identifier)
    }

    /// Looks up a function in the stack frame, only checking the most local
    /// scope
    pub fn lookup_local_function(&self, identifier: &str) -> Option<&FunctionGroup<'source>> {
        if self.scopes.is_empty() {
            self.local.lookup_function(identifier)
        } else {
            let last_index = self.scopes.len() - 1;
            self.scopes[last_index].lookup_function(identifier)
        }
    }

    /// Assigns a new value to an already existing variable
    ///
    /// # Panics
    /// - If the variable isn't already defined
    pub fn reassign_variable(&mut self, identifier: &str, value: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.lookup_function(identifier).is_some() {
                scope.reassign_variable(identifier, value);
                return;
            }
        }

        self.local.reassign_variable(identifier, value)
    }

    /// Declares a new variable and assigns the given value
    ///
    /// # Panics
    /// - If the variable is already defined
    pub fn declare_variable(&mut self, identifier: String, value: Value) {
        if self.scopes.is_empty() {
            self.local.declare_variable(identifier, value);
        } else {
            let last_index = self.scopes.len() - 1;
            self.scopes[last_index].declare_variable(identifier, value);
        }
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
        if self.scopes.is_empty() {
            self.local
                .declare_function(identifier, parameters, body, pos);
        } else {
            let last_index = self.scopes.len() - 1;
            self.scopes[last_index].declare_function(identifier, parameters, body, pos);
        }
    }
}
