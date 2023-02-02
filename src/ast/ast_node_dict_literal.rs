use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A dict literal AST node
#[derive(Debug, Clone)]
pub struct AstNodeDictLiteral<'source> {
    entries: Vec<(AstNode<'source>, AstNode<'source>)>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeDictLiteral<'source> {
    /// Constructs a new AstNodeDictLiteral with the given entries and pos
    pub fn new(
        entries: Vec<(AstNode<'source>, AstNode<'source>)>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self { entries, pos }
    }

    /// Returns the entries of the dict literal node
    pub fn entries(&self) -> &Vec<(AstNode<'source>, AstNode<'source>)> {
        &self.entries
    }

    /// Returns the position in the source code of this dict literal node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// dict literal node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeDictLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Literal] (dict)",
                self.entries
                    .iter()
                    .map(|entry| format_as_node(
                        "Entry",
                        vec![
                            format_as_node("Key", vec![entry.0.to_string()]),
                            format_as_node("Value", vec![entry.1.to_string()])
                        ]
                    ))
                    .collect()
            )
        )
    }
}

impl PartialEq for AstNodeDictLiteral<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}
impl Eq for AstNodeDictLiteral<'_> {}
