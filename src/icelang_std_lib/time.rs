use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::{
    error::runtime_error::RuntimeError,
    interpreter::{NonLinearControlFlow, RuntimeResult},
    runtime_state::RuntimeState,
    source_range::SourceRange,
    value::Value,
};

/// The `now` icelang standard library function
pub fn isl_now<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => Ok(Value::Int(BigInt::from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_millis())
                .unwrap_or(0),
        ))),
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "now".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `sleep` icelang standard library function
pub fn isl_sleep<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            match &arguments[0] {
                Value::Int(millis) => {
                    // u64::MAX milliseconds is ~584 million years
                    std::thread::sleep(std::time::Duration::from_millis(millis.to_u64().unwrap_or(u64::MAX)));

                    Ok(Value::Null)
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`sleep(...)` expects an int as it's first argument, but got a value of type {}",
                            arguments[0].icelang_type()
                        )
                    )
                )),
            }
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "sleep".to_string(),
                argument_count,
            ),
        )),
    }
}
