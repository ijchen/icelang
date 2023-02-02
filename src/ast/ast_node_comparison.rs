use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A comparison AST node
#[derive(Debug, Clone)]
pub struct AstNodeComparison<'source> {
    first: Box<AstNode<'source>>,
    comparisons: Vec<(ComparisonKind, AstNode<'source>)>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeComparison<'source> {
    /// Constructs a new AstNodeComparison with the given first operand and
    /// subsequent comparisons
    pub fn new(
        first: AstNode<'source>,
        comparisons: Vec<(ComparisonKind, AstNode<'source>)>,
    ) -> Self {
        let pos = if comparisons.is_empty() {
            first.pos().clone()
        } else {
            first
                .pos()
                .extended_to(comparisons[comparisons.len() - 1].1.pos())
        };
        Self {
            first: Box::new(first),
            comparisons,
            pos,
        }
    }

    /// Returns the first operand of the comparison node
    pub fn first(&self) -> &AstNode<'source> {
        &self.first
    }

    /// Returns the subsequent comparisons of the comparison node
    pub fn comparisons(&self) -> &Vec<(ComparisonKind, AstNode<'source>)> {
        &self.comparisons
    }

    /// Returns the position in the source code of this comparison node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// comparison node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeComparison<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node_children = Vec::with_capacity(1 + self.comparisons.len());
        node_children.push(format_as_node(
            "First operand",
            vec![self.first.to_string()],
        ));
        for comparison in &self.comparisons {
            node_children.push(format_as_node(
                &format!("Comparison ({})", comparison.0),
                vec![comparison.1.to_string()],
            ));
        }
        write!(f, "{}", format_as_node("[Comparison]", node_children))
    }
}

impl PartialEq for AstNodeComparison<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.comparisons == other.comparisons
    }
}
impl Eq for AstNodeComparison<'_> {}
