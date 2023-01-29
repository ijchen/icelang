use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A variable declaration AST node
#[derive(Debug)]
pub struct AstNodeVariableDeclaration<'source> {
    declarations: Vec<(String, Option<AstNode<'source>>, SourceRange<'source>)>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeVariableDeclaration<'source> {
    /// Constructs a new AstNodeVariableDeclaration with the given declarations
    /// and pos
    pub fn new(
        declarations: Vec<(String, Option<AstNode<'source>>, SourceRange<'source>)>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self { declarations, pos }
    }

    /// Returns the list of declarations of the assignment node
    pub fn declarations(&self) -> &Vec<(String, Option<AstNode<'source>>, SourceRange<'source>)> {
        &self.declarations
    }
    /// Returns the position in the source code of this variable declaration node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// variable declaration node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeVariableDeclaration<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Variable Declaration]",
                self.declarations
                    .iter()
                    .map(|(ident, value, _)| format_as_node(
                        if value.is_some() {
                            ident.to_string()
                        } else {
                            format!("{ident} (uninitialized)")
                        }
                        .as_str(),
                        value.iter().map(|node| node.to_string()).collect()
                    ))
                    .collect()
            )
        )
    }
}

impl PartialEq for AstNodeVariableDeclaration<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.declarations
            .iter()
            .zip(other.declarations.iter())
            .all(|((ident1, value1, _), (ident2, value2, _))| ident1 == ident2 && value1 == value2)
    }
}
impl Eq for AstNodeVariableDeclaration<'_> {}
