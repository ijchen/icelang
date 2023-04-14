use num_traits::{Signed, ToPrimitive};

use crate::{
    ast::{AstNodeComputedMemberAccess, AstNodeDotMemberAccess},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::{
    core::interpret_expression,
    runtime_result::{NonLinearControlFlow, RuntimeResult},
};

/// Interprets an AstNodeDotMemberAccess
///
/// # Panics
/// - If the AstNodeDotMemberAccess is invalid in any way
pub fn interpret_dot_member_access<'source>(
    node: &AstNodeDotMemberAccess<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    // TODO Can we avoid the to_string() here? If the lookup fails, we don't
    // need the mutable borrow anymore... I think?
    let scope_display_name = state.scope_display_name().to_string();
    let root = interpret_expression(node.root(), state)?;
    match root {
        Value::Dict(dict) => {
            let key = Value::String(node.member().into());
            match dict.borrow().get(&key) {
                Some(value) => Ok(value.reference_copy()),
                None => Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_invalid_member_access_error(
                        node.pos().clone(),
                        scope_display_name,
                        format!("key \"{}\" does not exist", node.member()),
                    ),
                )),
            }
        }
        root => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_invalid_member_access_error(
                node.pos().clone(),
                scope_display_name,
                format!("cannot index value of type {}", root.icelang_type()),
            ),
        )),
    }
}

/// Interprets an AstNodeComputedMemberAccess
///
/// # Panics
/// - If the AstNodeComputedMemberAccess is invalid in any way
pub fn interpret_computed_member_access<'source>(
    node: &AstNodeComputedMemberAccess<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    // TODO Can we avoid the to_string() here? If the lookup fails, we don't
    // need the mutable borrow anymore... I think?
    let scope_display_name = state.scope_display_name().to_string();
    let root = interpret_expression(node.root(), state)?;
    let member = interpret_expression(node.member_node(), state)?;
    match root {
        Value::List(list) => {
            let list = list.borrow();
            let index: usize = match member {
                Value::Int(index) => {
                    if index.is_negative() {
                        return Err(NonLinearControlFlow::RuntimeError(
                            RuntimeError::new_invalid_member_access_error(
                                node.pos().clone(),
                                scope_display_name,
                                format!(
                                    "index out of bounds (index {}, length {})",
                                    index,
                                    list.len(),
                                ),
                            ),
                        ));
                    }
                    match index.to_usize() {
                        Some(index) => index,
                        None => todo!(),
                    }
                }
                Value::Byte(byte) => byte as usize,
                member => {
                    return Err(NonLinearControlFlow::RuntimeError(
                        RuntimeError::new_invalid_member_access_error(
                            node.pos().clone(),
                            scope_display_name,
                            format!(
                                "cannot index a list with a value of type {}",
                                member.icelang_type()
                            ),
                        ),
                    ));
                }
            };

            // Ensure the index is in-bounds
            if index >= list.len() {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_invalid_member_access_error(
                        node.pos().clone(),
                        scope_display_name,
                        format!(
                            "index out of bounds (index {}, length {})",
                            index,
                            list.len(),
                        ),
                    ),
                ));
            }

            Ok(list[index].reference_copy())
        }
        Value::Dict(dict) => match dict.borrow().get(&member) {
            Some(value) => Ok(value.reference_copy()),
            None => Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_invalid_member_access_error(
                    node.pos().clone(),
                    scope_display_name,
                    format!("key {} does not exist", member.icelang_debug()),
                ),
            )),
        },
        Value::String(string) => {
            let index: usize = match member {
                Value::Int(index) => {
                    if index.is_negative() {
                        return Err(NonLinearControlFlow::RuntimeError(
                            RuntimeError::new_invalid_member_access_error(
                                node.pos().clone(),
                                scope_display_name,
                                format!(
                                    "index out of bounds (index {}, length {})",
                                    index,
                                    string.len(),
                                ),
                            ),
                        ));
                    }
                    match index.to_usize() {
                        Some(index) => index,
                        None => todo!(),
                    }
                }
                Value::Byte(byte) => byte as usize,
                member => {
                    return Err(NonLinearControlFlow::RuntimeError(
                        RuntimeError::new_invalid_member_access_error(
                            node.pos().clone(),
                            scope_display_name,
                            format!(
                                "cannot index a list with a value of type {}",
                                member.icelang_type()
                            ),
                        ),
                    ));
                }
            };

            // Ensure the index is in-bounds
            if index >= string.len() {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_invalid_member_access_error(
                        node.pos().clone(),
                        scope_display_name,
                        format!(
                            "index out of bounds (index {}, length {})",
                            index,
                            string.len(),
                        ),
                    ),
                ));
            }

            Ok(Value::String(
                string.chars().nth(index).unwrap().to_string().into(),
            ))
        }
        root => {
            return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_invalid_member_access_error(
                    node.pos().clone(),
                    scope_display_name,
                    format!("cannot index value of type {}", root.icelang_type()),
                ),
            ));
        }
    }
}
