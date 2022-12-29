//! Contains code related to errors in icelang code

mod error_formatting;
mod lexer_error;
mod parse_error;

use error_formatting::IcelangErrorKind;
pub use lexer_error::LexerError;
pub use parse_error::ParseError;
