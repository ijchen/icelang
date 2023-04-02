use std::io::Write;

use crate::{
    error::runtime_error::RuntimeError,
    interpreter::{NonLinearControlFlow, RuntimeResult},
    runtime_state::RuntimeState,
    source_range::SourceRange,
    value::Value,
};

/// The `print` icelang standard library function
pub fn isl_print<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            print!("{}", arguments[0].icelang_display());
            if std::io::stdout().flush().is_err() {
                todo!();
            }

            Ok(Value::Null)
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "print".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `println` icelang standard library function
pub fn isl_println<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => {
            println!();

            Ok(Value::Null)
        }
        1 => {
            println!("{}", arguments[0].icelang_display());

            Ok(Value::Null)
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "println".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `eprint` icelang standard library function
pub fn isl_eprint<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            eprint!("{}", arguments[0].icelang_display());
            if std::io::stdout().flush().is_err() {
                todo!();
            }

            Ok(Value::Null)
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "eprint".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `eprintln` icelang standard library function
pub fn isl_eprintln<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => {
            eprintln!();

            Ok(Value::Null)
        }
        1 => {
            eprintln!("{}", arguments[0].icelang_display());

            Ok(Value::Null)
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "eprintln".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `input` icelang standard library function
pub fn isl_input<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => match std::io::stdin().lines().next() {
            Some(input_result) => match input_result {
                Ok(input) => Ok(Value::String(input.into())),
                Err(_) => Ok(Value::Null),
            },
            None => Ok(Value::Null),
        },
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "input".to_string(),
                argument_count,
            ),
        )),
    }
}
