use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A for loop AST node
#[derive(Debug, Clone)]
pub struct AstNodeForLoop<'source> {
    ident: String,
    iterable: Box<AstNode<'source>>,
    body: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeForLoop<'source> {
    /// Constructs a new AstNodeForLoop with the given loop-control variable
    /// identifier, iterable expression, body, and pos
    pub fn new(
        ident: String,
        iterable: AstNode<'source>,
        body: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            ident,
            iterable: Box::new(iterable),
            body,
            pos,
        }
    }

    /// Returns the loop-control variable identifier of the for loop node
    pub fn ident(&self) -> &str {
        &self.ident
    }

    /// Returns the iterable expression of the for loop node
    pub fn iterable(&self) -> &AstNode {
        &self.iterable
    }

    /// Returns the body of the for loop node
    pub fn body(&self) -> &Vec<AstNode> {
        &self.body
    }

    /// Returns the position in the source code of this for loop node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// for loop node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeForLoop<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!("[For loop] for {} in ...", self.ident),
                vec![
                    format_as_node("Iterable", vec![self.iterable.to_string()]),
                    format_as_node(
                        "Body",
                        self.body.iter().map(|node| node.to_string()).collect()
                    )
                ]
            )
        )
    }
}

impl PartialEq for AstNodeForLoop<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.iterable == other.iterable && self.body == other.body
    }
}
impl Eq for AstNodeForLoop<'_> {}
