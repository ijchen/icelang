use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A match statement AST node
#[derive(Debug, Clone)]
pub struct AstNodeMatchStatement<'source> {
    matched_expression: Box<AstNode<'source>>,
    arms: Vec<MatchArm<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeMatchStatement<'source> {
    /// Constructs a new AstNodeMatchStatement with the given matched
    /// expression, match arms, and pos
    pub fn new(
        matched_expression: AstNode<'source>,
        arms: Vec<MatchArm<'source>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            matched_expression: Box::new(matched_expression),
            arms,
            pos,
        }
    }

    /// Returns the matched expression of the match statement
    pub fn matched_expression(&self) -> &AstNode {
        &self.matched_expression
    }

    /// Returns the match arms of the match statement
    pub fn arms(&self) -> &Vec<MatchArm<'source>> {
        &self.arms
    }

    /// Returns the position in the source code of this match statement
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// match statement
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeMatchStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Match Statement]",
                vec![
                    format_as_node(
                        "Matched expression",
                        vec![self.matched_expression.to_string()]
                    ),
                    format_as_node(
                        "Match arms",
                        self.arms
                            .iter()
                            .map(|arm| {
                                format_as_node(
                                    "Match arm",
                                    vec![
                                        format_as_node("Pattern", vec![arm.pattern().to_string()]),
                                        format_as_node(
                                            "Body",
                                            arm.body()
                                                .iter()
                                                .map(|node| node.to_string())
                                                .collect(),
                                        ),
                                    ],
                                )
                            })
                            .collect()
                    )
                ]
            )
        )
    }
}

impl PartialEq for AstNodeMatchStatement<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.matched_expression == other.matched_expression && self.arms == other.arms
    }
}
impl Eq for AstNodeMatchStatement<'_> {}
