use std::fmt::Display;

/// The kind of a unary operation
#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperationKind {
    /// The not (!) unary prefix operation
    Not,

    /// The identity (+) unary prefix operation
    Identity,

    /// The negation (-) unary prefix operation
    Negation,
}

impl Display for UnaryOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnaryOperationKind::Not => "!",
                UnaryOperationKind::Identity => "+",
                UnaryOperationKind::Negation => "-",
            }
        )
    }
}
