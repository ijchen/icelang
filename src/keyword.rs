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
    /// The "int" keyword
    Int,
    /// The "byte" keyword
    Byte,
    /// The "float" keyword
    Float,
    /// The "bool" keyword
    Bool,
    /// The "string" keyword
    String,
    /// The "list" keyword
    List,
    /// The "dict" keyword
    Dict,
    /// The "true" bool keyword literal
    True,
    /// The "false" bool keyword literal
    False,
    /// The "null" keyword
    Null,
    /// The "infinity" float keyword literal
    Infinity,
    /// The "nan" float keyword literal
    Nan,
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
            "int" => Ok(Self::Int),
            "byte" => Ok(Self::Byte),
            "float" => Ok(Self::Float),
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String),
            "list" => Ok(Self::List),
            "dict" => Ok(Self::Dict),
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "null" => Ok(Self::Null),
            "Infinity" => Ok(Self::Infinity),
            "NaN" => Ok(Self::Nan),
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
                Self::Int => "int",
                Self::Byte => "byte",
                Self::Float => "float",
                Self::Bool => "bool",
                Self::String => "string",
                Self::List => "list",
                Self::Dict => "dict",
                Self::True => "true",
                Self::False => "false",
                Self::Null => "null",
                Self::Infinity => "Infinity",
                Self::Nan => "NaN",
            }
        )
    }
}

impl Keyword {
    /// Returns whether or not this keyword can be a keyword literal (null isn't
    /// always, but can be a literal, so returns true)
    pub fn can_be_literal(&self) -> bool {
        match self {
            Keyword::If => false,
            Keyword::Else => false,
            Keyword::Loop => false,
            Keyword::While => false,
            Keyword::For => false,
            Keyword::In => false,
            Keyword::Match => false,
            Keyword::Break => false,
            Keyword::Continue => false,
            Keyword::Return => false,
            Keyword::Fn => false,
            Keyword::Let => false,
            Keyword::Int => false,
            Keyword::Byte => false,
            Keyword::Float => false,
            Keyword::Bool => false,
            Keyword::String => false,
            Keyword::List => false,
            Keyword::Dict => false,
            Keyword::True => true,
            Keyword::False => true,
            Keyword::Null => true,
            Keyword::Infinity => true,
            Keyword::Nan => true,
        }
    }

    /// Returns whether or not this keyword can only be a keyword literal (null
    /// isn't always a literal, so returns false)
    pub fn can_only_be_literal(&self) -> bool {
        match self {
            Keyword::If => false,
            Keyword::Else => false,
            Keyword::Loop => false,
            Keyword::While => false,
            Keyword::For => false,
            Keyword::In => false,
            Keyword::Match => false,
            Keyword::Break => false,
            Keyword::Continue => false,
            Keyword::Return => false,
            Keyword::Fn => false,
            Keyword::Let => false,
            Keyword::Int => false,
            Keyword::Byte => false,
            Keyword::Float => false,
            Keyword::Bool => false,
            Keyword::String => false,
            Keyword::List => false,
            Keyword::Dict => false,
            Keyword::True => true,
            Keyword::False => true,
            Keyword::Null => false,
            Keyword::Infinity => true,
            Keyword::Nan => true,
        }
    }

    /// Returns whether or not this keyword can be a type (null isn't always,
    /// but can be a type, so returns true)
    pub fn can_be_type(&self) -> bool {
        match self {
            Keyword::If => false,
            Keyword::Else => false,
            Keyword::Loop => false,
            Keyword::While => false,
            Keyword::For => false,
            Keyword::In => false,
            Keyword::Match => false,
            Keyword::Break => false,
            Keyword::Continue => false,
            Keyword::Return => false,
            Keyword::Fn => false,
            Keyword::Let => false,
            Keyword::Int => true,
            Keyword::Byte => true,
            Keyword::Float => true,
            Keyword::Bool => true,
            Keyword::String => true,
            Keyword::List => true,
            Keyword::Dict => true,
            Keyword::True => false,
            Keyword::False => false,
            Keyword::Null => true,
            Keyword::Infinity => false,
            Keyword::Nan => false,
        }
    }

    /// Returns whether or not this keyword can only be a type (null isn't
    /// always a type, so returns false)
    pub fn can_only_be_type(&self) -> bool {
        match self {
            Keyword::If => false,
            Keyword::Else => false,
            Keyword::Loop => false,
            Keyword::While => false,
            Keyword::For => false,
            Keyword::In => false,
            Keyword::Match => false,
            Keyword::Break => false,
            Keyword::Continue => false,
            Keyword::Return => false,
            Keyword::Fn => false,
            Keyword::Let => false,
            Keyword::Int => true,
            Keyword::Byte => true,
            Keyword::Float => true,
            Keyword::Bool => true,
            Keyword::String => true,
            Keyword::List => true,
            Keyword::Dict => true,
            Keyword::True => false,
            Keyword::False => false,
            Keyword::Null => false,
            Keyword::Infinity => false,
            Keyword::Nan => false,
        }
    }

    /// Returns the icelang type associated with this keyword, or None if there
    /// isn't an icelang type meaningfully associated with this keyword
    ///
    /// This returns Some(...) for literal keywords like "true" and for type
    /// keywords like "bool"
    pub fn icelang_type(&self) -> Option<IcelangType> {
        match self {
            Keyword::If => None,
            Keyword::Else => None,
            Keyword::Loop => None,
            Keyword::While => None,
            Keyword::For => None,
            Keyword::In => None,
            Keyword::Match => None,
            Keyword::Break => None,
            Keyword::Continue => None,
            Keyword::Return => None,
            Keyword::Fn => None,
            Keyword::Let => None,
            Keyword::Int => Some(IcelangType::Int),
            Keyword::Byte => Some(IcelangType::Byte),
            Keyword::Float => Some(IcelangType::Float),
            Keyword::Bool => Some(IcelangType::Bool),
            Keyword::String => Some(IcelangType::String),
            Keyword::List => Some(IcelangType::List),
            Keyword::Dict => Some(IcelangType::Dict),
            Keyword::True => Some(IcelangType::Bool),
            Keyword::False => Some(IcelangType::Bool),
            Keyword::Null => Some(IcelangType::Null),
            Keyword::Infinity => Some(IcelangType::Float),
            Keyword::Nan => Some(IcelangType::Float),
        }
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
        assert_eq!(Keyword::Int.to_string(), "int");
        assert_eq!(Keyword::Byte.to_string(), "byte");
        assert_eq!(Keyword::Bool.to_string(), "bool");
        assert_eq!(Keyword::Float.to_string(), "float");
        assert_eq!(Keyword::String.to_string(), "string");
        assert_eq!(Keyword::List.to_string(), "list");
        assert_eq!(Keyword::Dict.to_string(), "dict");
        assert_eq!(Keyword::True.to_string(), "true");
        assert_eq!(Keyword::False.to_string(), "false");
        assert_eq!(Keyword::Null.to_string(), "null");
        assert_eq!(Keyword::Infinity.to_string(), "Infinity");
        assert_eq!(Keyword::Nan.to_string(), "NaN");
    }

    #[test]
    fn test_keyword_from_str() {
        assert_eq!(Ok(Keyword::If), Keyword::try_from("if"));
        assert_eq!(Ok(Keyword::Else), Keyword::try_from("else"));
        assert_eq!(Ok(Keyword::Loop), Keyword::try_from("loop"));
        assert_eq!(Ok(Keyword::While), Keyword::try_from("while"));
        assert_eq!(Ok(Keyword::For), Keyword::try_from("for"));
        assert_eq!(Ok(Keyword::In), Keyword::try_from("in"));
        assert_eq!(Ok(Keyword::Match), Keyword::try_from("match"));
        assert_eq!(Ok(Keyword::Break), Keyword::try_from("break"));
        assert_eq!(Ok(Keyword::Continue), Keyword::try_from("continue"));
        assert_eq!(Ok(Keyword::Return), Keyword::try_from("return"));
        assert_eq!(Ok(Keyword::Fn), Keyword::try_from("fn"));
        assert_eq!(Ok(Keyword::Let), Keyword::try_from("let"));
        assert_eq!(Ok(Keyword::Int), Keyword::try_from("int"));
        assert_eq!(Ok(Keyword::Byte), Keyword::try_from("byte"));
        assert_eq!(Ok(Keyword::Float), Keyword::try_from("float"));
        assert_eq!(Ok(Keyword::Bool), Keyword::try_from("bool"));
        assert_eq!(Ok(Keyword::String), Keyword::try_from("string"));
        assert_eq!(Ok(Keyword::List), Keyword::try_from("list"));
        assert_eq!(Ok(Keyword::Dict), Keyword::try_from("dict"));
        assert_eq!(Ok(Keyword::True), Keyword::try_from("true"));
        assert_eq!(Ok(Keyword::False), Keyword::try_from("false"));
        assert_eq!(Ok(Keyword::Null), Keyword::try_from("null"));
        assert_eq!(Ok(Keyword::Infinity), Keyword::try_from("Infinity"));
        assert_eq!(Ok(Keyword::Nan), Keyword::try_from("NaN"));
        assert_eq!(Err(()), Keyword::try_from("asdf"));
        assert_eq!(Err(()), Keyword::try_from(""));
        assert_eq!(Err(()), Keyword::try_from("looop"));
        assert_eq!(Err(()), Keyword::try_from("reutrn"));
        assert_eq!(Err(()), Keyword::try_from("functino"));
        assert_eq!(Err(()), Keyword::try_from("flase"));
        assert_eq!(Err(()), Keyword::try_from("True"));
        assert_eq!(Err(()), Keyword::try_from("False"));
        assert_eq!(Err(()), Keyword::try_from("TRUE"));
        assert_eq!(Err(()), Keyword::try_from("FALSE"));
        assert_eq!(Err(()), Keyword::try_from("infinity"));
        assert_eq!(Err(()), Keyword::try_from("INFINITY"));
        assert_eq!(Err(()), Keyword::try_from("nan"));
        assert_eq!(Err(()), Keyword::try_from("NAN"));
        assert_eq!(Err(()), Keyword::try_from("Nan"));
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // I find this more readable
    fn test_keyword_can_be_literal() {
        assert_eq!(Keyword::If.can_be_literal(), false);
        assert_eq!(Keyword::Else.can_be_literal(), false);
        assert_eq!(Keyword::Loop.can_be_literal(), false);
        assert_eq!(Keyword::While.can_be_literal(), false);
        assert_eq!(Keyword::For.can_be_literal(), false);
        assert_eq!(Keyword::In.can_be_literal(), false);
        assert_eq!(Keyword::Match.can_be_literal(), false);
        assert_eq!(Keyword::Break.can_be_literal(), false);
        assert_eq!(Keyword::Continue.can_be_literal(), false);
        assert_eq!(Keyword::Return.can_be_literal(), false);
        assert_eq!(Keyword::Fn.can_be_literal(), false);
        assert_eq!(Keyword::Let.can_be_literal(), false);
        assert_eq!(Keyword::Int.can_be_literal(), false);
        assert_eq!(Keyword::Byte.can_be_literal(), false);
        assert_eq!(Keyword::Float.can_be_literal(), false);
        assert_eq!(Keyword::Bool.can_be_literal(), false);
        assert_eq!(Keyword::String.can_be_literal(), false);
        assert_eq!(Keyword::List.can_be_literal(), false);
        assert_eq!(Keyword::Dict.can_be_literal(), false);
        assert_eq!(Keyword::True.can_be_literal(), true);
        assert_eq!(Keyword::False.can_be_literal(), true);
        assert_eq!(Keyword::Null.can_be_literal(), true);
        assert_eq!(Keyword::Infinity.can_be_literal(), true);
        assert_eq!(Keyword::Nan.can_be_literal(), true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // I find this more readable
    fn test_keyword_can_only_be_literal() {
        assert_eq!(Keyword::If.can_only_be_literal(), false);
        assert_eq!(Keyword::Else.can_only_be_literal(), false);
        assert_eq!(Keyword::Loop.can_only_be_literal(), false);
        assert_eq!(Keyword::While.can_only_be_literal(), false);
        assert_eq!(Keyword::For.can_only_be_literal(), false);
        assert_eq!(Keyword::In.can_only_be_literal(), false);
        assert_eq!(Keyword::Match.can_only_be_literal(), false);
        assert_eq!(Keyword::Break.can_only_be_literal(), false);
        assert_eq!(Keyword::Continue.can_only_be_literal(), false);
        assert_eq!(Keyword::Return.can_only_be_literal(), false);
        assert_eq!(Keyword::Fn.can_only_be_literal(), false);
        assert_eq!(Keyword::Let.can_only_be_literal(), false);
        assert_eq!(Keyword::Int.can_only_be_literal(), false);
        assert_eq!(Keyword::Byte.can_only_be_literal(), false);
        assert_eq!(Keyword::Float.can_only_be_literal(), false);
        assert_eq!(Keyword::Bool.can_only_be_literal(), false);
        assert_eq!(Keyword::String.can_only_be_literal(), false);
        assert_eq!(Keyword::List.can_only_be_literal(), false);
        assert_eq!(Keyword::Dict.can_only_be_literal(), false);
        assert_eq!(Keyword::True.can_only_be_literal(), true);
        assert_eq!(Keyword::False.can_only_be_literal(), true);
        assert_eq!(Keyword::Null.can_only_be_literal(), false);
        assert_eq!(Keyword::Infinity.can_only_be_literal(), true);
        assert_eq!(Keyword::Nan.can_only_be_literal(), true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // I find this more readable
    fn test_keyword_can_be_type() {
        assert_eq!(Keyword::If.can_be_type(), false);
        assert_eq!(Keyword::Else.can_be_type(), false);
        assert_eq!(Keyword::Loop.can_be_type(), false);
        assert_eq!(Keyword::While.can_be_type(), false);
        assert_eq!(Keyword::For.can_be_type(), false);
        assert_eq!(Keyword::In.can_be_type(), false);
        assert_eq!(Keyword::Match.can_be_type(), false);
        assert_eq!(Keyword::Break.can_be_type(), false);
        assert_eq!(Keyword::Continue.can_be_type(), false);
        assert_eq!(Keyword::Return.can_be_type(), false);
        assert_eq!(Keyword::Fn.can_be_type(), false);
        assert_eq!(Keyword::Let.can_be_type(), false);
        assert_eq!(Keyword::Int.can_be_type(), true);
        assert_eq!(Keyword::Byte.can_be_type(), true);
        assert_eq!(Keyword::Float.can_be_type(), true);
        assert_eq!(Keyword::Bool.can_be_type(), true);
        assert_eq!(Keyword::String.can_be_type(), true);
        assert_eq!(Keyword::List.can_be_type(), true);
        assert_eq!(Keyword::Dict.can_be_type(), true);
        assert_eq!(Keyword::True.can_be_type(), false);
        assert_eq!(Keyword::False.can_be_type(), false);
        assert_eq!(Keyword::Null.can_be_type(), true);
        assert_eq!(Keyword::Infinity.can_be_type(), false);
        assert_eq!(Keyword::Nan.can_be_type(), false);
    }

    #[test]
    fn test_keyword_literal_icelang_type() {
        assert_eq!(Keyword::True.icelang_type(), Some(IcelangType::Bool));
        assert_eq!(Keyword::False.icelang_type(), Some(IcelangType::Bool));
        assert_eq!(Keyword::Null.icelang_type(), Some(IcelangType::Null));
        assert_eq!(Keyword::Infinity.icelang_type(), Some(IcelangType::Float));
        assert_eq!(Keyword::Nan.icelang_type(), Some(IcelangType::Float));
    }

    #[test]
    fn test_keyword_type_icelang_type() {
        assert_eq!(Keyword::Int.icelang_type(), Some(IcelangType::Int));
        assert_eq!(Keyword::Byte.icelang_type(), Some(IcelangType::Byte));
        assert_eq!(Keyword::Float.icelang_type(), Some(IcelangType::Float));
        assert_eq!(Keyword::Bool.icelang_type(), Some(IcelangType::Bool));
        assert_eq!(Keyword::String.icelang_type(), Some(IcelangType::String));
        assert_eq!(Keyword::List.icelang_type(), Some(IcelangType::List));
        assert_eq!(Keyword::Dict.icelang_type(), Some(IcelangType::Dict));
        assert_eq!(Keyword::Null.icelang_type(), Some(IcelangType::Null));
    }
}
