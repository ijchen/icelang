use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A unary operation AST node
///
/// A unary operation is an operation with only one operand
#[derive(Debug, Clone)]
pub struct AstNodeUnaryOperation<'source> {
    operand: Box<AstNode<'source>>,
    operation: UnaryOperationKind,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeUnaryOperation<'source> {
    /// Constructs a new AstNodeUnaryOperation with the given operand, unary
    /// operation kind, and pos
    pub fn new(
        operand: AstNode<'source>,
        operation: UnaryOperationKind,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            operand: Box::new(operand),
            operation,
            pos,
        }
    }

    /// Returns the operand of the unary operation node
    pub fn operand(&self) -> &AstNode<'source> {
        &self.operand
    }

    /// Returns the operation of the unary operation node
    pub fn operation(&self) -> UnaryOperationKind {
        self.operation
    }

    /// Returns the position in the source code of this unary operation node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// unary operation node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeUnaryOperation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!("[Unary Operation] {}operand", self.operation),
                vec![self.operand.to_string(),]
            )
        )
    }
}

impl PartialEq for AstNodeUnaryOperation<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.operation == other.operation && self.operand == other.operand
    }
}
impl Eq for AstNodeUnaryOperation<'_> {}
