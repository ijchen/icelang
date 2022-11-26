//! Contains code related to tokens (the smallest meaningful individual unit of
//! code)

use crate::source_range::SourceRange;

/// A generic token of any type
#[derive(Debug)]
pub enum Token<'source> {
    /// An identifier token
    Ident(TokenIdent<'source>),
}

/// An identifier token
#[derive(Debug)]
pub struct TokenIdent<'source> {
    ident: String,
    pos: SourceRange<'source>,
}

impl<'source> TokenIdent<'source> {
    /// Returns the identifier as a string
    pub fn ident(&self) -> &str {
        &self.ident
    }

    /// Returns a the position in the source code of this identifier
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}
