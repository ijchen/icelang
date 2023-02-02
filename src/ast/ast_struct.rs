use std::fmt::Display;

use super::*;
use ast_node_format::format_as_node;

/// Represents an Abstract Syntax Tree of some icelang source code
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast<'source> {
    /// An icelang program is just a collection of statements. These are those
    /// statements, in order
    pub statements: Vec<AstNode<'source>>,
}

impl Display for Ast<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_display_empty() {
        let ast = Ast { statements: vec![] };

        assert_eq!(ast.to_string(), "<empty AST>");
    }

    // TODO add more tests once we have more AstNodes
}
