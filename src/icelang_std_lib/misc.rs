use std::{cell::RefCell, rc::Rc};

use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};

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

/// The `range` icelang standard library function
pub fn isl_range<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    let (start, end, step) = match arguments.len() {
        1 => match &arguments[0] {
            Value::Int(end) => (BigInt::from(0), end.to_owned(), BigInt::from(1)),
            arg => {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`range(...)` expects an int as it's first argument, but got a value of type {}",
                            arg.icelang_type()
                        )
                    ),
                ))
            }
        },
        2 => match (&arguments[0], &arguments[1]) {
            (Value::Int(start), Value::Int(end)) => (start.to_owned(), end.to_owned(), BigInt::from(1)),
            (Value::Int(_), arg) => return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_assertion_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "`range(...)` expects an int as it's second argument, but got a value of type {}",
                        arg.icelang_type()
                    )
                ),
            )),
            (arg, _) => return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_assertion_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "`range(...)` expects an int as it's first argument, but got a value of type {}",
                        arg.icelang_type()
                    )
                ),
            )),
        },
        3 => match (&arguments[0], &arguments[1], &arguments[2]) {
            (Value::Int(start), Value::Int(end), Value::Int(step)) => (start.to_owned(), end.to_owned(), step.to_owned()),
            (Value::Int(_), Value::Int(_), arg) => return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_assertion_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "`range(...)` expects an int as it's third argument, but got a value of type {}",
                        arg.icelang_type()
                    )
                ),
            )),
            (Value::Int(_), arg, _) => return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_assertion_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "`range(...)` expects an int as it's second argument, but got a value of type {}",
                        arg.icelang_type()
                    )
                ),
            )),
            (arg, _, _) => return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_assertion_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "`range(...)` expects an int as it's first argument, but got a value of type {}",
                        arg.icelang_type()
                    )
                ),
            )),
        },
        argument_count => {
            return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_invalid_overload_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    "range".to_string(),
                    argument_count,
                ),
            ))
        }
    };

    // If start == end, there is no iteration to do - this list should be empty
    // note that this applies even if step is 0 - this is the only situation in
    // which 0 is allowed for the step
    if start == end {
        return Ok(Value::List(Rc::new(RefCell::new(Vec::new()))));
    }

    // Zero step is an error (we already handled start == end)
    if step.is_zero() {
        return Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_assertion_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "0 is not a valid step value".to_string(),
            ),
        ));
    }

    // If start < end, step must be positive
    if start < end && step.is_negative() {
        return Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_assertion_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "step value must be positive if start < end".to_string(),
            ),
        ));
    }

    // If start > end, step must be negative
    if start > end && step.is_positive() {
        return Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_assertion_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "step value must be negative if start > end".to_string(),
            ),
        ));
    }

    let capacity: BigInt = (&end - &start) / &step;
    let Some(capacity): Option<usize> = capacity.to_usize() else {
        return Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_resource_unavailable_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "range results in list with too many elements".to_string()
            )
        ));
    };

    let mut values = Vec::with_capacity(capacity);
    let mut curr = start;
    while (&end - &curr).sign() == step.sign() {
        values.push(Value::Int(curr.clone()));
        curr += &step;
    }

    Ok(Value::List(Rc::new(RefCell::new(values))))
}
