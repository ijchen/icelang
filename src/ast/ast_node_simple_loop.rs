use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A simple (`loop`) loop AST node
#[derive(Debug, Clone)]
pub struct AstNodeSimpleLoop<'source> {
    condition: Option<Box<AstNode<'source>>>,
    body: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeSimpleLoop<'source> {
    /// Constructs a new AstNodeSimpleLoop with the given condition, body, and
    /// pos
    pub fn new(
        condition: Option<AstNode<'source>>,
        body: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            condition: condition.map(Box::new),
            body,
            pos,
        }
    }

    /// Returns the condition of the while loop node
    pub fn condition(&self) -> Option<&AstNode> {
        self.condition.as_deref()
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

impl Display for AstNodeSimpleLoop<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Simple loop]",
                vec![
                    self.condition.as_ref().map(|condition| {
                        format_as_node("Condition", vec![condition.to_string()])
                    }),
                    Some(format_as_node(
                        "Body",
                        self.body.iter().map(|node| node.to_string()).collect()
                    ))
                ]
                .into_iter()
                .flatten()
                .collect()
            )
        )
    }
}

impl PartialEq for AstNodeSimpleLoop<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.condition == other.condition && self.body == other.body
    }
}
impl Eq for AstNodeSimpleLoop<'_> {}
