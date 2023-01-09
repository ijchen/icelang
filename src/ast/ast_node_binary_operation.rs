use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A binary operation AST node
///
/// A binary operation is an operation with two operands - a left-hand side
/// (lhs) and a right-hand side (rhs)
#[derive(Debug)]
pub struct AstNodeBinaryOperation<'source> {
    lhs: Box<AstNode<'source>>,
    rhs: Box<AstNode<'source>>,
    operation: BinaryOperationKind,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeBinaryOperation<'source> {
    /// Constructs a new AstNodeBinaryOperation with the given operands and
    /// binary operation kind
    pub fn new(
        lhs: AstNode<'source>,
        rhs: AstNode<'source>,
        operation: BinaryOperationKind,
    ) -> Self {
        let pos = lhs.pos().extended_to(rhs.pos());
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operation,
            pos,
        }
    }

    /// Returns the left-hand side of the binary operation node
    pub fn lhs(&self) -> &AstNode {
        &self.lhs
    }

    /// Returns the right-hand side of the binary operation node
    pub fn rhs(&self) -> &AstNode {
        &self.rhs
    }

    /// Returns the position in the source code of this binary operation node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// binary operation node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeBinaryOperation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!("[Binary Operation] lhs {} rhs", self.operation),
                vec![self.lhs.to_string(), self.rhs.to_string(),]
            )
        )
    }
}

impl PartialEq for AstNodeBinaryOperation<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.operation == other.operation && self.lhs == other.lhs && self.rhs == other.rhs
    }
}
impl Eq for AstNodeBinaryOperation<'_> {}
