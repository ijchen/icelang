use crate::{
    ast::{AstNodeBinaryOperation, BinaryOperationKind},
    error::runtime_error::RuntimeError,
    interpreter::core::interpret_expression,
    runtime_state::RuntimeState,
    value::Value,
};

fn interpret_addition<'source>(
    node: &AstNodeBinaryOperation<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    assert!(node.operation() == BinaryOperationKind::Addition);

    let lhs = interpret_expression(node.lhs(), state)?;
    let rhs = interpret_expression(node.rhs(), state)?;

    match (lhs, rhs) {
        (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Int(lhs + rhs)),
        (Value::Byte(lhs), Value::Byte(rhs)) => Ok(Value::Byte(lhs.wrapping_add(rhs))),
        (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Float(lhs + rhs)),
        (Value::String(lhs), Value::String(rhs)) => {
            let _ = (lhs, rhs);
            todo!()
        }
        (lhs, rhs) => Err(RuntimeError::Type {
            pos: node.pos().clone(),
            why: format!(
                "invalid types for binary operation: {} {} {}",
                lhs.icelang_type(),
                node.operation(),
                rhs.icelang_type(),
            ),
        }),
    }
}

/// Interprets an AstNodeBinaryOperation
///
/// # Panics
/// - If the AstNodeBinaryOperation isn't a valid binary operation node
pub fn interpret_binary_operation<'source>(
    node: &AstNodeBinaryOperation<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    match node.operation() {
        BinaryOperationKind::LogicalOr => todo!(),
        BinaryOperationKind::LogicalAnd => todo!(),
        BinaryOperationKind::BitwiseOr => todo!(),
        BinaryOperationKind::BitwiseXor => todo!(),
        BinaryOperationKind::BitwiseAnd => todo!(),
        BinaryOperationKind::ShiftLeft => todo!(),
        BinaryOperationKind::ShiftRight => todo!(),
        BinaryOperationKind::Addition => interpret_addition(node, state),
        BinaryOperationKind::Subtraction => todo!(),
        BinaryOperationKind::Multiplication => todo!(),
        BinaryOperationKind::Division => todo!(),
        BinaryOperationKind::Modulo => todo!(),
        BinaryOperationKind::Exponentiation => todo!(),
    }
}
