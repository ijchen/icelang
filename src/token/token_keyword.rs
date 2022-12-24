use std::fmt::Display;

use crate::{keyword::Keyword, source_range::SourceRange};

/// A keyword token
#[derive(Debug)]
pub struct TokenKeyword<'source> {
    keyword: Keyword,
    pos: SourceRange<'source>,
}

impl<'source> TokenKeyword<'source> {
    /// Constructs a new TokenKeyword
    pub fn new(keyword: Keyword, pos: SourceRange<'source>) -> Self {
        Self { keyword, pos }
    }

    /// Returns the keyword as a string
    pub fn keyword(&self) -> Keyword {
        self.keyword
    }

    /// Returns the position in the source code of this keyword
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Display for TokenKeyword<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Token] Keyword: {}", self.keyword)
    }
}
