use std::fmt::Display;

use crate::{icelang_type::IcelangType, source_range::SourceRange};

/// A literal token
#[derive(Debug)]
pub struct TokenLiteral<'source> {
    raw: String,
    ice_type: IcelangType,
    pos: SourceRange<'source>,
}

impl<'source> TokenLiteral<'source> {
    /// Constructs a new TokeTokenLiteralnIdent
    pub fn new(raw: String, ice_type: IcelangType, pos: SourceRange<'source>) -> Self {
        Self { raw, ice_type, pos }
    }

    /// Returns the literal as a string
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the ice type of this literal
    pub fn ice_type(&self) -> IcelangType {
        self.ice_type
    }

    /// Returns the position in the source code of this literal
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Display for TokenLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Token] Literal ({}): {}", self.ice_type, self.raw)
    }
}
