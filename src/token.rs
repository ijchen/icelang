//! Contains code related to tokens (the smallest meaningful individual unit of
//! code)

use crate::{ice_type::IceType, source_range::SourceRange};

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
    pub fn new_keyword(keyword: String, pos: SourceRange<'source>) -> Self {
        Self::Keyword(TokenKeyword { keyword, pos })
    }

    /// Constructs a new Punctuator Token
    pub fn new_punctuator(punctuator: String, pos: SourceRange<'source>) -> Self {
        Self::Punctuator(TokenPunctuator { punctuator, pos })
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

/// A keyword token
#[derive(Debug)]
pub struct TokenKeyword<'source> {
    keyword: String,
    pos: SourceRange<'source>,
}

impl<'source> TokenKeyword<'source> {
    /// Returns the keyword as a string
    pub fn keyword(&self) -> &str {
        &self.keyword
    }

    /// Returns the position in the source code of this keyword
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
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
