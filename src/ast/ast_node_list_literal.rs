use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A while loop AST node
#[derive(Debug)]
pub struct AstNodeListLiteral<'source> {
    elements: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeListLiteral<'source> {
    /// Constructs a new AstNodeListLiteral with the given elements and pos
    pub fn new(elements: Vec<AstNode<'source>>, pos: SourceRange<'source>) -> Self {
        Self { elements, pos }
    }

    /// Returns the elements of the list literal node
    pub fn elements(&self) -> &Vec<AstNode> {
        &self.elements
    }

    /// Returns the position in the source code of this list literal node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// list literal node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeListLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Literal] (list)",
                self.elements.iter().map(AstNode::to_string).collect()
            )
        )
    }
}

impl PartialEq for AstNodeListLiteral<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}
impl Eq for AstNodeListLiteral<'_> {}
