use std::fmt::Display;

use crate::source_range::SourceRange;

use super::*;

/// A generic token of any kind
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

macro_rules! impl_from_specific_token {
    ($specific_type:ident, $variant_name:ident) => {
        impl<'source> From<$specific_type<'source>> for Token<'source> {
            fn from(token: $specific_type<'source>) -> Self {
                Self::$variant_name(token)
            }
        }
    };
}
impl_from_specific_token!(TokenIdent, Ident);
impl_from_specific_token!(TokenKeyword, Keyword);
impl_from_specific_token!(TokenLiteral, Literal);
impl_from_specific_token!(
    TokenFormattedStringLiteralSection,
    FormattedStringLiteralSection
);
impl_from_specific_token!(TokenPunctuator, Punctuator);
