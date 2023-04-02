use crate::error::runtime_error::RuntimeError;

use super::jump_statement::JumpStatement;

/// Represents some non-linear control flow, like a bubbling runtime error or
/// a jump statement like `break`, `continue`, or `return`
#[derive(Debug)]
pub enum NonLinearControlFlow<'source> {
    /// A jump statement like `break`, `continue`, or `return`
    JumpStatement(JumpStatement<'source>),

    /// A runtime error that is bubbling up the call stack
    RuntimeError(RuntimeError<'source>),
}

/// A result containing either T or some non-linear control flow
pub type RuntimeResult<'source, T> = Result<T, NonLinearControlFlow<'source>>;
