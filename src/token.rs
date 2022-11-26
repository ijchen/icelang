//! Contains code related to tokens (the smallest meaningful individual unit of
//! code)

/// A generic token of any type
#[derive(Debug)]
pub enum Token {
    /// An identifier token
    Ident(TokenIdent),
}

/// An identifier token
#[derive(Debug)]
pub struct TokenIdent {
    ident: String,
}

impl TokenIdent {
    /// Returns the identifier as a string
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
