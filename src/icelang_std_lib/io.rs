use crate::{
    error::runtime_error::RuntimeError, runtime_state::RuntimeState, source_range::SourceRange,
    value::Value,
};

/// The `print` icelang standard library function
pub fn isl_print<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    _state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    match arguments.len() {
        1 => {
            print!("{}", arguments[0].icelang_display());

            Ok(Value::Null)
        }
        argument_count => Err(RuntimeError::new_invalid_overload_error(
            pos.clone(),
            "print".to_string(),
            argument_count,
        )),
    }
}

/// The `println` icelang standard library function
pub fn isl_println<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    _state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    match arguments.len() {
        0 => {
            println!();

            Ok(Value::Null)
        }
        1 => {
            println!("{}", arguments[0].icelang_display());

            Ok(Value::Null)
        }
        argument_count => Err(RuntimeError::new_invalid_overload_error(
            pos.clone(),
            "println".to_string(),
            argument_count,
        )),
    }
}

/// The `eprint` icelang standard library function
pub fn isl_eprint<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    _state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    match arguments.len() {
        1 => {
            eprint!("{}", arguments[0].icelang_display());

            Ok(Value::Null)
        }
        argument_count => Err(RuntimeError::new_invalid_overload_error(
            pos.clone(),
            "eprint".to_string(),
            argument_count,
        )),
    }
}

/// The `eprintln` icelang standard library function
pub fn isl_eprintln<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    _state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    match arguments.len() {
        0 => {
            eprintln!();

            Ok(Value::Null)
        }
        1 => {
            eprintln!("{}", arguments[0].icelang_display());

            Ok(Value::Null)
        }
        argument_count => Err(RuntimeError::new_invalid_overload_error(
            pos.clone(),
            "eprintln".to_string(),
            argument_count,
        )),
    }
}
