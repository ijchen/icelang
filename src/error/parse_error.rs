use std::{error::Error, fmt::Display};

use crate::source_range::SourceRange;

use super::*;

/// Represents an error that occurred during parsing
#[derive(Debug)]
pub enum ParseError<'source> {
    /// An unexpected token was encountered
    UnexpectedToken {
        /// A description of why the token was unexpected
        why: String,
        /// The position of the unexpected token
        pos: SourceRange<'source>,
    },
    /// An unexpected end-of-file (EOF) was encountered (for some reason, more tokens were necessary in order for the icelang source code to be valid)
    UnexpectedEOF {
        /// A description of why the EOF was unexpected
        why: String,
        /// The position of the unexpected EOF (possibly including context from
        /// before the EOF of the AstNode that was being built)
        pos: SourceRange<'source>,
    },
}

impl<'source> ParseError<'source> {
    /// Constructs a new UnexpectedToken ParseError
    pub fn new_unexpected_token(why: String, pos: SourceRange<'source>) -> Self {
        Self::UnexpectedToken { why, pos }
    }

    /// Constructs a new UnexpectedEOF ParseError
    pub fn new_unexpected_eof(why: String, pos: SourceRange<'source>) -> Self {
        Self::UnexpectedEOF { why, pos }
    }

    /// Returns the SourceRange corresponding to this error
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            Self::UnexpectedToken { why: _, pos } => pos,
            Self::UnexpectedEOF { why: _, pos } => pos,
        }
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            ParseError::UnexpectedToken { why, pos: _ } => format!("unexpected token ({why})"),
            ParseError::UnexpectedEOF { why, pos: _ } => {
                format!("unexpected end-of-file ({why})")
            }
        };

        error_formatting::write_error(f, IcelangErrorKind::Syntax, &description, self.pos(), None)
    }
}

impl Error for ParseError<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unexpected_token_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let reasons = [
            "Some reason",
            "oashjifn aiuhs4h3 fiasune fikau3hf is",
            "something went wrong",
        ];

        for reason in reasons {
            let parse_error = ParseError::new_unexpected_token(reason.to_string(), nowhere.clone());

            assert!(parse_error.to_string().contains("unexpected token"));
            assert!(parse_error.to_string().contains(reason));
        }
    }

    #[test]
    fn test_unexpected_eof_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let reasons = [
            "Some reason",
            "oashjifn aiuhs4h3 fiasune fikau3hf is",
            "something went wrong",
        ];

        for reason in reasons {
            let parse_error = ParseError::new_unexpected_eof(reason.to_string(), nowhere.clone());

            assert!(parse_error.to_string().contains("unexpected end-of-file"));
            assert!(parse_error.to_string().contains(reason));
        }
    }
}
