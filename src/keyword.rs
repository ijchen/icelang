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
#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_display() {
        assert_eq!(Keyword::If.to_string(), "if");
        assert_eq!(Keyword::Else.to_string(), "else");
        assert_eq!(Keyword::Loop.to_string(), "loop");
        assert_eq!(Keyword::While.to_string(), "while");
        assert_eq!(Keyword::For.to_string(), "for");
        assert_eq!(Keyword::In.to_string(), "in");
        assert_eq!(Keyword::Match.to_string(), "match");
        assert_eq!(Keyword::Break.to_string(), "break");
        assert_eq!(Keyword::Continue.to_string(), "continue");
        assert_eq!(Keyword::Return.to_string(), "return");
        assert_eq!(Keyword::Fn.to_string(), "fn");
        assert_eq!(Keyword::Let.to_string(), "let");
    }

    #[test]
    fn test_keyword_from_str() {
        assert_eq!(Ok(Keyword::If), "if".try_into());
        assert_eq!(Ok(Keyword::Else), "else".try_into());
        assert_eq!(Ok(Keyword::Loop), "loop".try_into());
        assert_eq!(Ok(Keyword::While), "while".try_into());
        assert_eq!(Ok(Keyword::For), "for".try_into());
        assert_eq!(Ok(Keyword::In), "in".try_into());
        assert_eq!(Ok(Keyword::Match), "match".try_into());
        assert_eq!(Ok(Keyword::Break), "break".try_into());
        assert_eq!(Ok(Keyword::Continue), "continue".try_into());
        assert_eq!(Ok(Keyword::Return), "return".try_into());
        assert_eq!(Ok(Keyword::Fn), "fn".try_into());
        assert_eq!(Ok(Keyword::Let), "let".try_into());
        assert_eq!(Err::<Keyword, ()>(()), "asdf".try_into());
        assert_eq!(Err::<Keyword, ()>(()), "".try_into());
        assert_eq!(Err::<Keyword, ()>(()), "looop".try_into());
        assert_eq!(Err::<Keyword, ()>(()), "reutrn".try_into());
    }

    #[test]
    fn test_keyword_literal_display() {
        assert_eq!(KeywordLiteral::True.to_string(), "true");
        assert_eq!(KeywordLiteral::False.to_string(), "false");
        assert_eq!(KeywordLiteral::Null.to_string(), "null");
        assert_eq!(KeywordLiteral::Infinity.to_string(), "Infinity");
        assert_eq!(KeywordLiteral::Nan.to_string(), "NaN");
    }

    #[test]
    fn test_keyword_literal_icelang_type() {
        assert_eq!(KeywordLiteral::True.icelang_type(), IcelangType::Bool);
        assert_eq!(KeywordLiteral::False.icelang_type(), IcelangType::Bool);
        assert_eq!(KeywordLiteral::Null.icelang_type(), IcelangType::Null);
        assert_eq!(KeywordLiteral::Infinity.icelang_type(), IcelangType::Float);
        assert_eq!(KeywordLiteral::Nan.icelang_type(), IcelangType::Float);
    }

    #[test]
    fn test_keyword_literal_from_str() {
        assert_eq!(Ok(KeywordLiteral::True), "true".try_into());
        assert_eq!(Ok(KeywordLiteral::False), "false".try_into());
        assert_eq!(Ok(KeywordLiteral::Null), "null".try_into());
        assert_eq!(Ok(KeywordLiteral::Infinity), "Infinity".try_into());
        assert_eq!(Ok(KeywordLiteral::Nan), "NaN".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "asdf".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "flase".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "True".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "False".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "TRUE".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "FALSE".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "infinity".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "nan".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "NAN".try_into());
        assert_eq!(Err::<KeywordLiteral, ()>(()), "Nan".try_into());
    }
}
