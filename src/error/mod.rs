//! Contains code related to errors in icelang code

mod error_formatting;
mod lexer_error;

use error_formatting::IcelangErrorType;
pub use lexer_error::LexerError;
