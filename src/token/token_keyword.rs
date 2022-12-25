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

#[cfg(test)]
mod tests {
    use crate::{keyword::Keyword, source_range::SourceRange, token::TokenKeyword};

    #[test]
    fn test_keyword() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        for kw in enum_iterator::all::<Keyword>() {
            let tok: TokenKeyword = TokenKeyword::new(kw, nowhere.clone());

            assert_eq!(tok.keyword(), kw);
        }
    }

    #[test]
    fn test_keyword_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        for kw in enum_iterator::all::<Keyword>() {
            let tok: TokenKeyword = TokenKeyword::new(kw, nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Keyword: {kw}"));
        }
    }
}
