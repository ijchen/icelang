use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, AstNode};

#[derive(Debug, Clone)]
/// A computed member access usage suffix
pub struct AstNodeComputedMemberAccess<'source> {
    root: Box<AstNode<'source>>,
    member_node: Box<AstNode<'source>>,
    pos: SourceRange<'source>,
}
impl<'source> AstNodeComputedMemberAccess<'source> {
    /// Constructs a new AstNodeComputedMemberAccess
    pub fn new(
        root: AstNode<'source>,
        member_node: AstNode<'source>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            root: Box::new(root),
            member_node: Box::new(member_node),
            pos,
        }
    }

    /// Returns the root of the computed member access node
    pub fn root(&self) -> &AstNode<'source> {
        &self.root
    }

    /// Returns the node representing the member to access
    pub fn member_node(&self) -> &AstNode<'source> {
        &self.member_node
    }

    /// Returns the position in the source code of this
    /// AstNodeComputedMemberAccess
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// AstNodeComputedMemberAccess
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeComputedMemberAccess<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Computed Member Access]",
                vec![
                    format_as_node("Root", vec![self.root.to_string()]),
                    format_as_node("Computed Member", vec![self.member_node.to_string()])
                ]
            )
        )
    }
}

impl PartialEq for AstNodeComputedMemberAccess<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.member_node == other.member_node
    }
}
impl Eq for AstNodeComputedMemberAccess<'_> {}
