use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// An assignment AST node
///
/// This is the expression form, not a variable declaration
#[derive(Debug, Clone)]
pub struct AstNodeAssignment<'source> {
    lhs: Box<AstNode<'source>>,
    rhs: Box<AstNode<'source>>,
    assignment_kind: AssignmentKind,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeAssignment<'source> {
    /// Constructs a new AstNodeAssignment with the given left-hand side,
    /// right-hand side, and assignment kind
    pub fn new(
        lhs: AstNode<'source>,
        rhs: AstNode<'source>,
        assignment_kind: AssignmentKind,
    ) -> Self {
        let pos = lhs.pos().extended_to(rhs.pos());
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            assignment_kind,
            pos,
        }
    }

    /// Returns the left-hand side of the assignment node
    pub fn lhs(&self) -> &AstNode<'source> {
        &self.lhs
    }

    /// Returns the right-hand side of the assignment node
    pub fn rhs(&self) -> &AstNode<'source> {
        &self.rhs
    }

    /// Returns the kind of the assignment node
    pub fn assignment_kind(&self) -> AssignmentKind {
        self.assignment_kind
    }

    /// Returns the position in the source code of this assignment node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// assignment node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeAssignment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!("[Assignment] lhs {} rhs", self.assignment_kind),
                vec![self.lhs.to_string(), self.rhs.to_string(),]
            )
        )
    }
}

impl PartialEq for AstNodeAssignment<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.assignment_kind == other.assignment_kind
            && self.lhs == other.lhs
            && self.rhs == other.rhs
    }
}
impl Eq for AstNodeAssignment<'_> {}
