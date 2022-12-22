//! Contains code related to tokens (the smallest meaningful individual unit of
//! code)

use std::fmt::Display;

use enum_iterator::Sequence;

use crate::{ice_type::IceType, source_range::SourceRange};

/// Represents an icelang keyword
#[derive(Debug, Clone, Copy, Sequence)]
pub enum Keyword {
    /// The "if" keyword
    If,
    /// The "else" keyword
    Else,
    /// The "loop" keyword
    Loop,
    /// The "while" keyword
    While,
    /// The "for" keyword
    For,
    /// The "in" keyword
    In,
    /// The "break" keyword
    Break,
    /// The "continue" keyword
    Continue,
    /// The "return" keyword
    Return,
    /// The "fn" keyword
    Fn,
    /// The "let" keyword
    Let,
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "loop" => Ok(Self::Loop),
            "while" => Ok(Self::While),
            "for" => Ok(Self::For),
            "in" => Ok(Self::In),
            "break" => Ok(Self::Break),
            "continue" => Ok(Self::Continue),
            "return" => Ok(Self::Return),
            "fn" => Ok(Self::Fn),
            "let" => Ok(Self::Let),
            _ => Err(()),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If => "if",
                Self::Else => "else",
                Self::Loop => "loop",
                Self::While => "while",
                Self::For => "for",
                Self::In => "in",
                Self::Break => "break",
                Self::Continue => "continue",
                Self::Return => "return",
                Self::Fn => "fn",
                Self::Let => "let",
            }
        )
    }
}

/// Represents an icelang keyword literal
#[derive(Debug, Clone, Copy, Sequence)]
pub enum KeywordLiteral {
    /// The "true" bool keyword literal
    True,
    /// The "false" bool keyword literal
    False,
    /// The "null" null keyword literal
    Null,
    /// The "infinity" float keyword literal
    Infinity,
    /// The "nan" float keyword literal
    Nan,
}

impl KeywordLiteral {
    /// Returns the icelang type of the keyword literal
    pub fn ice_type(&self) -> IceType {
        match self {
            Self::True => IceType::Bool,
            Self::False => IceType::Bool,
            Self::Null => IceType::Null,
            Self::Infinity => IceType::Float,
            Self::Nan => IceType::Float,
        }
    }
}

impl TryFrom<&str> for KeywordLiteral {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "null" => Ok(Self::Null),
            "Infinity" => Ok(Self::Infinity),
            "NaN" => Ok(Self::Nan),
            _ => Err(()),
        }
    }
}

impl Display for KeywordLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::True => "true",
                Self::False => "false",
                Self::Null => "null",
                Self::Infinity => "Infinity",
                Self::Nan => "NaN",
            }
        )
    }
}

/// A generic token of any type
#[derive(Debug)]
pub enum Token<'source> {
    /// An identifier token
    Ident(TokenIdent<'source>),
    /// A literal token
    Literal(TokenLiteral<'source>),
    /// A keyword token
    Keyword(TokenKeyword<'source>),
    /// A punctuator (separator or operator) token
    Punctuator(TokenPunctuator<'source>),
}

impl<'source> Token<'source> {
    /// Constructs a new Ident Token
    pub fn new_ident(ident: String, pos: SourceRange<'source>) -> Self {
        Self::Ident(TokenIdent { ident, pos })
    }

    /// Constructs a new Literal Token
    pub fn new_literal(raw: String, ice_type: IceType, pos: SourceRange<'source>) -> Self {
        Self::Literal(TokenLiteral { raw, ice_type, pos })
    }

    /// Constructs a new Keyword Token
    pub fn new_keyword(keyword: Keyword, pos: SourceRange<'source>) -> Self {
        Self::Keyword(TokenKeyword { keyword, pos })
    }

    /// Constructs a new Punctuator Token
    pub fn new_punctuator(punctuator: String, pos: SourceRange<'source>) -> Self {
        Self::Punctuator(TokenPunctuator { punctuator, pos })
    }

    /// Returns the position in the source code of this token
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            Self::Ident(token) => token.pos(),
            Self::Keyword(token) => token.pos(),
            Self::Literal(token) => token.pos(),
            Self::Punctuator(token) => token.pos(),
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(token) => write!(f, "{token}"),
            Self::Keyword(token) => write!(f, "{token}"),
            Self::Literal(token) => write!(f, "{token}"),
            Self::Punctuator(token) => write!(f, "{token}"),
        }
    }
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

    /// Returns the position in the source code of this identifier
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Display for TokenIdent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Token] Identifier: {}", self.ident)
    }
}

/// A literal token
#[derive(Debug)]
pub struct TokenLiteral<'source> {
    raw: String,
    ice_type: IceType,
    pos: SourceRange<'source>,
}

impl<'source> TokenLiteral<'source> {
    /// Returns the literal as a string
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the ice type of this literal
    pub fn ice_type(&self) -> IceType {
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

/// A keyword token
#[derive(Debug)]
pub struct TokenKeyword<'source> {
    keyword: Keyword,
    pos: SourceRange<'source>,
}

impl<'source> TokenKeyword<'source> {
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

/// A punctuator token
#[derive(Debug)]
pub struct TokenPunctuator<'source> {
    punctuator: String,
    pos: SourceRange<'source>,
}

impl<'source> TokenPunctuator<'source> {
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
    use super::*;

    #[test]
    fn test_ident_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let idents = [
            "foo",
            "bar",
            "snake_case",
            "camelCase",
            "flatcase",
            "UPPERCASE",
            "PascalCase",
            "SCREAMING_SNAKE_CASE",
            "camel_Snake_Case",
            "Pascal_Snake_Case",
            "ujfai83yuafishvf89amhj39vfa87y398asy3vfans3fyfpavws3m78yfams9837vy\
            fhap89ws3y7fma8374hfmva8s7y3fn0vlaifjsp98ufa9ps3j8ufmvioaj8mu38fav9\
            83yua98v3uynf9as8yn398vasyum9faa8s7",
        ];

        for ident in idents {
            let tok = Token::new_ident(ident.to_string(), nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Identifier: {ident}"));
        }
    }

    #[test]
    fn test_literal_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let lits = [
            ("true", IceType::Bool),
            ("false", IceType::Bool),
            ("8bFF", IceType::Byte),
            ("8b00", IceType::Byte),
            ("Merriam-Webster", IceType::Dict),
            ("3.14", IceType::Float),
            ("1330", IceType::Int),
            (":thinking:", IceType::List),
            ("null", IceType::Null),
            ("\"Strange thing this is\"", IceType::String),
        ];

        for (lit, ty) in lits {
            let tok = Token::new_literal(lit.to_string(), ty, nowhere.clone());

            assert_eq!(
                tok.to_string(),
                format!("[Token] Literal ({}): {}", ty, lit)
            );
        }
    }

    #[test]
    fn test_keyword_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        for kw in enum_iterator::all::<Keyword>() {
            let tok = Token::new_keyword(kw, nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Keyword: {kw}"));
        }
    }

    #[test]
    fn test_punctuator_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let puncs = ["(", ")", "{", "*", "+", "]", "==", "**=", ","];

        for punc in puncs {
            let tok = Token::new_punctuator(punc.to_string(), nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Punctuator: {punc}"));
        }
    }
}
