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

        /// An explanation of why the type is invalid
        why: String,
    },

    /// A mathematical error occured
    Mathematical {
        /// The position of the error
        pos: SourceRange<'source>,

        /// An explanation of what went wrong
        why: String,
    },

    /// A declaration attempted to declare an identifier which already existed
    IdentifierAlreadyDeclared {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The identifier which was already declared
        identifier: String,
    },

    /// An undefined reference was made
    UndefinedReference {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The identifier which was referenced but not defined
        identifier: String,
    },

    /// A necessary resource was unavailable
    // TODO eventually catch all (if possible, or most) memory/allocation errors
    ResourceUnavailable {
        /// The position of the error
        pos: SourceRange<'source>,

        /// An explanation of what went wrong
        why: String,
    },

    /// A function was called with an invalid number of arguments
    InvalidOverload {
        /// The position of the error
        pos: SourceRange<'source>,

        /// The name of the function
        function_name: String,

        /// The invalid number of arguments provided
        argument_count: usize,
    },
}

impl<'source> RuntimeError<'source> {
    /// Constructs a new Type RuntimeError
    pub fn new_type_error(pos: SourceRange<'source>, why: String) -> Self {
        Self::Type { pos, why }
    }

    /// Constructs a new Mathematical RuntimeError
    pub fn new_mathematical_error(pos: SourceRange<'source>, why: String) -> Self {
        Self::Mathematical { pos, why }
    }

    /// Constructs a new IdentifierAlreadyDeclared RuntimeError
    pub fn new_identifier_already_declared_error(
        pos: SourceRange<'source>,
        identifier: String,
    ) -> Self {
        Self::IdentifierAlreadyDeclared { pos, identifier }
    }

    /// Constructs a new UndefinedReference RuntimeError
    pub fn new_undefined_reference_error(pos: SourceRange<'source>, identifier: String) -> Self {
        Self::UndefinedReference { pos, identifier }
    }

    /// Constructs a new InvalidOverload RuntimeError
    pub fn new_invalid_overload_error(
        pos: SourceRange<'source>,
        function_name: String,
        argument_count: usize,
    ) -> Self {
        Self::InvalidOverload {
            pos,
            function_name,
            argument_count,
        }
    }

    /// Returns the SourceRange corresponding to this error
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            Self::Type { pos, why: _ } => pos,
            Self::Mathematical { pos, why: _ } => pos,
            Self::IdentifierAlreadyDeclared { pos, identifier: _ } => pos,
            Self::UndefinedReference { pos, identifier: _ } => pos,
            Self::ResourceUnavailable { pos, why: _ } => pos,
            Self::InvalidOverload {
                pos,
                function_name: _,
                argument_count: _,
            } => pos,
        }
    }
}

impl Display for RuntimeError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::Type { pos: _, why } => why.to_string(),
            Self::Mathematical { pos: _, why } => why.to_string(),
            Self::IdentifierAlreadyDeclared { pos: _, identifier } => {
                format!("identifier \"{identifier}\" already declared")
            }
            Self::UndefinedReference { pos: _, identifier } => {
                format!("identifier \"{identifier}\" is not defined")
            }
            Self::ResourceUnavailable { pos: _, why } => why.to_string(),
            Self::InvalidOverload {
                pos: _,
                function_name,
                argument_count,
            } => {
                format!("no function overload for {function_name} that takes {argument_count} arguments")
            }
        };

        error_formatting::write_error(f, IcelangErrorKind::Runtime, &description, self.pos(), None)
    }
}

impl Error for RuntimeError<'_> {}
