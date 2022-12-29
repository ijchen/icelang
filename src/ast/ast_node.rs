use std::fmt::{Debug, Display};

/// Represents a node in an abstract syntax tree (AST)
#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    /// An empty AstNode
    Empty,

    /// An AstNode containing multiple statements
    Statements {
        /// The list of statements, in order
        statements: Vec<AstNode>,
    },
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}
