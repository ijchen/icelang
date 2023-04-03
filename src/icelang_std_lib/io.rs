use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{
    error::runtime_error::RuntimeError,
    interpreter::{NonLinearControlFlow, RuntimeResult},
    runtime_state::RuntimeState,
    source_range::SourceRange,
    value::Value,
};

/// The `args` icelang standard library function
pub fn isl_args<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        0 => Ok(Value::List(Rc::new(RefCell::new(
            std::env::args()
                .skip(1)
                .map(|arg| Value::String(arg.into()))
                .collect::<Vec<_>>(),
        )))),
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "args".to_string(),
                argument_count,
            ),
        )),
    }
}

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
                Err(_) => todo!(),
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

/// The `read_file` icelang standard library function
pub fn isl_read_file<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            let Value::String(path) = arguments[0].clone() else {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`read_file(...)` expects a string as it's first argument, but got a value of type {}",
                            arguments[0].icelang_type()
                        )
                    )
                ));
            };

            match std::fs::read_to_string(path.as_ref()) {
                Ok(contents) => Ok(Value::String(contents.into())),
                Err(_err) => {
                    // TODO someday, let's give the programmer more info on what
                    // went wrong
                    // let why = match _err.kind() {
                    //     std::io::ErrorKind::NotFound => format!("file \"{path}\" not found"),
                    //     std::io::ErrorKind::PermissionDenied => {
                    //         format!("permission denied attempting to read file \"{path}\"")
                    //     }
                    //     std::io::ErrorKind::InvalidData => {
                    //         format!("file \"{path}\" did not contain valid UTF-8 (did you mean to use `read_file_bin(...)`?)")
                    //     }
                    //     std::io::ErrorKind::Interrupted => {
                    //         format!("interrupted while reading file \"{path}\"")
                    //     }
                    //     std::io::ErrorKind::Unsupported => {
                    //         "file reading is not supported on this platform".to_string()
                    //     }
                    //     _ => format!("failed to read file \"{path}\""),
                    // };

                    // return Err(NonLinearControlFlow::RuntimeError(
                    //     RuntimeError::new_resource_unavailable_error(
                    //         pos.clone(),
                    //         state.scope_display_name().to_string(),
                    //         why,
                    //     ),
                    // ));
                    Ok(Value::Null)
                }
            }
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "read_file".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `read_file_bin` icelang standard library function
pub fn isl_read_file_bin<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        1 => {
            let Value::String(path) = arguments[0].clone() else {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`read_file_bin(...)` expects a string as it's first argument, but got a value of type {}",
                            arguments[0].icelang_type()
                        )
                    )
                ));
            };

            match std::fs::read(path.as_ref()) {
                Ok(contents) => Ok(Value::List(Rc::new(RefCell::new(
                    contents.into_iter().map(|byte| Value::Byte(byte)).collect(),
                )))),
                Err(_err) => {
                    // TODO someday, let's give the programmer more info on what
                    // went wrong
                    // let why = match _err.kind() {
                    //     std::io::ErrorKind::NotFound => format!("file \"{path}\" not found"),
                    //     std::io::ErrorKind::PermissionDenied => {
                    //         format!("permission denied attempting to read file \"{path}\"")
                    //     }
                    //     std::io::ErrorKind::Interrupted => {
                    //         format!("interrupted while reading file \"{path}\"")
                    //     }
                    //     std::io::ErrorKind::Unsupported => {
                    //         "file reading is not supported on this platform".to_string()
                    //     }
                    //     _ => format!("failed to read file \"{path}\""),
                    // };

                    // return Err(NonLinearControlFlow::RuntimeError(
                    //     RuntimeError::new_resource_unavailable_error(
                    //         pos.clone(),
                    //         state.scope_display_name().to_string(),
                    //         why,
                    //     ),
                    // ));
                    Ok(Value::Null)
                }
            }
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "read_file_bin".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `write_file` icelang standard library function
pub fn isl_write_file<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        2 => {
            let Value::String(path) = arguments[0].clone() else {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`write_file(...)` expects a string as it's first argument, but got a value of type {}",
                            arguments[0].icelang_type()
                        )
                    )
                ));
            };
            let Value::String(contents) = arguments[1].clone() else {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`write_file(...)` expects a string as it's second argument, but got a value of type {}",
                            arguments[0].icelang_type()
                        )
                    )
                ));
            };

            match std::fs::write(path.as_ref(), contents.as_bytes()) {
                Ok(()) => Ok(Value::Bool(true)),
                Err(_) => {
                    // TODO someday, let's give the programmer more info on what
                    // went wrong
                    Ok(Value::Bool(false))
                }
            }
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "write_file".to_string(),
                argument_count,
            ),
        )),
    }
}

/// The `write_file_bin` icelang standard library function
pub fn isl_write_file_bin<'source>(
    arguments: Vec<Value>,
    pos: &SourceRange<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match arguments.len() {
        2 => {
            let Value::String(path) = arguments[0].clone() else {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`write_file_bin(...)` expects a string as it's first argument, but got a value of type {}",
                            arguments[0].icelang_type()
                        )
                    )
                ));
            };
            let Value::List(contents) = arguments[1].clone() else {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`write_file_bin(...)` expects a list as it's second argument, but got a value of type {}",
                            arguments[0].icelang_type()
                        )
                    )
                ));
            };

            let bytes: Vec<u8> = contents
                .borrow()
                .iter()
                .map(|value| match value {
                    Value::Byte(byte) => Ok(*byte),
                    _ => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_assertion_error(
                        pos.clone(),
                        state.scope_display_name().to_string(),
                        format!(
                            "`write_file_bin(...)` expects a list containing only bytes as it's second argument, but the list contained a value of type {}",
                            value.icelang_type()
                        )
                    )))
                })
                .collect::<Result<_, _>>()?;

            match std::fs::write(path.as_ref(), bytes) {
                Ok(()) => Ok(Value::Bool(true)),
                Err(_) => {
                    // TODO someday, let's give the programmer more info on what
                    // went wrong
                    Ok(Value::Bool(false))
                }
            }
        }
        argument_count => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_overload_error(
                pos.clone(),
                state.scope_display_name().to_string(),
                "write_file_bin".to_string(),
                argument_count,
            ),
        )),
    }
}
