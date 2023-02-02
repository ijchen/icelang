use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, AstNode};

#[derive(Debug, Clone)]
/// A dot member access AST node
pub struct AstNodeDotMemberAccess<'source> {
    root: Box<AstNode<'source>>,
    member: String,
    pos: SourceRange<'source>,
}
impl<'source> AstNodeDotMemberAccess<'source> {
    /// Constructs AstNodeDotMemberAccess
    pub fn new(root: AstNode<'source>, member: String, pos: SourceRange<'source>) -> Self {
        Self {
            root: Box::new(root),
            member,
            pos,
        }
    }

    /// Returns the root of the usage suffix node
    pub fn root(&self) -> &AstNode {
        &self.root
    }

    /// Returns the member to access
    pub fn member(&self) -> &str {
        &self.member
    }

    /// Returns the position in the source code of this AstNodeDotMemberAccess
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// AstNodeDotMemberAccess
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeDotMemberAccess<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Dot Member Access]",
                vec![
                    format_as_node("Root", vec![self.root.to_string()]),
                    format_as_node("Property", vec![self.member.to_string()])
                ]
            )
        )
    }
}

impl PartialEq for AstNodeDotMemberAccess<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.member == other.member
    }
}
impl Eq for AstNodeDotMemberAccess<'_> {}
