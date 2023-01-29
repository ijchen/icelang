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

    /// Returns the SourceRange corresponding to this error
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            Self::Type { pos, why: _ } => pos,
            Self::Mathematical { pos, why: _ } => pos,
            Self::IdentifierAlreadyDeclared { pos, identifier: _ } => pos,
            Self::UndefinedReference { pos, identifier: _ } => pos,
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
        };

        error_formatting::write_error(f, IcelangErrorKind::Runtime, &description, self.pos(), None)
    }
}

impl Error for RuntimeError<'_> {}
