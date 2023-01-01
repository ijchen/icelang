use std::fmt::Display;

use super::*;
use ast_node_format::format_as_node;

/// Represents an Abstract Syntax Tree of some icelang source code
#[derive(Debug, PartialEq, Eq)]
pub struct Ast {
    /// An icelang program is just a collection of statements. These are those
    /// statements, in order
    pub statements: Vec<AstNode>,
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.statements.is_empty() {
            write!(f, "<empty AST>")
        } else {
            write!(
                f,
                "{}",
                format_as_node(
                    "[Program]",
                    self.statements.iter().map(AstNode::to_string).collect()
                )
            )
        }
    }
}
