use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// An if-else statement AST node
#[derive(Debug, Clone)]
pub struct AstNodeIfElseStatement<'source> {
    conditional_branches: Vec<(AstNode<'source>, Vec<AstNode<'source>>)>,
    else_branch: Option<Vec<AstNode<'source>>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeIfElseStatement<'source> {
    /// Constructs a new AstNodeIfElseStatement with the given conditional
    /// branches, optional else branch, and pos
    pub fn new(
        conditional_branches: Vec<(AstNode<'source>, Vec<AstNode<'source>>)>,
        else_branch: Option<Vec<AstNode<'source>>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            conditional_branches,
            else_branch,
            pos,
        }
    }

    /// Returns the conditional branches of the if-else statement
    pub fn conditional_branches(&self) -> &Vec<(AstNode, Vec<AstNode>)> {
        &self.conditional_branches
    }

    /// Returns the optional else branch of the if-else statement
    pub fn else_branch(&self) -> Option<&Vec<AstNode>> {
        self.else_branch.as_ref()
    }

    /// Returns the position in the source code of this if-else statement
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// if-else statement
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeIfElseStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cap = self.conditional_branches.len();
        if self.else_branch.is_some() {
            cap += 1
        };
        let mut child_nodes = Vec::with_capacity(cap);
        let mut branches = self.conditional_branches().iter();

        // If branch
        if let Some(if_branch) = branches.next() {
            child_nodes.push(format_as_node(
                "If",
                vec![
                    format_as_node("Condition", vec![if_branch.0.to_string()]),
                    format_as_node(
                        "Body",
                        if_branch.1.iter().map(|node| node.to_string()).collect(),
                    ),
                ],
            ));
        }

        // Else-if branches
        for else_if_branch in branches {
            child_nodes.push(format_as_node(
                "Else If",
                vec![
                    format_as_node("Condition", vec![else_if_branch.0.to_string()]),
                    format_as_node(
                        "Body",
                        else_if_branch
                            .1
                            .iter()
                            .map(|node| node.to_string())
                            .collect(),
                    ),
                ],
            ));
        }

        // Else branch
        if let Some(else_branch) = self.else_branch.as_ref() {
            child_nodes.push(format_as_node(
                "Else",
                vec![format_as_node(
                    "Body",
                    else_branch.iter().map(|node| node.to_string()).collect(),
                )],
            ));
        }

        write!(f, "{}", format_as_node("[If-Else Statement]", child_nodes))
    }
}

impl PartialEq for AstNodeIfElseStatement<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.conditional_branches == other.conditional_branches
            && self.else_branch == other.else_branch
    }
}
impl Eq for AstNodeIfElseStatement<'_> {}
