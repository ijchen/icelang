use std::fmt::Display;

/// The kind of a binary operation
#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperationKind {
    /// The logical or (||) binary operation
    LogicalOr,

    /// The logical and (&&) binary operation
    LogicalAnd,

    /// The bitwise or (|) binary operation
    BitwiseOr,

    /// The bitwise xor (^) binary operation
    BitwiseXor,

    /// The bitwise and (&) binary operation
    BitwiseAnd,

    /// The left shift (<<) binary operation
    ShiftLeft,

    /// The right shift (>>) binary operation
    ShiftRight,

    /// The addition (+) binary operation
    Addition,

    /// The subtraction (-) binary operation
    Subtraction,

    /// The multiplication (*) binary operation
    Multiplication,

    /// The division (/) binary operation
    Division,

    /// The modulo (%) binary operation
    Modulo,

    /// The exponentiation (**) binary operation
    Exponentiation,
}

impl Display for BinaryOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BinaryOperationKind::LogicalOr => "||",
                BinaryOperationKind::LogicalAnd => "&&",
                BinaryOperationKind::BitwiseOr => "|",
                BinaryOperationKind::BitwiseXor => "^",
                BinaryOperationKind::BitwiseAnd => "&",
                BinaryOperationKind::ShiftLeft => "<<",
                BinaryOperationKind::ShiftRight => ">>",
                BinaryOperationKind::Addition => "+",
                BinaryOperationKind::Subtraction => "-",
                BinaryOperationKind::Multiplication => "*",
                BinaryOperationKind::Division => "/",
                BinaryOperationKind::Modulo => "%",
                BinaryOperationKind::Exponentiation => "**",
            }
        )
    }
}
