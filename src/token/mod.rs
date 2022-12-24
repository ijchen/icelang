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

#[cfg(test)]
mod tests {
    use crate::{icelang_type::IcelangType, keyword::Keyword, source_range::SourceRange};

    use super::*;

    #[test]
    fn test_ident_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let idents = [
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

        for ident in idents {
            let tok = Token::new_ident(ident.to_string(), nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Identifier: {ident}"));
        }
    }

    #[test]
    fn test_literal_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let lits = [
            ("true", IcelangType::Bool),
            ("false", IcelangType::Bool),
            ("8bFF", IcelangType::Byte),
            ("8b00", IcelangType::Byte),
            ("Merriam-Webster", IcelangType::Dict),
            ("3.14", IcelangType::Float),
            ("1330", IcelangType::Int),
            (":thinking:", IcelangType::List),
            ("null", IcelangType::Null),
            ("\"Strange thing this is\"", IcelangType::String),
        ];

        for (lit, ty) in lits {
            let tok = Token::new_literal(lit.to_string(), ty, nowhere.clone());

            assert_eq!(
                tok.to_string(),
                format!("[Token] Literal ({}): {}", ty, lit)
            );
        }
    }

    #[test]
    fn test_keyword_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        for kw in enum_iterator::all::<Keyword>() {
            let tok = Token::new_keyword(kw, nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Keyword: {kw}"));
        }
    }

    #[test]
    fn test_punctuator_display() {
        let nowhere = SourceRange::new(" ", "", 0, 0);

        let puncs = ["(", ")", "{", "*", "+", "]", "==", "**=", ","];

        for punc in puncs {
            let tok = Token::new_punctuator(punc.to_string(), nowhere.clone());

            assert_eq!(tok.to_string(), format!("[Token] Punctuator: {punc}"));
        }
    }
}
