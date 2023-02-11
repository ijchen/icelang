//! Contains code related to `RuntimeError`s, which represent errors that
//! occurred during an icelang runtime

use std::{error::Error, fmt::Display};

use crate::source_range::SourceRange;

use super::*;

/// Represents an error that occurred during an icelang runtime
#[derive(Debug)]
pub enum RuntimeError<'source> {
    /// A value was an invalid type
    Type {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The stack trace for the error
        stack_trace: StackTrace<'source>,

        /// An explanation of why the type is invalid
        why: String,
    },

    /// A mathematical error occured
    Mathematical {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The stack trace for the error
        stack_trace: StackTrace<'source>,

        /// An explanation of what went wrong
        why: String,
    },

    /// A declaration attempted to declare an identifier which already existed
    IdentifierAlreadyDeclared {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The stack trace for the error
        stack_trace: StackTrace<'source>,

        /// The identifier which was already declared
        identifier: String,
    },

    /// An undefined reference was made
    UndefinedReference {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The stack trace for the error
        stack_trace: StackTrace<'source>,

        /// The identifier which was referenced but not defined
        identifier: String,
    },

    /// A necessary resource was unavailable
    // TODO eventually catch all (if possible, or most) memory/allocation errors
    ResourceUnavailable {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The stack trace for the error
        stack_trace: StackTrace<'source>,

        /// An explanation of what went wrong
        why: String,
    },

    /// A function was called with an invalid number of arguments
    InvalidOverload {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The stack trace for the error
        stack_trace: StackTrace<'source>,

        /// The name of the function
        function_name: String,

        /// The invalid number of arguments provided
        argument_count: usize,
    },

    /// Attempted to call something which is not a function
    CalledNonFunction {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The stack trace for the error
        stack_trace: StackTrace<'source>,
    },
}

impl<'source> RuntimeError<'source> {
    /// Constructs a new Type RuntimeError
    pub fn new_type_error(
        pos: SourceRange<'source>,
        scope_display_name: String,
        why: String,
    ) -> Self {
        let mut stack_trace = StackTrace::new();
        stack_trace.add_bottom(scope_display_name, pos.clone());
        Self::Type {
            pos,
            stack_trace,
            why,
        }
    }

    /// Constructs a new Mathematical RuntimeError
    pub fn new_mathematical_error(
        pos: SourceRange<'source>,
        scope_display_name: String,
        why: String,
    ) -> Self {
        let mut stack_trace = StackTrace::new();
        stack_trace.add_bottom(scope_display_name, pos.clone());
        Self::Mathematical {
            pos,
            stack_trace,
            why,
        }
    }

    /// Constructs a new IdentifierAlreadyDeclared RuntimeError
    pub fn new_identifier_already_declared_error(
        pos: SourceRange<'source>,
        scope_display_name: String,
        identifier: String,
    ) -> Self {
        let mut stack_trace = StackTrace::new();
        stack_trace.add_bottom(scope_display_name, pos.clone());
        Self::IdentifierAlreadyDeclared {
            pos,
            stack_trace,
            identifier,
        }
    }

    /// Constructs a new UndefinedReference RuntimeError
    pub fn new_undefined_reference_error(
        pos: SourceRange<'source>,
        scope_display_name: String,
        identifier: String,
    ) -> Self {
        let mut stack_trace = StackTrace::new();
        stack_trace.add_bottom(scope_display_name, pos.clone());
        Self::UndefinedReference {
            pos,
            stack_trace,
            identifier,
        }
    }

    /// Constructs a new InvalidOverload RuntimeError
    pub fn new_invalid_overload_error(
        pos: SourceRange<'source>,
        scope_display_name: String,
        function_name: String,
        argument_count: usize,
    ) -> Self {
        let mut stack_trace = StackTrace::new();
        stack_trace.add_bottom(scope_display_name, pos.clone());
        Self::InvalidOverload {
            pos,
            stack_trace,
            function_name,
            argument_count,
        }
    }

    /// Constructs a new InvalidOverload RuntimeError
    pub fn new_called_non_function_error(
        pos: SourceRange<'source>,
        scope_display_name: String,
    ) -> Self {
        let mut stack_trace = StackTrace::new();
        stack_trace.add_bottom(scope_display_name, pos.clone());
        Self::CalledNonFunction { pos, stack_trace }
    }

    /// Returns the StackTrace corresponding to this error
    pub fn stack_trace(&self) -> &StackTrace<'source> {
        match self {
            Self::Type {
                pos: _,
                stack_trace,
                why: _,
            } => stack_trace,
            Self::Mathematical {
                pos: _,
                stack_trace,
                why: _,
            } => stack_trace,
            Self::IdentifierAlreadyDeclared {
                pos: _,
                stack_trace,
                identifier: _,
            } => stack_trace,
            Self::UndefinedReference {
                pos: _,
                stack_trace,
                identifier: _,
            } => stack_trace,
            Self::ResourceUnavailable {
                pos: _,
                stack_trace,
                why: _,
            } => stack_trace,
            Self::InvalidOverload {
                pos: _,
                stack_trace,
                function_name: _,
                argument_count: _,
            } => stack_trace,
            Self::CalledNonFunction {
                pos: _,
                stack_trace,
            } => stack_trace,
        }
    }

    /// Returns the StackTrace corresponding to this error
    pub fn stack_trace_mut(&mut self) -> &mut StackTrace<'source> {
        match self {
            Self::Type {
                pos: _,
                stack_trace,
                why: _,
            } => stack_trace,
            Self::Mathematical {
                pos: _,
                stack_trace,
                why: _,
            } => stack_trace,
            Self::IdentifierAlreadyDeclared {
                pos: _,
                stack_trace,
                identifier: _,
            } => stack_trace,
            Self::UndefinedReference {
                pos: _,
                stack_trace,
                identifier: _,
            } => stack_trace,
            Self::ResourceUnavailable {
                pos: _,
                stack_trace,
                why: _,
            } => stack_trace,
            Self::InvalidOverload {
                pos: _,
                stack_trace,
                function_name: _,
                argument_count: _,
            } => stack_trace,
            Self::CalledNonFunction {
                pos: _,
                stack_trace,
            } => stack_trace,
        }
    }

    /// Returns the SourceRange corresponding to this error
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            Self::Type {
                pos,
                stack_trace: _,
                why: _,
            } => pos,
            Self::Mathematical {
                pos,
                stack_trace: _,
                why: _,
            } => pos,
            Self::IdentifierAlreadyDeclared {
                pos,
                stack_trace: _,
                identifier: _,
            } => pos,
            Self::UndefinedReference {
                pos,
                stack_trace: _,
                identifier: _,
            } => pos,
            Self::ResourceUnavailable {
                pos,
                stack_trace: _,
                why: _,
            } => pos,
            Self::InvalidOverload {
                pos,
                stack_trace: _,
                function_name: _,
                argument_count: _,
            } => pos,
            Self::CalledNonFunction {
                pos,
                stack_trace: _,
            } => pos,
        }
    }
}

impl Display for RuntimeError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::Type {
                pos: _,
                stack_trace: _,
                why,
            } => why.to_string(),
            Self::Mathematical {
                pos: _,
                stack_trace: _,
                why,
            } => why.to_string(),
            Self::IdentifierAlreadyDeclared {
                pos: _,
                stack_trace: _,
                identifier,
            } => {
                format!("identifier \"{identifier}\" already declared")
            }
            Self::UndefinedReference {
                pos: _,
                stack_trace: _,
                identifier,
            } => {
                format!("identifier \"{identifier}\" is not defined")
            }
            Self::ResourceUnavailable {
                pos: _,
                stack_trace: _,
                why,
            } => why.to_string(),
            Self::InvalidOverload {
                pos: _,
                stack_trace: _,
                function_name,
                argument_count,
            } => {
                format!("no overload for function \"{function_name}\" that takes {argument_count} arguments")
            }
            Self::CalledNonFunction {
                pos: _,
                stack_trace: _,
            } => "attempt to call something which is not a function".to_string(),
        };

        error_formatting::write_error(
            f,
            IcelangErrorKind::Runtime,
            &description,
            self.pos(),
            Some(self.stack_trace()),
        )
    }
}

impl Error for RuntimeError<'_> {}
