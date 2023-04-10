use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::{
    error::runtime_error::RuntimeError,
    interpreter::{NonLinearControlFlow, RuntimeResult},
    runtime_state::RuntimeState,
    source_range::SourceRange,
    value::Value,
};

/// The `from_codepoint` icelang standard library function
pub fn isl_from_codepoint<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            match &arguments[0] {
                Value::Int(codepoint) => {
                    Ok(codepoint
                        .to_u32()
                        .and_then(char::from_u32)
                        .map(|c| Value::String(c.to_string().into()))
                        .unwrap_or(Value::Null)
                    )
                },
                Value::Byte(codepoint) => {
                    Ok(char::from_u32(*codepoint as u32)
                        .map(|c| Value::String(c.to_string().into()))
                        .unwrap_or(Value::Null)
                    )
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`from_codepoint(...)` expects an int or byte as it's first argument, but got a value of type {}",
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
                "from_codepoint".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `to_codepoint` icelang standard library function
pub fn isl_to_codepoint<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            match &arguments[0] {
                Value::String(character) => {
                    Ok(if character.chars().count() != 1 {
                        Value::Null
                    } else {
                        Value::Int(BigInt::from(
                            character.chars().next().unwrap() as u32
                        ))
                    })
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`to_codepoint(...)` expects a string as it's first argument, but got a value of type {}",
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
                "to_codepoint".to_string(),
                argument_count,
            ),
        )),
    }
}
