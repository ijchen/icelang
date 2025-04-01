use std::fmt::Display;

use crate::{icelang_type::IcelangType, source_range::SourceRange, value::Value};

/// A literal token
#[derive(Debug)]
pub struct TokenLiteral<'source> {
    raw: String,
    icelang_type: IcelangType,
    value: Value,
    pos: SourceRange<'source>,
}

impl<'source> TokenLiteral<'source> {
    /// Constructs a new TokenLiteral
    ///
    /// # Panics
    /// - If the icelang type provided isn't the same as the icelang type of the
    ///   value
    pub fn new(
        raw: String,
        icelang_type: IcelangType,
        value: Value,
        pos: SourceRange<'source>,
    ) -> Self {
        assert_eq!(value.icelang_type(), icelang_type);

        Self {
            raw,
            icelang_type,
            value,
            pos,
        }
    }

    /// Returns the literal as a string
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the icelang type of this literal
    pub fn icelang_type(&self) -> IcelangType {
        self.icelang_type
    }

    /// Returns the icelang value of this literal
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Returns the position in the source code of this literal
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Display for TokenLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Token] Literal ({}): {}", self.icelang_type, self.raw)
    }
}

#[cfg(test)]
mod tests {
    // use crate::{icelang_type::IcelangType, source_range::SourceRange};

    // use super::*;

    // const LITS: &[(&str, IcelangType)] = &[
    //     ("1330", IcelangType::Int),
    //     ("8bFF", IcelangType::Byte),
    //     ("8b00", IcelangType::Byte),
    //     ("3.14", IcelangType::Float),
    //     ("true", IcelangType::Bool),
    //     ("false", IcelangType::Bool),
    //     ("\"Strange thing this is\"", IcelangType::String),
    //     (":thinking:", IcelangType::List),
    //     ("Merriam-Webster", IcelangType::Dict),
    //     ("null", IcelangType::Null),
    // ];

    // TODO commented out to accelerate refactor, tests need to be updated later
    // #[test]
    // fn test_literal_raw() {
    //     let nowhere = SourceRange::new(" ", "", 0, 0);

    //     for (lit, ty) in LITS {
    //         let tok = TokenLiteral::new(lit.to_string(), *ty, nowhere.clone());

    //         assert_eq!(tok.raw(), *lit);
    //     }
    // }

    // TODO commented out to accelerate refactor, tests need to be updated later
    // #[test]
    // fn test_literal_icelang_type() {
    //     let nowhere = SourceRange::new(" ", "", 0, 0);

    //     for (lit, ty) in LITS {
    //         let tok = TokenLiteral::new(lit.to_string(), *ty, nowhere.clone());

    //         assert_eq!(tok.icelang_type(), *ty);
    //     }
    // }

    // TODO commented out to accelerate refactor, tests need to be updated later
    // #[test]
    // fn test_literal_display() {
    //     let nowhere = SourceRange::new(" ", "", 0, 0);

    //     for (lit, ty) in LITS {
    //         let tok = TokenLiteral::new(lit.to_string(), *ty, nowhere.clone());

    //         assert_eq!(tok.to_string(), format!("[Token] Literal ({ty}): {lit}"));
    //     }
    // }
}
