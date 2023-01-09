use std::fmt::Display;

/// The kind of a jump statement (like `break`, `continue`, or `return`)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JumpStatementKind {
    /// The `break` jump statement
    Break,

    /// The `continue` jump statement
    Continue,

    /// The `return` jump statement
    Return,
}

impl Display for JumpStatementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JumpStatementKind::Break => "break",
                JumpStatementKind::Continue => "continue",
                JumpStatementKind::Return => "return",
            }
        )
    }
}
