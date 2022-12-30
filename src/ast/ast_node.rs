use std::fmt::{Debug, Display};

use super::ast_node_format::format_as_node;

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
        write!(
            f,
            "{}",
            match self {
                AstNode::Empty => format_as_node("[Empty]", vec![]),
                AstNode::Statements { statements } => format_as_node(
                    "[Statements]",
                    statements.iter().map(AstNode::to_string).collect()
                ),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_empty() {
        let node = AstNode::Empty;

        assert_eq!(node.to_string(), "● [Empty]");
    }

    #[test]
    fn test_display_statements() {
        // TODO make these not all the same once we have more AstNode kinds
        let node = AstNode::Statements {
            statements: vec![AstNode::Empty, AstNode::Empty, AstNode::Empty],
        };

        assert_eq!(
            node.to_string(),
            "\
● [Statements]
├─● [Empty]
├─● [Empty]
└─● [Empty]"
        );
    }
}
