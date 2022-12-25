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

#[cfg(test)]
mod tests {
    use crate::source_range::SourceRange;

    use super::*;

    const PUNCTUATORS: &[&str] = &["(", ")", "{", "*", "+", "]", "==", "**=", ","];

    #[test]
    fn test_punctuator() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        for punc in PUNCTUATORS {
            let tok = TokenPunctuator::new(punc.to_string(), nowhere.clone());

            assert_eq!(tok.punctuator(), *punc);
        }
    }

    #[test]
    fn test_punctuator_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        for punc in PUNCTUATORS {
            let tok = TokenPunctuator::new(punc.to_string(), nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Punctuator: {punc}"));
        }
    }
}
