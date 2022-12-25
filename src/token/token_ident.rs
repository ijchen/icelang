use std::fmt::Display;

use crate::source_range::SourceRange;

/// An identifier token
#[derive(Debug)]
pub struct TokenIdent<'source> {
    ident: String,
    pos: SourceRange<'source>,
}

impl<'source> TokenIdent<'source> {
    /// Constructs a new TokenIdent
    pub fn new(ident: String, pos: SourceRange<'source>) -> Self {
        Self { ident, pos }
    }

    /// Returns the identifier as a string
    pub fn ident(&self) -> &str {
        &self.ident
    }

    /// Returns the position in the source code of this identifier
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Display for TokenIdent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Token] Identifier: {}", self.ident)
    }
}

#[cfg(test)]
mod tests {
    use crate::source_range::SourceRange;

    use super::*;

    const IDENTS: &[&str] = &[
        "foo",
        "bar",
        "snake_case",
        "camelCase",
        "flatcase",
        "UPPERCASE",
        "PascalCase",
        "SCREAMING_SNAKE_CASE",
        "camel_Snake_Case",
        "Pascal_Snake_Case",
        "ujfai83yuafishvf89amhj39vfa87y398asy3vfans3fyfpavws3m78yfams9837vy\
            fhap89ws3y7fma8374hfmva8s7y3fn0vlaifjsp98ufa9ps3j8ufmvioaj8mu38fav9\
            83yua98v3uynf9as8yn398vasyum9faa8s7",
    ];

    #[test]
    fn test_ident() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        for ident in IDENTS {
            let tok: TokenIdent = TokenIdent::new(ident.to_string(), nowhere.clone());

            assert_eq!(tok.ident(), *ident);
        }
    }

    #[test]
    fn test_ident_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        for ident in IDENTS {
            let tok: TokenIdent = TokenIdent::new(ident.to_string(), nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Identifier: {ident}"));
        }
    }
}
