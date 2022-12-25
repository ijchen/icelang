//! Contains code related to icelang keywords

use std::fmt::Display;

use enum_iterator::Sequence;

use crate::icelang_type::IcelangType;

/// Represents an icelang keyword
#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq)]
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
    /// The "match" keyword
    Match,
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
            "match" => Ok(Self::Match),
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
                Self::Match => "match",
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
    pub fn icelang_type(&self) -> IcelangType {
        match self {
            Self::True => IcelangType::Bool,
            Self::False => IcelangType::Bool,
            Self::Null => IcelangType::Null,
            Self::Infinity => IcelangType::Float,
            Self::Nan => IcelangType::Float,
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

// TODO unit tests
