use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// An inline conditional (often called a
/// ["ternary operator"](https://en.wikipedia.org/wiki/Ternary_conditional_operator)
/// ) expression
#[derive(Debug)]
pub struct AstNodeInlineConditional<'source> {
    condition: Box<AstNode<'source>>,
    truthy_case: Box<AstNode<'source>>,
    falsey_case: Box<AstNode<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeInlineConditional<'source> {
    /// Constructs a new AstNodeInlineConditional with the given condition,
    /// truthy case, and falsey case
    pub fn new(
        condition: AstNode<'source>,
        truthy_case: AstNode<'source>,
        falsey_case: AstNode<'source>,
    ) -> Self {
        let pos = condition.pos().extended_to(falsey_case.pos());
        Self {
            condition: Box::new(condition),
            truthy_case: Box::new(truthy_case),
            falsey_case: Box::new(falsey_case),
            pos,
        }
    }

    /// Returns the condition of the inline conditional node
    pub fn condition(&self) -> &AstNode {
        &self.condition
    }

    /// Returns the truthy case of the inline conditional node
    pub fn truthy_case(&self) -> &AstNode {
        &self.truthy_case
    }

    /// Returns the falsey case of the inline conditional node
    pub fn falsey_case(&self) -> &AstNode {
        &self.falsey_case
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

impl Display for AstNodeInlineConditional<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Inline Conditional]",
                vec![
                    format_as_node("Condition", vec![self.condition.to_string()]),
                    format_as_node("Truthy case", vec![self.truthy_case.to_string()]),
                    format_as_node("Falsey case", vec![self.falsey_case.to_string()]),
                ]
            )
        )
    }
}

impl PartialEq for AstNodeInlineConditional<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.condition == other.condition
            && self.truthy_case == other.truthy_case
            && self.falsey_case == other.falsey_case
    }
}
impl Eq for AstNodeInlineConditional<'_> {}
