use crate::{
    error::runtime_error::RuntimeError,
    interpreter::{NonLinearControlFlow, RuntimeResult},
    runtime_state::RuntimeState,
    source_range::SourceRange,
    value::Value,
};

/// The `typeof` icelang standard library function
pub fn isl_typeof<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => Ok(Value::String(
            match arguments[0] {
                Value::Int(_) => "int",
                Value::Byte(_) => "byte",
                Value::Float(_) => "float",
                Value::Bool(_) => "bool",
                Value::String(_) => "string",
                Value::List(_) => "list",
                Value::Dict(_) => "dict",
                Value::Null => "null",
            }
            .to_string()
            .into(),
        )),
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "typeof".to_string(),
                argument_count,
            ),
        )),
    }
}
