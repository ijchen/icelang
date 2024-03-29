use std::{error::Error, fmt::Display};

use crate::source_range::SourceRange;

use super::*;

/// Represents an error that occurred during lexing
#[derive(Debug)]
pub enum LexerError<'source> {
    /// An illegal character was encountered (the character is either not
    /// allowed at all in icelang source code, or was not used in a valid
    /// position)
    IllegalChar {
        /// The illegal character
        character: char,
        /// The position of the illegal character
        pos: SourceRange<'source>,
    },
    /// An unexpected end-of-file (EOF) was encountered (for some reason, more
    /// characters were necessary in order for the icelang source code to be
    /// valid)
    UnexpectedEOF {
        /// A description of why the EOF was unexpected
        why: String,
        /// The position of the unexpected EOF (possibly including context from
        /// before the EOF of the token that was being built)
        pos: SourceRange<'source>,
    },
    /// An invalid literal was encountered
    InvalidLiteral {
        /// The position of the invalid literal
        pos: SourceRange<'source>,
    },
    /// An invalid escape sequence was encountered in a string literal
    InvalidEscapeSequence {
        /// The position of the invalid escape sequence
        pos: SourceRange<'source>,
    },
}

impl<'source> LexerError<'source> {
    /// Constructs a new IllegalChar LexerError
    pub fn new_illegal_char(character: char, pos: SourceRange<'source>) -> Self {
        Self::IllegalChar { character, pos }
    }

    /// Constructs a new UnexpectedEOF LexerError
    pub fn new_unexpected_eof(why: String, pos: SourceRange<'source>) -> Self {
        Self::UnexpectedEOF { why, pos }
    }

    /// Constructs a new InvalidLiteral LexerError
    pub fn new_invalid_literal(pos: SourceRange<'source>) -> Self {
        Self::InvalidLiteral { pos }
    }

    /// Constructs a new InvalidEscapeSequence LexerError
    pub fn new_invalid_escape_sequence(pos: SourceRange<'source>) -> Self {
        Self::InvalidEscapeSequence { pos }
    }

    /// Returns the SourceRange corresponding to this error
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            Self::IllegalChar { character: _, pos } => pos,
            Self::InvalidEscapeSequence { pos } => pos,
            Self::InvalidLiteral { pos } => pos,
            Self::UnexpectedEOF { why: _, pos } => pos,
        }
    }
}

impl Display for LexerError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            LexerError::IllegalChar {
                character: c,
                pos: _,
            } => match *c {
                '\n' => "illegal character '\\n'".to_string(),
                ' '..='~' => format!("illegal character '{c}'"),
                c => format!("illegal character '{c}' (0x{:0X})", c as u32),
            },
            LexerError::UnexpectedEOF { why, pos: _ } => {
                format!("unexpected end-of-file ({why})")
            }
            LexerError::InvalidLiteral { pos: _ } => "invalid literal".to_string(),
            LexerError::InvalidEscapeSequence { pos: _ } => {
                "invalid escape sequence in string literal".to_string()
            }
        };

        error_formatting::write_error(f, IcelangErrorKind::Syntax, &description, self.pos(), None)
    }
}

impl Error for LexerError<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_illegal_char_display() {
        let mut rng = make_rng();
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let chars = (' '..='~')
            .chain("\t\r\n\0ⱳ⛩⎃Ⅷℕ✫⽑▯∰⨡⿊".chars())
            .chain(std::iter::repeat_with(|| gen_rand_char(&mut rng)).take(RAND_ITERATIONS));

        for ch in chars {
            let le = LexerError::new_illegal_char(ch, nowhere.clone());
            assert!(le.to_string().contains("illegal character"));
            match ch {
                '\n' => {
                    assert!(le.to_string().contains("\\n"));
                }
                ' '..='~' => {
                    assert!(le.to_string().contains(ch));
                }
                ch => {
                    assert!(le.to_string().contains(ch));
                    assert!(le.to_string().contains(&format!("0x{:0X}", ch as u32)));
                }
            }
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
            let le = LexerError::new_unexpected_eof(reason.to_string(), nowhere.clone());

            assert!(le.to_string().contains(reason));
        }
    }

    #[test]
    fn test_invalid_literal_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);
        let le = LexerError::new_invalid_literal(nowhere);

        assert!(le.to_string().contains("invalid literal"));
    }

    #[test]
    fn test_invalid_escape_sequence_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);
        let le = LexerError::new_invalid_escape_sequence(nowhere);

        assert!(le
            .to_string()
            .contains("invalid escape sequence in string literal"));
    }
}
