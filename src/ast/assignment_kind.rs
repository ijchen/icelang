use std::fmt::Display;

/// The kind of a unary operation
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AssignmentKind {
    /// The normal (=) assignment expression
    Normal,
    /// The plus (+=) augmented assignment expression
    Plus,
    /// The minus (-=) augmented assignment expression
    Minus,
    /// The times (*=) augmented assignment expression
    Times,
    /// The div (/=) augmented assignment expression
    Div,
    /// The mod (%=) augmented assignment expression
    Mod,
    /// The exp (**=) augmented assignment expression
    Exp,
    /// The left-shift (<<=) augmented assignment expression
    Shl,
    /// The right-shift (>>=) augmented assignment expression
    Shr,
    /// The bitwise and (&=) augmented assignment expression
    BitAnd,
    /// The bitwise xor (^=) augmented assignment expression
    BitXor,
    /// The bitwise or (|=) augmented assignment expression
    BitOr,
    /// The logical and (&&=) augmented assignment expression
    LogAnd,
    /// The logical or (||=) augmented assignment expression
    LogOr,
}

impl Display for AssignmentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Normal => "=",
                Self::Plus => "+=",
                Self::Minus => "-=",
                Self::Times => "*=",
                Self::Div => "/=",
                Self::Mod => "%=",
                Self::Exp => "**=",
                Self::Shl => "<<=",
                Self::Shr => ">>=",
                Self::BitAnd => "&=",
                Self::BitXor => "^=",
                Self::BitOr => "|=",
                Self::LogAnd => "&&=",
                Self::LogOr => "||=",
            }
        )
    }
}
