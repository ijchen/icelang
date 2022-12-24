use std::fmt::Display;

use crate::source_range::SourceRange;

/// An identifier token
#[derive(Debug)]
pub struct TokenIdent<'source> {
    ident: String,
    pos: SourceRange<'source>,
}

impl<'source> TokenIdent<'source> {
    /// Constructs a new TokenIdent
    pub fn new(ident: String, pos: SourceRange<'source>) -> Self {
        Self { ident, pos }
    }

    /// Returns the identifier as a string
    pub fn ident(&self) -> &str {
        &self.ident
    }

    /// Returns the position in the source code of this identifier
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Display for TokenIdent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Token] Identifier: {}", self.ident)
    }
}
