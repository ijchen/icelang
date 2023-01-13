//! Contains code related to `RuntimeError`s, which represent errors that
//! occurred during an icelang runtime

use std::{error::Error, fmt::Display};

use crate::{ast::AstNode, source_range::SourceRange};

use super::*;

/// Represents an error that occurred during an icelang runtime
#[derive(Debug)]
pub enum RuntimeError<'source, 'ast> {
    /// A value was an invalid type
    Type {
        /// The position of the error
        pos: SourceRange<'source>,
    },
    /// Something was nonsensical about a node in the given Ast
    MalformedAstNode {
        /// The malformed AstNode
        malformed_node: &'ast AstNode<'source>,

        /// An explanation of why the AstNode is malformed
        why: String,
    },
}

impl<'source, 'ast> RuntimeError<'source, 'ast> {
    /// Constructs a new Type RuntimeError
    pub fn new_type_error(pos: SourceRange<'source>) -> Self {
        Self::Type { pos }
    }

    /// Constructs a new MalformedAstNode RuntimeError
    pub fn new_malformed_ast_node_error(
        malformed_node: &'ast AstNode<'source>,
        why: String,
    ) -> Self {
        Self::MalformedAstNode {
            malformed_node,
            why,
        }
    }

    /// Returns the SourceRange corresponding to this error
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            Self::MalformedAstNode {
                malformed_node,
                why: _,
            } => malformed_node.pos(),
            Self::Type { pos } => pos,
        }
    }
}

impl Display for RuntimeError<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (description, error_kind) = match self {
            RuntimeError::Type { pos: _ } => {
                ("invalid type".to_string(), IcelangErrorKind::Runtime)
            }
            RuntimeError::MalformedAstNode {
                malformed_node: _,
                why,
            } => (
                format!("malformed AST node ({why})"),
                IcelangErrorKind::Syntax,
            ),
        };

        error_formatting::write_error(f, error_kind, &description, self.pos(), None)
    }
}

impl Error for RuntimeError<'_, '_> {}
