use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A while loop AST node
#[derive(Debug)]
pub struct AstNodeFormattedStringLiteral<'source> {
    start: (String, Box<AstNode<'source>>),
    continuations: Vec<(String, Box<AstNode<'source>>)>,
    end: String,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeFormattedStringLiteral<'source> {
    /// Constructs a new AstNodeFormattedStringLiteral with the given start,
    /// continuations, end, and pos
    pub fn new(
        start: (String, AstNode<'source>),
        continuations: Vec<(String, AstNode<'source>)>,
        end: String,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            start: (start.0, Box::new(start.1)),
            continuations: continuations
                .into_iter()
                .map(|(s, r)| (s, Box::new(r)))
                .collect(),
            end,
            pos,
        }
    }

    /// Returns the start of the formatted string literal node
    pub fn start(&self) -> (&String, &AstNode) {
        (&self.start.0, &self.start.1)
    }

    /// Returns the continuations of the formatted string literal node
    pub fn continuations(&self) -> Vec<(&String, &AstNode)> {
        self.continuations
            .iter()
            .map(|(s, r)| (s, r.as_ref()))
            .collect()
    }

    /// Returns the end of the formatted string literal node
    pub fn end(&self) -> &str {
        &self.end
    }

    /// Returns the position in the source code of this formatted string literal
    /// node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// formatted string literal node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeFormattedStringLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!(
                    "[Formatted String Literal] {}...{}{}",
                    self.start.0,
                    self.continuations
                        .iter()
                        .map(|(s, _)| format!("{s}..."))
                        .collect::<String>(),
                    self.end
                ),
                vec![self.start.1.to_string()]
                    .into_iter()
                    .chain(self.continuations.iter().map(|(_, r)| r.to_string()))
                    .collect()
            )
        )
    }
}

impl PartialEq for AstNodeFormattedStringLiteral<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.continuations == other.continuations
            && self.end == other.end
    }
}
impl Eq for AstNodeFormattedStringLiteral<'_> {}
