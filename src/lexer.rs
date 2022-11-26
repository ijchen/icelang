//! Contains code related to lexing (converting source code to tokens)

use std::{error::Error, fmt::Display};

use crate::{source_range::SourceRange, token::Token};

/// Represents an error that occurred during lexing
#[derive(Debug)]
pub enum LexerError<'source> {
    /// An illegal character was encountered (the character is either not
    /// allowed at all in ice source code, or was not used in a valid position)
    IllegalChar {
        /// The illegal character
        character: char,
        /// The position of the illegal character
        pos: SourceRange<'source>,
    },
}

impl<'source> LexerError<'source> {
    /// Constructs a new IllegalChar LexerError
    pub fn new_illegal_char(character: char, pos: SourceRange<'source>) -> Self {
        Self::IllegalChar { character, pos }
    }
}

impl Display for LexerError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::IllegalChar {
                character: c,
                pos: _,
            } => match *c {
                ' '..='~' => write!(f, "unexpected character '{c}'"),
                c => write!(f, "unexpected character '{c}' (0x{:0X})", c as u32),
            },
        }
    }
}

impl Error for LexerError<'_> {}

/// Reads some ice source code and produces a list of tokens
pub fn tokenize<'source>(
    source_code: &'source str,
    source_file_name: &'source str,
) -> Result<Vec<Token<'source>>, LexerError<'source>> {
    let chars: Vec<char> = source_code.chars().collect();
    let mut index = 0;
    let tokens: Vec<Token> = vec![]; // TODO shouldn't this need a lifetime annotation?

    while index < chars.len() {
        // Ignore whitespace
        if chars[index].is_whitespace() {
            // Do nothing, just advance and continue
            index += 1;
            continue;
        }

        // Numeric literals
        // if false && chars[index].is_ascii_digit() {
        //     // TODO
        //     todo!();
        // }

        // Identifiers, keywords, and keyword literals
        // if false && (chars[index].is_ascii_alphabetic() || chars[index] == '_') {
        //     // TODO
        //     todo!();
        // }

        // Punctuators
        if false {
            todo!();
        }

        // Invalid characters
        return Err(LexerError::new_illegal_char(
            chars[index],
            SourceRange::new(source_code, source_file_name, index, index),
        ));
    }

    Ok(tokens)
}
