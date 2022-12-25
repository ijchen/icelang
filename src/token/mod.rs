//! Contains code related to tokens (the smallest meaningful individual unit of
//! code)

mod generic_token;
mod token_formatted_string_literal_section;
mod token_ident;
mod token_keyword;
mod token_literal;
mod token_punctuator;

pub use generic_token::Token;
pub use token_formatted_string_literal_section::{
    FormattedStringLiteralSectionKind, TokenFormattedStringLiteralSection,
};
pub use token_ident::TokenIdent;
pub use token_keyword::TokenKeyword;
pub use token_literal::TokenLiteral;
pub use token_punctuator::TokenPunctuator;
