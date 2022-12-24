use std::fmt::Display;

use crate::{icelang_type::IcelangType, keyword::Keyword, source_range::SourceRange};

use super::*;

/// A generic token of any type
#[derive(Debug)]
pub enum Token<'source> {
    /// An identifier token
    Ident(TokenIdent<'source>),
    /// A literal token
    Literal(TokenLiteral<'source>),
    /// A section of a formatted string literal
    FormattedStringLiteralSection(TokenFormattedStringLiteralSection<'source>),
    /// A keyword token
    Keyword(TokenKeyword<'source>),
    /// A punctuator (separator or operator) token
    Punctuator(TokenPunctuator<'source>),
}

impl<'source> Token<'source> {
    /// Constructs a new Ident Token
    pub fn new_ident(ident: String, pos: SourceRange<'source>) -> Self {
        Self::Ident(TokenIdent::new(ident, pos))
    }

    /// Constructs a new Literal Token
    pub fn new_literal(raw: String, ice_type: IcelangType, pos: SourceRange<'source>) -> Self {
        Self::Literal(TokenLiteral::new(raw, ice_type, pos))
    }

    /// Constructs a new FormattedStringLiteralSection Token
    pub fn new_formatted_string_literal_section(
        raw: String,
        kind: FormattedStringLiteralSectionKind,
        pos: SourceRange<'source>,
    ) -> Self {
        Self::FormattedStringLiteralSection(TokenFormattedStringLiteralSection::new(raw, kind, pos))
    }

    /// Constructs a new Keyword Token
    pub fn new_keyword(keyword: Keyword, pos: SourceRange<'source>) -> Self {
        Self::Keyword(TokenKeyword::new(keyword, pos))
    }

    /// Constructs a new Punctuator Token
    pub fn new_punctuator(punctuator: String, pos: SourceRange<'source>) -> Self {
        Self::Punctuator(TokenPunctuator::new(punctuator, pos))
    }

    /// Returns the position in the source code of this token
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            Self::Ident(token) => token.pos(),
            Self::Keyword(token) => token.pos(),
            Self::Literal(token) => token.pos(),
            Self::FormattedStringLiteralSection(token) => token.pos(),
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
            Self::FormattedStringLiteralSection(token) => write!(f, "{token}"),
            Self::Punctuator(token) => write!(f, "{token}"),
        }
    }
}
