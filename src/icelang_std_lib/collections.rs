use std::{cell::RefCell, rc::Rc};

use num_bigint::BigInt;

use crate::{
    error::runtime_error::RuntimeError,
    interpreter::{NonLinearControlFlow, RuntimeResult},
    runtime_state::RuntimeState,
    source_range::SourceRange,
    value::Value,
};

/// The `len` icelang standard library function
pub fn isl_len<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            match &arguments[0] {
                Value::String(string) => {
                    Ok(Value::Int(BigInt::from(string.len())))
                },
                Value::List(list) => {
                    Ok(Value::Int(BigInt::from(list.borrow().len())))
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`len(...)` expects a list or string as it's first argument, but got a value of type {}",
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
                "len".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `push` icelang standard library function
pub fn isl_push<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        2 => {
            match &arguments[0] {
                Value::List(list) => {
                    list.borrow_mut().push(arguments[1].reference_copy());
                    Ok(Value::Null)
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`push(...)` expects a list as it's first argument, but got a value of type {}",
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
                "push".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `pop` icelang standard library function
pub fn isl_pop<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            match &arguments[0] {
                Value::List(list) => {
                    Ok(list.borrow_mut().pop().unwrap_or(Value::Null))
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`pop(...)` expects a list as it's first argument, but got a value of type {}",
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
                "pop".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `push_start` icelang standard library function
pub fn isl_push_start<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        2 => {
            match &arguments[0] {
                Value::List(list) => {
                    // TODO switch to a VecDeque and use `.push_start(...)`
                    list.borrow_mut().insert(0, arguments[1].reference_copy());
                    Ok(Value::Null)
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`push_start(...)` expects a list as it's first argument, but got a value of type {}",
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
                "push_start".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `pop_start` icelang standard library function
pub fn isl_pop_start<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            match &arguments[0] {
                Value::List(list) => {
                    // TODO switch to a VecDeque and use `.pop_front(...)`
                    let mut list = list.borrow_mut();

                    Ok(if list.is_empty() {
                        Value::Null
                    } else {
                        list.remove(0)
                    })
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`pop_start(...)` expects a list as it's first argument, but got a value of type {}",
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
                "pop_start".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `contains_key` icelang standard library function
pub fn isl_contains_key<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        2 => {
            match &arguments[0] {
                Value::Dict(dict) => {
                    let dict = dict.borrow();

                    Ok(Value::Bool(dict.contains_key(&arguments[1])))
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`contains_key(...)` expects a dict as it's first argument, but got a value of type {}",
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
                "contains_key".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `remove_entry` icelang standard library function
pub fn isl_remove_entry<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        2 => {
            match &arguments[0] {
                Value::Dict(dict) => {
                    let mut dict = dict.borrow_mut();

                    Ok(dict.remove_entry(&arguments[1]).map(|(_, value)| value).unwrap_or(Value::Null))
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`remove_entry(...)` expects a dict as it's first argument, but got a value of type {}",
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
                "remove_entry".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `keys` icelang standard library function
pub fn isl_keys<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            match &arguments[0] {
                Value::Dict(dict) => {
                    let dict = dict.borrow();

                    Ok(Value::List(Rc::new(RefCell::new(dict.keys().map(Value::reference_copy).collect()))))
                },
                _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`keys(...)` expects a dict as it's first argument, but got a value of type {}",
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
                "keys".to_string(),
                argument_count,
            ),
        )),
    }
}
