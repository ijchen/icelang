use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A jump statement AST node
#[derive(Debug, Clone)]
pub struct AstNodeJumpStatement<'source> {
    body: Option<Box<AstNode<'source>>>,
    jump_kind: JumpStatementKind,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeJumpStatement<'source> {
    /// Constructs a new AstNodeJumpStatement with the given body, jump kind,
    /// and pos
    pub fn new(
        body: Option<AstNode<'source>>,
        jump_kind: JumpStatementKind,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            body: body.map(Box::new),
            jump_kind,
            pos,
        }
    }

    /// Returns the body of the jump statement node
    pub fn body(&self) -> Option<&AstNode> {
        self.body.as_deref()
    }

    /// Returns the jump kind of the jump statement node
    pub fn jump_kind(&self) -> JumpStatementKind {
        self.jump_kind
    }

    /// Returns the position in the source code of this jump statement node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// jump statement node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeJumpStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!("[Jump Statement] {}", self.jump_kind),
                self.body.iter().map(|node| node.to_string()).collect()
            )
        )
    }
}

impl PartialEq for AstNodeJumpStatement<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body && self.jump_kind == other.jump_kind
    }
}
impl Eq for AstNodeJumpStatement<'_> {}
