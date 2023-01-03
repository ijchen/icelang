use std::fmt::Display;

use crate::source_range::SourceRange;

use super::*;
use ast_node_format::format_as_node;

/// A variable access AST node
#[derive(Debug)]
pub struct AstNodeVariableAccess<'source> {
    ident: String,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeVariableAccess<'source> {
    /// Constructs a new AstNodeVariableAccess
    pub fn new(ident: String, pos: SourceRange<'source>) -> Self {
        Self { ident, pos }
    }

    /// Returns the identifier of the variable to access
    pub fn ident(&self) -> &str {
        &self.ident
    }

    /// Returns the position in the source code of this variable access
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// variable access
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl PartialEq for AstNodeVariableAccess<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}
impl Eq for AstNodeVariableAccess<'_> {}

impl Display for AstNodeVariableAccess<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(&format!("[Variable Access] {}", self.ident), vec![])
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_node_display_variable_access() {
        let nowhere = SourceRange::new(" ", "", 0, 0);
        let node: AstNode = AstNodeVariableAccess::new("foo".to_string(), nowhere).into();

        assert_eq!(node.to_string(), "● [Variable Access] foo");
    }

    #[test]
    fn test_ast_node_display_variable_access_long() {
        let nowhere = SourceRange::new(" ", "", 0, 0);
        let node: AstNode = AstNodeVariableAccess::new(
            "__Wow_WHAT_a_long_ANDsTrangeIdentifierWehAVEhERE____".to_string(),
            nowhere,
        )
        .into();

        assert_eq!(
            node.to_string(),
            "● [Variable Access] __Wow_WHAT_a_long_ANDsTrangeIdentifierWehAVEhERE____"
        );
    }
}
