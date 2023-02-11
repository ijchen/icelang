use crate::{
    ast::{AssignmentKind, AstNode, AstNodeAssignment},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::core::interpret_expression;

/// Interprets an AstNodeAssignment
///
/// # Panics
/// - if the AstNodeAssignment isn't a valid assignment
pub fn interpret_assignment<'source>(
    assignment: &AstNodeAssignment<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    match assignment.assignment_kind() {
        AssignmentKind::Normal => {
            let value = interpret_expression(assignment.rhs(), state)?;

            match assignment.lhs() {
                AstNode::VariableAccess(node) => {
                    if state.lookup_variable(node.ident()).is_none() {
                        return Err(RuntimeError::new_undefined_reference_error(
                            node.pos().clone(),
                            state.scope_display_name().to_string(),
                            node.ident().to_string(),
                        ));
                    }

                    state.reassign_variable(node.ident(), value.clone());
                    Ok(value)
                }
                AstNode::DotMemberAccess(_) => todo!(),
                AstNode::ComputedMemberAccess(_) => todo!(),
                _ => todo!(),
            }
        }
        AssignmentKind::Plus => todo!(),
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
