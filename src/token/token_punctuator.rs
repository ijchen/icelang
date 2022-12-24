use std::fmt::Display;

use crate::source_range::SourceRange;

/// A punctuator token
#[derive(Debug)]
pub struct TokenPunctuator<'source> {
    punctuator: String,
    pos: SourceRange<'source>,
}

impl<'source> TokenPunctuator<'source> {
    /// Constructs a new TokenPunctuator
    pub fn new(punctuator: String, pos: SourceRange<'source>) -> Self {
        Self { punctuator, pos }
    }

    /// Returns the punctuator as a string
    pub fn punctuator(&self) -> &str {
        &self.punctuator
    }

    /// Returns the position in the source code of this punctuator
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Display for TokenPunctuator<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Token] Punctuator: {}", self.punctuator)
    }
}
