use num_traits::{Signed, ToPrimitive};

use crate::{
    ast::{AssignmentKind, AstNode, AstNodeAssignment},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::{
    core::interpret_expression,
    operations::addition,
    runtime_result::{NonLinearControlFlow, RuntimeResult},
};

/// Assigns a value to an lvalue node
pub fn assign_to_lvalue<'source>(
    lvalue: &AstNode<'source>,
    value: Value,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, ()> {
    // TODO Can we avoid the to_string() here? If the lookup fails, we don't
    // need the mutable borrow anymore... I think?
    let scope_display_name = state.scope_display_name().to_string();
    match lvalue {
        AstNode::VariableAccess(node) => match state.lookup_variable_mut(node.ident()) {
            Some(lvalue) => *lvalue = value,
            None => {
                return Err(NonLinearControlFlow::RuntimeError(
                    RuntimeError::new_undefined_reference_error(
                        node.pos().clone(),
                        scope_display_name,
                        node.ident().to_string(),
                    ),
                ))
            }
        },
        AstNode::DotMemberAccess(node) => {
            let root = interpret_expression(node.root(), state)?;
            match root {
                Value::Dict(dict) => {
                    let key = Value::String(node.member().into());
                    dict.borrow_mut().insert(key, value);
                }
                root => {
                    return Err(NonLinearControlFlow::RuntimeError(
                        RuntimeError::new_invalid_member_access_error(
                            node.pos().clone(),
                            scope_display_name,
                            format!("cannot index value of type {}", root.icelang_type()),
                        ),
                    ))
                }
            }
        }
        AstNode::ComputedMemberAccess(node) => {
            let root = interpret_expression(node.root(), state)?;
            let member = interpret_expression(node.member_node(), state)?;
            match root {
                Value::List(list) => {
                    let mut list = list.borrow_mut();
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

                    list[index] = value;
                }
                Value::Dict(dict) => {
                    dict.borrow_mut().insert(member, value);
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
        _ => todo!(),
    };

    Ok(())
}

/// Interprets an AstNodeAssignment
///
/// # Panics
/// - if the AstNodeAssignment isn't a valid assignment
pub fn interpret_assignment<'source>(
    assignment: &AstNodeAssignment<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match assignment.assignment_kind() {
        AssignmentKind::Normal => {
            let value = interpret_expression(assignment.rhs(), state)?;

            assign_to_lvalue(assignment.lhs(), value.clone(), state)?;

            Ok(value)
        }
        AssignmentKind::Plus => {
            let lhs = interpret_expression(assignment.lhs(), state)?;
            let rhs = interpret_expression(assignment.rhs(), state)?;

            let Ok(value) = addition(lhs, rhs) else {
                todo!();
            };

            assign_to_lvalue(assignment.lhs(), value.clone(), state)?;

            Ok(value)
        }
        AssignmentKind::Minus => todo!(),
        AssignmentKind::Times => todo!(),
        AssignmentKind::Div => todo!(),
        AssignmentKind::Mod => todo!(),
        AssignmentKind::Exp => todo!(),
        AssignmentKind::Shl => todo!(),
        AssignmentKind::Shr => todo!(),
        AssignmentKind::BitAnd => todo!(),
        AssignmentKind::BitXor => todo!(),
        AssignmentKind::BitOr => todo!(),
        AssignmentKind::LogAnd => todo!(),
        AssignmentKind::LogOr => todo!(),
    }
}
