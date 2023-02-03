use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, AstNode};

#[derive(Debug, Clone)]
/// A function call usage suffix
pub struct AstNodeFunctionCall<'source> {
    root: Box<AstNode<'source>>,
    arguments: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}
impl<'source> AstNodeFunctionCall<'source> {
    /// Constructs a new AstNodeFunctionCall
    pub fn new(
        root: AstNode<'source>,
        arguments: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            root: Box::new(root),
            arguments,
            pos,
        }
    }

    /// Returns the root of the function call node
    pub fn root(&self) -> &AstNode<'source> {
        &self.root
    }

    /// Returns the arguments to the function call
    pub fn arguments(&self) -> &Vec<AstNode<'source>> {
        &self.arguments
    }

    /// Returns the position in the source code of this AstNodeFunctionCall
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// AstNodeFunctionCall
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeFunctionCall<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Function Call]",
                vec![
                    format_as_node("Root", vec![self.root.to_string()]),
                    format_as_node(
                        "Arguments",
                        self.arguments.iter().map(AstNode::to_string).collect()
                    )
                ]
            )
        )
    }
}

impl PartialEq for AstNodeFunctionCall<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.arguments == other.arguments
    }
}
impl Eq for AstNodeFunctionCall<'_> {}
