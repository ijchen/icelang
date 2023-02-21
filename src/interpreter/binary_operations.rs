use crate::{
    ast::{AstNodeBinaryOperation, BinaryOperationKind},
    error::runtime_error::RuntimeError,
    interpreter::{
        core::interpret_expression,
        operations::{
            addition, bitwise_and, bitwise_or, bitwise_xor, division, exponentiation, modulo,
            multiplication, shift_left, shift_right, subtraction, OperationError,
        },
    },
    runtime_state::RuntimeState,
    value::Value,
};

macro_rules! impl_bin_op {
    (
        $func_name: ident,
        $operation_func_name: ident,
        $operation_kind: ident
    ) => {
        fn $func_name<'source>(
            node: &AstNodeBinaryOperation<'source>,
            state: &mut RuntimeState<'source>,
        ) -> Result<Value, RuntimeError<'source>> {
            assert!(node.operation() == BinaryOperationKind::$operation_kind);

            let lhs = interpret_expression(node.lhs(), state)?;
            let rhs = interpret_expression(node.rhs(), state)?;

            $operation_func_name(lhs, rhs).map_err(|op_err| match op_err {
                OperationError::Type { why } => RuntimeError::new_type_error(
                    node.pos().clone(),
                    state.scope_display_name().to_string(),
                    why,
                ),
                OperationError::Mathematical { why } => RuntimeError::new_mathematical_error(
                    node.pos().clone(),
                    state.scope_display_name().to_string(),
                    why,
                ),
            })
        }
    };
}

fn interpret_logical_or<'source>(
    node: &AstNodeBinaryOperation<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    assert!(node.operation() == BinaryOperationKind::LogicalOr);

    let lhs = interpret_expression(node.lhs(), state)?;
    let Value::Bool(lhs_value) = lhs else {
        return Err(RuntimeError::new_type_error(
            node.pos().clone(),
            state.scope_display_name().to_string(),
            format!(
                "invalid types for binary operation: {} {} ...",
                lhs.icelang_type(),
                BinaryOperationKind::LogicalOr
            ),
        ));
    };

    if lhs_value {
        return Ok(Value::Bool(true));
    }

    let rhs = interpret_expression(node.rhs(), state)?;
    let Value::Bool(rhs_value) = rhs else {
        return Err(RuntimeError::new_type_error(
            node.pos().clone(),
            state.scope_display_name().to_string(),
            format!(
                "invalid types for binary operation: {} {} {}",
                lhs.icelang_type(),
                BinaryOperationKind::LogicalOr,
                rhs.icelang_type(),
            ),
        ));
    };

    Ok(Value::Bool(rhs_value))
}

fn interpret_logical_and<'source>(
    node: &AstNodeBinaryOperation<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    assert!(node.operation() == BinaryOperationKind::LogicalAnd);

    let lhs = interpret_expression(node.lhs(), state)?;
    let Value::Bool(lhs_value) = lhs else {
        return Err(RuntimeError::new_type_error(
            node.pos().clone(),
            state.scope_display_name().to_string(),
            format!(
                "invalid types for binary operation: {} {} ...",
                lhs.icelang_type(),
                BinaryOperationKind::LogicalAnd
            ),
        ));
    };

    if !lhs_value {
        return Ok(Value::Bool(false));
    }

    let rhs = interpret_expression(node.rhs(), state)?;
    let Value::Bool(rhs_value) = rhs else {
        return Err(RuntimeError::new_type_error(
            node.pos().clone(),
            state.scope_display_name().to_string(),
            format!(
                "invalid types for binary operation: {} {} {}",
                lhs.icelang_type(),
                BinaryOperationKind::LogicalOr,
                rhs.icelang_type(),
            ),
        ));
    };

    Ok(Value::Bool(rhs_value))
}

impl_bin_op!(interpret_bitwise_xor, bitwise_xor, BitwiseXor);
impl_bin_op!(interpret_bitwise_or, bitwise_or, BitwiseOr);
impl_bin_op!(interpret_bitwise_and, bitwise_and, BitwiseAnd);
impl_bin_op!(interpret_shift_left, shift_left, ShiftLeft);
impl_bin_op!(interpret_shift_right, shift_right, ShiftRight);
impl_bin_op!(interpret_addition, addition, Addition);
impl_bin_op!(interpret_subtraction, subtraction, Subtraction);
impl_bin_op!(interpret_multiplication, multiplication, Multiplication);
impl_bin_op!(interpret_division, division, Division);
impl_bin_op!(interpret_modulo, modulo, Modulo);
impl_bin_op!(interpret_exponentiation, exponentiation, Exponentiation);

/// Interprets an AstNodeBinaryOperation
///
/// # Panics
/// - If the AstNodeBinaryOperation isn't a valid binary operation node
pub fn interpret_binary_operation<'source>(
    node: &AstNodeBinaryOperation<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    match node.operation() {
        BinaryOperationKind::LogicalOr => interpret_logical_or(node, state),
        BinaryOperationKind::LogicalAnd => interpret_logical_and(node, state),
        BinaryOperationKind::BitwiseOr => interpret_bitwise_or(node, state),
        BinaryOperationKind::BitwiseXor => interpret_bitwise_xor(node, state),
        BinaryOperationKind::BitwiseAnd => interpret_bitwise_and(node, state),
        BinaryOperationKind::ShiftLeft => interpret_shift_left(node, state),
        BinaryOperationKind::ShiftRight => interpret_shift_right(node, state),
        BinaryOperationKind::Addition => interpret_addition(node, state),
        BinaryOperationKind::Subtraction => interpret_subtraction(node, state),
        BinaryOperationKind::Multiplication => interpret_multiplication(node, state),
        BinaryOperationKind::Division => interpret_division(node, state),
        BinaryOperationKind::Modulo => interpret_modulo(node, state),
        BinaryOperationKind::Exponentiation => interpret_exponentiation(node, state),
    }
}
