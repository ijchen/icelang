//! Contains code related to lexing (converting source code to tokens)

use std::{error::Error, fmt::Display};

use crate::{
    ice_error::{self, IceErrorType},
    ice_type::IceType,
    source_range::SourceRange,
    token::{KeywordLiteral, Token},
};

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
    /// An unexpected end-of-file (EOF) was encountered (for some reason, more
    /// characters were necessary in order for the ice source code to be valid)
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
                '\n' => "unexpected character '\\n'".to_string(),
                ' '..='~' => format!("unexpected character '{c}'"),
                c => format!("unexpected character '{c}' (0x{:0X})", c as u32),
            },
            LexerError::UnexpectedEOF { why, pos: _ } => {
                format!("unexpected end-of-file ({why})")
            }
            LexerError::InvalidLiteral { pos: _ } => "invalid literal".to_string(),
            LexerError::InvalidEscapeSequence { pos: _ } => {
                "invalid escape sequence in string literal".to_string()
            }
        };

        ice_error::display(f, IceErrorType::Syntax, &description, self.pos(), None)
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
    let mut tokens: Vec<Token> = vec![];

    while index < chars.len() {
        // Ignore whitespace
        if chars[index].is_whitespace() {
            // Do nothing, just advance and continue
            index += 1;
            continue;
        }

        // Ignore comments
        // Block comments
        if matches!((chars[index], chars.get(index + 1)), ('/', Some('*'))) {
            // Store the starting index of the comment (might be needed later in
            // an error message)
            let start_index = index;

            // Advance past the "/*"
            index += 2;
            if index >= chars.len() {
                return Err(LexerError::new_unexpected_eof(
                    "unclosed block comment".to_string(),
                    SourceRange::new(source_code, source_file_name, start_index, index - 1),
                ));
            }

            // Advance until we find a matching "*/"
            while index < chars.len() {
                match (chars[index], chars.get(index + 1)) {
                    // End of comment
                    ('*', Some('/')) => {
                        // Advance past the comment end
                        index += 2;

                        break;
                    }
                    // Part of the comment
                    (_, Some(_)) => {
                        // Advance through the comment
                        index += 1;
                    }
                    // End of file (without a matching comment end)
                    (_, None) => {
                        return Err(LexerError::new_unexpected_eof(
                            "unclosed block comment".to_string(),
                            SourceRange::new(source_code, source_file_name, start_index, index),
                        ));
                    }
                }
            }

            continue;
        }
        // Line comments
        if matches!((chars[index], chars.get(index + 1)), ('/', Some('/'))) {
            // Advance past the comment start
            index += 2;

            // Advance until the next newline, or EOF
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }

            continue;
        }

        // Numeric literals
        if chars[index].is_ascii_digit() {
            // Store the starting index of the literal
            let start_index = index;

            #[derive(Copy, Clone, PartialEq)]
            enum Base {
                Decimal,
                Binary,
                Hex,
                Octal,
            }
            use Base::*;
            #[derive(PartialEq)]
            enum NumericLiteralKind {
                BasedInt(Base, bool),
                BasedByte(Base),
                IntOrFloat,
                Int,
                Float,
            }
            use NumericLiteralKind::*;

            // Identify what kind of numeric literal we're building
            let mut kind: NumericLiteralKind = match (chars[index], chars.get(index + 1)) {
                // Int literal with base
                ('0', Some('d')) => BasedInt(Decimal, false),
                ('0', Some('b')) => BasedInt(Binary, false),
                ('0', Some('x')) => BasedInt(Hex, false),
                ('0', Some('o')) => BasedInt(Octal, false),

                // Byte literal with base
                ('8', Some('d')) => BasedByte(Decimal),
                ('8', Some('b')) => BasedByte(Binary),
                ('8', Some('x')) => BasedByte(Hex),
                ('8', Some('o')) => BasedByte(Octal),

                // Decimal int or float literal
                (_, _) => IntOrFloat,
            };

            let mut literal = String::new();
            let mut literal_has_enough_chars = false;

            // Append the prefix characters, if they exist
            if matches!(kind, BasedByte(_) | BasedInt(_, _)) {
                for _ in 0..2 {
                    literal.push(chars[index]);
                    index += 1;
                }
            }
            // Read successive characters until the literal is complete
            while index < chars.len() {
                match kind {
                    BasedInt(base, _) | BasedByte(base) => {
                        // Ensure the character is valid for the base
                        if match base {
                            Decimal => chars[index].is_ascii_digit(),
                            Hex => chars[index].is_ascii_hexdigit(),
                            Binary => chars[index] == '0' || chars[index] == '1',
                            Octal => matches!(chars[index], '0'..='7'),
                        } {
                            literal.push(chars[index]);
                            index += 1;
                            literal_has_enough_chars = true;
                        } else if chars[index] == '_' {
                            literal.push(chars[index]);
                            index += 1;
                        } else if matches!(kind, BasedInt(Decimal, false)) && chars[index] == 'e' {
                            kind = BasedInt(Decimal, true);
                            literal_has_enough_chars = false;
                            literal.push(chars[index]);
                            index += 1;
                        } else {
                            break;
                        }
                    }
                    IntOrFloat => match chars[index] {
                        c if c.is_ascii_digit() => {
                            literal.push(chars[index]);
                            index += 1;
                            literal_has_enough_chars = true;
                        }
                        '_' => {
                            literal.push(chars[index]);
                            index += 1;
                        }
                        '.' => {
                            // We know now this literal is a float literal
                            kind = Float;
                            literal_has_enough_chars = false;
                            literal.push(chars[index]);
                            index += 1;
                        }
                        'e' => {
                            // We know now this literal is an int literal
                            kind = Int;
                            literal_has_enough_chars = false;
                            literal.push(chars[index]);
                            index += 1;
                        }
                        _ => {
                            break;
                        }
                    },
                    Float => match chars[index] {
                        c if c.is_ascii_digit() => {
                            literal.push(chars[index]);
                            index += 1;
                            literal_has_enough_chars = true;
                        }
                        '_' => {
                            literal.push(chars[index]);
                            index += 1;
                        }
                        'e' => {
                            // If the literal doesn't currently have enough
                            // characters (i.e. if we just hit the '.'), then
                            // we can't have an exponent yet
                            if !literal_has_enough_chars {
                                // Pretend like this "e" is any old invalid
                                // character, since (in this context) it is
                                break;
                            }

                            // Add the 'e'
                            literal.push(chars[index]);
                            index += 1;
                            literal_has_enough_chars = false;

                            if index >= chars.len() {
                                break;
                            }

                            // If the next character is a sign, add that too
                            if matches!(chars.get(index), Some('+' | '-')) {
                                literal.push(chars[index]);
                                index += 1;
                            }

                            // Read the exponent
                            while index < chars.len() {
                                if chars[index].is_ascii_digit() {
                                    literal.push(chars[index]);
                                    index += 1;
                                    literal_has_enough_chars = true;
                                } else if chars[index] == '_' {
                                    literal.push(chars[index]);
                                    index += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                        _ => {
                            break;
                        }
                    },
                    Int => match chars[index] {
                        c if c.is_ascii_digit() => {
                            literal.push(chars[index]);
                            index += 1;
                            literal_has_enough_chars = true;
                        }
                        '_' => {
                            literal.push(chars[index]);
                            index += 1;
                        }
                        _ => {
                            break;
                        }
                    },
                };
            }
            if kind == IntOrFloat {
                kind = Int;
            }

            if !literal_has_enough_chars {
                return Err(LexerError::new_invalid_literal(SourceRange::new(
                    source_code,
                    source_file_name,
                    start_index,
                    index - 1,
                )));
            }

            // Add the new int literal to tokens
            let literal_type = match kind {
                BasedInt(_, _) | Int => IceType::Int,
                BasedByte(_) => IceType::Byte,
                Float => IceType::Float,
                IntOrFloat => unreachable!(),
            };
            let literal_pos =
                SourceRange::new(source_code, source_file_name, start_index, index - 1);
            tokens.push(Token::new_literal(literal, literal_type, literal_pos));

            continue;
        }

        // String literals
        match (chars[index], chars.get(index + 1)) {
            // Normal string literal
            ('"', _) => {
                // Store the starting index of the string literal
                let start_index = index;

                // Read characters into the string literal until we reach the
                // end of it
                let mut string_literal_is_complete = false;
                let mut raw = String::new();
                raw.push(chars[index]);
                index += 1;
                while !string_literal_is_complete && index < chars.len() {
                    match chars[index] {
                        '"' => {
                            // Add the closing quote to the string literal
                            raw.push(chars[index]);
                            index += 1;

                            // The string literal is now complete
                            string_literal_is_complete = true;
                        }
                        '\\' => {
                            // Store the starting index of the escape sequence
                            let escape_sequence_start_index = index;

                            // Add the backslash to the string literal
                            raw.push(chars[index]);
                            index += 1;

                            // If we reached EOF, this literal is invalid
                            if index >= chars.len() {
                                break;
                            }

                            match chars[index] {
                                '"' => {
                                    raw.push('"');
                                    index += 1;
                                }
                                '\\' => {
                                    raw.push('\\');
                                    index += 1;
                                }
                                't' => {
                                    raw.push('\t');
                                    index += 1;
                                }
                                'n' => {
                                    raw.push('\n');
                                    index += 1;
                                }
                                'r' => {
                                    raw.push('\r');
                                    index += 1;
                                }
                                '0' => {
                                    raw.push('\0');
                                    index += 1;
                                }
                                '\n' => {
                                    // The newline was escaped, ignore it
                                    index += 1;
                                }
                                // ASCII escape sequence
                                'x' => {
                                    todo!();
                                }
                                // Unicode escape sequence
                                'u' => {
                                    todo!();
                                }
                                _ => {
                                    return Err(LexerError::new_invalid_escape_sequence(
                                        SourceRange::new(
                                            source_code,
                                            source_file_name,
                                            escape_sequence_start_index,
                                            index,
                                        ),
                                    ));
                                }
                            };
                        }
                        c => {
                            // Add the character to the string literal
                            raw.push(c);
                            index += 1;
                        }
                    };
                }

                // If we reached EOF without completing the string literal,
                // return an error
                if !string_literal_is_complete {
                    return Err(LexerError::new_invalid_literal(SourceRange::new(
                        source_code,
                        source_file_name,
                        start_index,
                        index - 1,
                    )));
                }

                // Add the new string literal to tokens
                tokens.push(Token::new_literal(
                    raw,
                    IceType::String,
                    SourceRange::new(source_code, source_file_name, start_index, index - 1),
                ));

                continue;
            }
            // Raw string literal
            ('r', Some('#' | '"')) => {
                // Store the starting index of the string literal
                let start_index = index;

                // Add the 'r' to the string literal
                let mut raw = String::new();
                raw.push(chars[index]);
                index += 1;

                // Count the number of hashtags for this raw string literal
                let mut hash_count = 0;
                while index < chars.len() && chars[index] == '#' {
                    raw.push(chars[index]);
                    hash_count += 1;
                    index += 1;
                }
                if index < chars.len() && chars[index] == '"' {
                    raw.push(chars[index]);
                    index += 1;
                } else {
                    return Err(LexerError::new_invalid_literal(SourceRange::new(
                        source_code,
                        source_file_name,
                        start_index,
                        index - 1,
                    )));
                }

                // Read characters into the string literal until we reach the
                // end of it
                let mut string_literal_is_complete = false;
                while !string_literal_is_complete && index < chars.len() {
                    match chars[index] {
                        '"' => {
                            // Add the closing quote to the string literal
                            raw.push(chars[index]);
                            index += 1;

                            // Check to see if this is the end of the raw string
                            // literal
                            let mut closing_hashes_needed = hash_count;
                            while closing_hashes_needed > 0
                                && index < chars.len()
                                && chars[index] == '#'
                            {
                                raw.push(chars[index]);
                                closing_hashes_needed -= 1;
                                index += 1;
                            }
                            if closing_hashes_needed == 0 {
                                string_literal_is_complete = true;
                            }
                        }
                        c => {
                            // Add the character to the string literal
                            raw.push(c);
                            index += 1;
                        }
                    };
                }

                // If we reached EOF without completing the string literal,
                // return an error
                if !string_literal_is_complete {
                    return Err(LexerError::new_invalid_literal(SourceRange::new(
                        source_code,
                        source_file_name,
                        start_index,
                        index - 1,
                    )));
                }

                // Add the new string literal to tokens
                tokens.push(Token::new_literal(
                    raw,
                    IceType::String,
                    SourceRange::new(source_code, source_file_name, start_index, index - 1),
                ));

                continue;
            }
            // Format string literal
            ('f', Some('"')) => {
                todo!();

                // continue;
            }
            _ => { /* Not a string literal, carry on */ }
        }

        // Identifiers, keywords, and keyword literals
        if chars[index].is_ascii_alphabetic() || chars[index] == '_' {
            // Store the starting index of the identifier (or keyword (or
            // keyword literal))
            let start_index = index;

            // Keep adding characters until we reach the end of the identifier/
            // keyword/keyword literal
            let mut raw = String::new();
            while index < chars.len() && (chars[index].is_alphanumeric() || chars[index] == '_') {
                raw.push(chars[index]);
                index += 1;
            }

            // Check if the string we've built matches a keyword literal
            if let Ok(keyword_literal) = <&str as TryInto<KeywordLiteral>>::try_into(raw.as_str()) {
                tokens.push(Token::new_literal(
                    keyword_literal.to_string(),
                    keyword_literal.ice_type(),
                    SourceRange::new(source_code, source_file_name, start_index, index - 1),
                ));
                continue;
            };

            // Check if the string we've built matches a keyword
            if let Ok(keyword) = raw.as_str().try_into() {
                tokens.push(Token::new_keyword(
                    keyword,
                    SourceRange::new(source_code, source_file_name, start_index, index - 1),
                ));
                continue;
            };

            // It must be an identifier
            tokens.push(Token::new_ident(
                raw,
                SourceRange::new(source_code, source_file_name, start_index, index - 1),
            ));
            continue;
        }

        // Punctuators
        match (chars[index], chars.get(index + 1), chars.get(index + 2)) {
            ('*', Some('*'), Some('=')) => {
                let start_index = index;
                let mut punctuator = String::with_capacity(3);
                for _ in 0..3 {
                    punctuator.push(chars[index]);
                    index += 1;
                }
                tokens.push(Token::new_punctuator(
                    punctuator,
                    SourceRange::new(source_code, source_file_name, start_index, index - 1),
                ));
                continue;
            }
            ('*', Some('*'), _) | ('+' | '-' | '*' | '/' | '%', Some('='), _) => {
                let start_index = index;
                let mut punctuator = String::with_capacity(2);
                for _ in 0..2 {
                    punctuator.push(chars[index]);
                    index += 1;
                }
                tokens.push(Token::new_punctuator(
                    punctuator,
                    SourceRange::new(source_code, source_file_name, start_index, index - 1),
                ));
                continue;
            }
            (
                ';' | ':' | ',' | '.' | '"' | '(' | ')' | '{' | '}' | '[' | ']' | '=' | '_' | '+'
                | '-' | '*' | '/' | '%',
                _,
                _,
            ) => {
                let start_index = index;
                let mut punctuator = String::with_capacity(1);
                for _ in 0..1 {
                    punctuator.push(chars[index]);
                    index += 1;
                }
                tokens.push(Token::new_punctuator(
                    punctuator,
                    SourceRange::new(source_code, source_file_name, start_index, index - 1),
                ));
                continue;
            }
            _ => { /* Not a punctuator, carry on */ }
        }

        // Invalid characters
        return Err(LexerError::new_illegal_char(
            chars[index],
            SourceRange::new(source_code, source_file_name, index, index),
        ));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};

    const RAND_SEED: u64 = 123;
    const RAND_ITERATIONS: usize = 1000;

    fn gen_rand_char(rng: &mut impl Rng) -> char {
        // Most of the time, we'll just use a normal ASCII value...
        if rng.gen_bool(0.75) {
            rng.gen_range(' '..='~')
        }
        // ...but every now and then, let's mix things up
        else {
            // Sometimes with a completely random character
            if rng.gen_bool(0.9) {
                rng.gen::<char>()
            }
            // And other times with a weird control character
            else {
                rng.gen_range('\0'..=' ')
            }
        }
    }

    #[test]
    fn test_lexer_err_illegal_char_display() {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(RAND_SEED);
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let chars = (' '..='~')
            .chain("\t\r\n\0ⱳ⛩⎃Ⅷℕ✫⽑▯∰⨡⿊".chars())
            .chain(std::iter::repeat_with(|| gen_rand_char(&mut rng)).take(RAND_ITERATIONS));

        for ch in chars {
            let le = LexerError::new_illegal_char(ch, nowhere.clone());
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
    fn test_lexer_err_unexpected_eof_display() {
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
    fn test_tokenize_empty() {
        let source_code = "";
        let source_file_name = "empty.ice";
        let tokens: Vec<Token> = tokenize(source_code, source_file_name).unwrap();

        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_hardcoded() {
        let source_code = "\
// Ignored comment (hopefully?)
/*
 * $ Multiline ★ comment $
 */

    \t21 \t  
  \t
   

foo/* Something */bar
if null true while loop false break NaN return return _ _hi ___hey ___
0 3185415982 0d42 0b101010 0x2A 0xbAD_c0FFeE 0o52 34e6 0d34e6 1_234_567 \
0b1011_0000_0000_1011 0x52_75_73_74_20_3C_33 3__4_e_6_ 0d___3__4_e_6_
8d42 8b101010 8x2A 8o52 8d1_2_8 8b0110_1001 8xF_E 8o3_7_1 8o___3_7_1_____
76.54321 3.14159265358979323 0.0 1.0 0.25 6.67430e-11 0.0000314e+5 0.0000314e5 \
123.456e3 NaN Infinity
true false null
; : , . ( ) { } [ ] =
";
        let source_file_name = "tokens.ice";
        let tokens: Vec<Token> = tokenize(source_code, source_file_name).unwrap();

        let tokens = tokens.into_iter().map(|token| token.to_string());

        let expected = [
            "[Token] Literal (int): 21",
            "[Token] Identifier: foo",
            "[Token] Identifier: bar",
            "[Token] Keyword: if",
            "[Token] Literal (null): null",
            "[Token] Literal (bool): true",
            "[Token] Keyword: while",
            "[Token] Keyword: loop",
            "[Token] Literal (bool): false",
            "[Token] Keyword: break",
            "[Token] Literal (float): NaN",
            "[Token] Keyword: return",
            "[Token] Keyword: return",
            "[Token] Identifier: _",
            "[Token] Identifier: _hi",
            "[Token] Identifier: ___hey",
            "[Token] Identifier: ___",
            "[Token] Literal (int): 0",
            "[Token] Literal (int): 3185415982",
            "[Token] Literal (int): 0d42",
            "[Token] Literal (int): 0b101010",
            "[Token] Literal (int): 0x2A",
            "[Token] Literal (int): 0xbAD_c0FFeE",
            "[Token] Literal (int): 0o52",
            "[Token] Literal (int): 34e6",
            "[Token] Literal (int): 0d34e6",
            "[Token] Literal (int): 1_234_567",
            "[Token] Literal (int): 0b1011_0000_0000_1011",
            "[Token] Literal (int): 0x52_75_73_74_20_3C_33",
            "[Token] Literal (int): 3__4_e_6_",
            "[Token] Literal (int): 0d___3__4_e_6_",
            "[Token] Literal (byte): 8d42",
            "[Token] Literal (byte): 8b101010",
            "[Token] Literal (byte): 8x2A",
            "[Token] Literal (byte): 8o52",
            "[Token] Literal (byte): 8d1_2_8",
            "[Token] Literal (byte): 8b0110_1001",
            "[Token] Literal (byte): 8xF_E",
            "[Token] Literal (byte): 8o3_7_1",
            "[Token] Literal (byte): 8o___3_7_1_____",
            "[Token] Literal (float): 76.54321",
            "[Token] Literal (float): 3.14159265358979323",
            "[Token] Literal (float): 0.0",
            "[Token] Literal (float): 1.0",
            "[Token] Literal (float): 0.25",
            "[Token] Literal (float): 6.67430e-11",
            "[Token] Literal (float): 0.0000314e+5",
            "[Token] Literal (float): 0.0000314e5",
            "[Token] Literal (float): 123.456e3",
            "[Token] Literal (float): NaN",
            "[Token] Literal (float): Infinity",
            "[Token] Literal (bool): true",
            "[Token] Literal (bool): false",
            "[Token] Literal (null): null",
            "[Token] Punctuator: ;",
            "[Token] Punctuator: :",
            "[Token] Punctuator: ,",
            "[Token] Punctuator: .",
            "[Token] Punctuator: (",
            "[Token] Punctuator: )",
            "[Token] Punctuator: {",
            "[Token] Punctuator: }",
            "[Token] Punctuator: [",
            "[Token] Punctuator: ]",
            "[Token] Punctuator: =",
        ];

        for (token, expected) in tokens.zip(expected) {
            assert_eq!(token, expected);
        }

        // TODO continue work on this
    }

    mod test_tokenize_randomized {
        use rand::seq::IteratorRandom;

        use crate::token::Keyword;

        use super::*;

        struct TokenSample {
            raw: String,
            expected: String,
        }

        /// Generates a random sequence of whitespace-like (whitespace or
        /// separating comment) characters
        fn gen_whitespace(rng: &mut impl Rng) -> String {
            let part_count = if rng.gen_bool(0.75) {
                1
            } else {
                rng.gen_range(1..=10)
            };
            let mut raw = String::new();

            for _ in 0..part_count {
                let part = match rng.gen_range(0..4) {
                    0 => " ".to_string(),
                    1 => "\t".to_string(),
                    2 => match rng.gen_bool(0.5) {
                        true => "\n".to_string(),
                        false => "\r\n".to_string(),
                    },
                    3 => {
                        let comment_len = rng.gen_range(0..=50);
                        match rng.gen_bool(0.5) {
                            true => {
                                let mut comment = String::with_capacity(comment_len + 2);

                                comment.push_str("//");
                                for _ in 0..comment_len {
                                    let mut c = gen_rand_char(rng);
                                    while c == '\n' {
                                        c = gen_rand_char(rng);
                                    }
                                    comment.push(c);
                                }
                                if rng.gen_bool(0.5) {
                                    comment.push('\r');
                                }
                                comment.push('\n');

                                comment
                            }
                            false => {
                                let mut comment = String::with_capacity(comment_len + 4);

                                comment.push_str("/*");
                                for _ in 0..comment_len {
                                    let mut c = gen_rand_char(rng);
                                    while c == '\n' || c == '*' {
                                        c = gen_rand_char(rng);
                                    }
                                    comment.push(c);
                                }
                                comment.push_str("*/");
                                if rng.gen_bool(0.5) {
                                    comment.push('\r');
                                }
                                comment.push('\n');

                                comment
                            }
                        }
                    }
                    _ => unreachable!(),
                };

                raw.push_str(&part);
            }

            raw
        }

        fn gen_ident_token_sample(rng: &mut impl Rng) -> TokenSample {
            let len = rng.gen_range(1..4);
            let mut ident = String::with_capacity(len);
            while ident == ""
                || enum_iterator::all::<Keyword>().any(|keyword| keyword.to_string() == ident)
                || enum_iterator::all::<KeywordLiteral>()
                    .any(|keyword_literal| keyword_literal.to_string() == ident)
            {
                ident.clear();
                for i in 0..len {
                    let ident_start = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
                    let ident_cont = &(ident_start.to_string() + "0123456789");
                    let c = if i == 0 { ident_start } else { ident_cont }
                        .chars()
                        .choose(rng)
                        .unwrap();
                    ident.push(c);
                }
            }

            let expected = format!("[Token] Identifier: {ident}");
            let raw = ident;
            TokenSample { raw, expected }
        }

        fn gen_token_sample(rng: &mut impl Rng) -> TokenSample {
            // TODO add all token types
            match rng.gen_range(0..1) {
                0 => gen_ident_token_sample(rng),
                _ => unreachable!(),
            }
        }

        #[test]
        fn test_tokenize_randomized() {
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(RAND_SEED);

            for _ in 0..RAND_ITERATIONS {
                let token_count = rng.gen_range(0..=50);
                let mut generated_source = String::new();
                let mut expected: Vec<String> = Vec::with_capacity(token_count);

                // Construct the source code
                for i in 0..token_count {
                    if i > 0 {
                        generated_source.push_str(&gen_whitespace(&mut rng));
                    }
                    let token_sample = gen_token_sample(&mut rng);
                    generated_source.push_str(&token_sample.raw);
                    expected.push(token_sample.expected);
                }

                let tokens = tokenize(&generated_source, "<test generated source>").unwrap();
                assert_eq!(expected.len(), tokens.len());
                for (token, expected) in tokens.into_iter().zip(expected) {
                    assert_eq!(token.to_string(), expected);
                }
            }
        }
    }
}
