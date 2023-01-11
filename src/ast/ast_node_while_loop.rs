use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A while loop AST node
#[derive(Debug)]
pub struct AstNodeWhileLoop<'source> {
    condition: Box<AstNode<'source>>,
    body: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeWhileLoop<'source> {
    /// Constructs a new AstNodeWhileLoop with the given condition, body, and
    /// pos
    pub fn new(
        condition: AstNode<'source>,
        body: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            body,
            pos,
        }
    }

    /// Returns the condition of the while loop node
    pub fn condition(&self) -> &AstNode {
        &self.condition
    }

    /// Returns the body of the while loop node
    pub fn body(&self) -> &Vec<AstNode> {
        &self.body
    }

    /// Returns the position in the source code of this while loop node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// while loop node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeWhileLoop<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[While loop]",
                vec![
                    format_as_node("Condition", vec![self.condition.to_string()]),
                    format_as_node(
                        "Body",
                        self.body.iter().map(|node| node.to_string()).collect()
                    )
                ]
            )
        )
    }
}

impl PartialEq for AstNodeWhileLoop<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.condition == other.condition && self.body == other.body
    }
}
impl Eq for AstNodeWhileLoop<'_> {}
