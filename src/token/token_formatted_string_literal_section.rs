use std::fmt::Display;

use crate::source_range::SourceRange;

/// The kind of a formatted string literal section token
///
/// Here are a few example formatted string literals, and their section kinds
/// ```text
/// let msg = f"Hello, {name}! You are {curr_year - birth_year} years old";
/// //        ^^^^^^^^^^    ^^^^^^^^^^^^                      ^^^^^^^^^^^^
/// //           start      continuation                           end
///
/// let my_fstring = f"{9} + {10} = {2 + 2} is a {true} fact, {name}";
/// //               ^^^ ^^^^^   ^^^^^     ^^^^^^^^    ^^^^^^^^^    ^^
/// //                |    |       |          |            |        |
/// //              start  |  continuation    |       continuation  |
/// //                     |                  |                     |
/// //                continuation       continuation              end
///
/// let my_fstring = f"No replacement fields here";
/// //               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// //                          complete          
/// ```
#[derive(Debug, Clone, Copy)]
pub enum FormattedStringLiteralSectionKind {
    /// The start of a formatted string literal, before the first replacement
    /// field
    Start,
    /// A continuation of a formatted string literal, between replacement fields
    Continuation,
    /// The end of a formatted literal string, after the last replacement field
    End,
    /// A complete formatted string literal with no replacement fields
    Complete,
}

impl Display for FormattedStringLiteralSectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start => "start",
                Self::Continuation => "continuation",
                Self::End => "end",
                Self::Complete => "complete",
            }
        )
    }
}

/// A formatted string literal section token
#[derive(Debug)]
pub struct TokenFormattedStringLiteralSection<'source> {
    raw: String,
    kind: FormattedStringLiteralSectionKind,
    pos: SourceRange<'source>,
}

impl<'source> TokenFormattedStringLiteralSection<'source> {
    /// Constructs a new TokenFormattedStringLiteralSection
    pub fn new(
        raw: String,
        kind: FormattedStringLiteralSectionKind,
        pos: SourceRange<'source>,
    ) -> Self {
        Self { raw, kind, pos }
    }

    /// Returns the formatted string literal as a string
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the kind of this formatted string literal section
    pub fn kind(&self) -> FormattedStringLiteralSectionKind {
        self.kind
    }

    /// Returns the position in the source code of this literal
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Display for TokenFormattedStringLiteralSection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Token] Formatted string literal section ({}): {}",
            self.kind, self.raw
        )
    }
}
