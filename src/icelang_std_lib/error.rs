use crate::{
    error::runtime_error::RuntimeError,
    interpreter::{NonLinearControlFlow, RuntimeResult},
    runtime_state::RuntimeState,
    source_range::SourceRange,
    value::Value,
};

/// The `error` icelang standard library function
pub fn isl_error<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_assertion_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "explicit error".to_string(),
            ),
        )),
        1 => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_assertion_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                match &arguments[0] {
                    Value::String(msg) => msg.to_string(),
                    arg => format!(
                        "`error(...)` expects a string as it's first argument, but got a value of type {}",
                        arg.icelang_type()
                    )
                },
            )
        )),
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "error".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `assert` icelang standard library function
pub fn isl_assert<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => match &arguments[0] {
            Value::Bool(true) => Ok(Value::Null),
            Value::Bool(false) => Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_assertion_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    "assertion failed".to_string(),
                ),
            )),
            arg => Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_assertion_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "`assert(...)` expects a bool as it's first argument, but got a value of type {}",
                        arg.icelang_type()
                    )
                )
            )),
        },
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "assert".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `todo` icelang standard library function
pub fn isl_todo<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_assertion_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "execution reached unfinished code".to_string(),
            ),
        )),
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "todo".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `unimplemented` icelang standard library function
pub fn isl_unimplemented<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_assertion_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "execution reached unimplemented code".to_string(),
            ),
        )),
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "unimplemented".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `unreachable` icelang standard library function
pub fn isl_unreachable<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_assertion_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "execution reached unreachable code".to_string(),
            ),
        )),
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "unreachable".to_string(),
                argument_count,
            ),
        )),
    }
}
