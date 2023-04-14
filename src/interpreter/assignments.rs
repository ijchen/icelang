use num_traits::{Signed, ToPrimitive};

use crate::{
    ast::{AssignmentKind, AstNode, AstNodeAssignment, BinaryOperationKind},
    error::runtime_error::RuntimeError,
    interpreter::operations,
    runtime_state::RuntimeState,
    value::Value,
};

use super::{
    core::interpret_expression,
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
    macro_rules! augmented_assignment {
        ($operation: expr) => {{
            let lhs = interpret_expression(assignment.lhs(), state)?;
            let rhs = interpret_expression(assignment.rhs(), state)?;

            let value = $operation(lhs, rhs).or_else(|err| {
                Err(NonLinearControlFlow::RuntimeError(err.into_runtime_error(
                    assignment.pos().clone(),
                    state.scope_display_name().to_string(),
                )))
            })?;

            assign_to_lvalue(assignment.lhs(), value.reference_copy(), state)?;

            Ok(value)
        }};
    }

    match assignment.assignment_kind() {
        AssignmentKind::Normal => {
            let value = interpret_expression(assignment.rhs(), state)?;

            assign_to_lvalue(assignment.lhs(), value.reference_copy(), state)?;

            Ok(value)
        }
        AssignmentKind::Plus => augmented_assignment!(operations::addition),
        AssignmentKind::Minus => augmented_assignment!(operations::subtraction),
        AssignmentKind::Times => augmented_assignment!(operations::multiplication),
        AssignmentKind::Div => augmented_assignment!(operations::division),
        AssignmentKind::Mod => augmented_assignment!(operations::modulo),
        AssignmentKind::Exp => augmented_assignment!(operations::exponentiation),
        AssignmentKind::Shl => augmented_assignment!(operations::shift_left),
        AssignmentKind::Shr => augmented_assignment!(operations::shift_right),
        AssignmentKind::BitAnd => augmented_assignment!(operations::bitwise_and),
        AssignmentKind::BitXor => augmented_assignment!(operations::bitwise_xor),
        AssignmentKind::BitOr => augmented_assignment!(operations::bitwise_or),
        AssignmentKind::LogAnd => {
            let lhs = interpret_expression(assignment.lhs(), state)?;

            // Short-circuit if the lhs isn't a bool
            let Value::Bool(lhs_value) = lhs else {
                return Err(NonLinearControlFlow::RuntimeError(RuntimeError::new_type_error(
                    assignment.pos().clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "invalid types for binary operation: {} {} ...",
                        lhs.icelang_type(),
                        BinaryOperationKind::LogicalAnd
                    ),
                )));
            };

            // Short-circuit if the lhs is false
            #[allow(clippy::bool_comparison)] // I like being explicit here
            if lhs_value == false {
                return Ok(Value::Bool(false));
            }

            let rhs = interpret_expression(assignment.rhs(), state)?;
            let Value::Bool(rhs_value) = rhs else {
                return Err(NonLinearControlFlow::RuntimeError(RuntimeError::new_type_error(
                    assignment.pos().clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "invalid types for binary operation: {} {} {}",
                        lhs.icelang_type(),
                        BinaryOperationKind::LogicalAnd,
                        rhs.icelang_type(),
                    ),
                )));
            };

            assign_to_lvalue(assignment.lhs(), Value::Bool(rhs_value), state)?;
            Ok(Value::Bool(rhs_value))
        }
        AssignmentKind::LogOr => {
            let lhs = interpret_expression(assignment.lhs(), state)?;

            // Short-circuit if the lhs isn't a bool
            let Value::Bool(lhs_value) = lhs else {
                return Err(NonLinearControlFlow::RuntimeError(RuntimeError::new_type_error(
                    assignment.pos().clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "invalid types for binary operation: {} {} ...",
                        lhs.icelang_type(),
                        BinaryOperationKind::LogicalOr
                    ),
                )));
            };

            // Short-circuit if the lhs is true
            #[allow(clippy::bool_comparison)] // I like being explicit here
            if lhs_value == true {
                return Ok(Value::Bool(true));
            }

            let rhs = interpret_expression(assignment.rhs(), state)?;
            let Value::Bool(rhs_value) = rhs else {
                return Err(NonLinearControlFlow::RuntimeError(RuntimeError::new_type_error(
                    assignment.pos().clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "invalid types for binary operation: {} {} {}",
                        lhs.icelang_type(),
                        BinaryOperationKind::LogicalOr,
                        rhs.icelang_type(),
                    ),
                )));
            };

            assign_to_lvalue(assignment.lhs(), Value::Bool(rhs_value), state)?;
            Ok(Value::Bool(rhs_value))
        }
    }
}
