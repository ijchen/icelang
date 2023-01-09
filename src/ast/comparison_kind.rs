use std::fmt::Display;

/// The kind of a comparison
#[derive(Debug, PartialEq, Eq)]
pub enum ComparisonKind {
    /// The equal to (==) comparison
    Equal,

    /// The not equal to (!=) comparison
    NotEqual,

    /// The less than (<) comparison
    LessThan,

    /// The greater than (>) comparison
    GreaterThan,

    /// The less than or equal to (<=) comparison
    LessThanOrEqual,

    /// The greater than or equal to (>=) comparison
    GreaterThanOrEqual,
}

impl Display for ComparisonKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ComparisonKind::Equal => "==",
                ComparisonKind::NotEqual => "!=",
                ComparisonKind::LessThan => "<",
                ComparisonKind::GreaterThan => ">",
                ComparisonKind::LessThanOrEqual => "<=",
                ComparisonKind::GreaterThanOrEqual => ">=",
            }
        )
    }
}
