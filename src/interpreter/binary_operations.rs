use num_traits::Zero;

use crate::{
    ast::{AstNodeBinaryOperation, BinaryOperationKind},
    error::runtime_error::RuntimeError,
    interpreter::core::interpret_expression,
    runtime_state::RuntimeState,
    value::Value,
};

macro_rules! impl_simple_bin_op {
    ($func_name:ident, $node:ident, $lhs:ident, $rhs:ident, $op_kind:ident, {$($lhs_type:ident, $rhs_type:ident => $result:expr),+$(,)?}) => {
        fn $func_name<'source>(
            $node: &AstNodeBinaryOperation<'source>,
            state: &mut RuntimeState,
        ) -> Result<Value, RuntimeError<'source>> {
            assert!($node.operation() == BinaryOperationKind::$op_kind);

            let lhs = interpret_expression($node.lhs(), state)?;
            let rhs = interpret_expression($node.rhs(), state)?;

            match (lhs, rhs) {
                $(
                    (Value::$lhs_type($lhs), Value::$rhs_type($rhs)) => $result,
                )+
                ($lhs, $rhs) => Err(RuntimeError::Type {
                    pos: $node.pos().clone(),
                    why: format!(
                        "invalid types for binary operation: {} {} {}",
                        $lhs.icelang_type(),
                        $node.operation(),
                        $rhs.icelang_type(),
                    ),
                }),
            }
        }
    };
}

impl_simple_bin_op!(interpret_addition, node, lhs, rhs, Addition, {
    Int, Int => Ok(Value::Int(lhs + rhs)),
    Byte, Byte => Ok(Value::Byte(lhs.wrapping_add(rhs))),
    Float, Float => Ok(Value::Float(lhs + rhs)),
    String, String => {
        let _ = (lhs, rhs);
        todo!()
    },
});

impl_simple_bin_op!(interpret_subtraction, node, lhs, rhs, Subtraction, {
    Int, Int => Ok(Value::Int(lhs - rhs)),
    Byte, Byte => Ok(Value::Byte(lhs.wrapping_sub(rhs))),
    Float, Float => Ok(Value::Float(lhs - rhs)),
});

impl_simple_bin_op!(interpret_multiplication, node, lhs, rhs, Multiplication, {
    Int, Int => Ok(Value::Int(lhs * rhs)),
    Byte, Byte => Ok(Value::Byte(lhs.wrapping_mul(rhs))),
    Float, Float => Ok(Value::Float(lhs * rhs)),
    String, Int => {
        let _ = (lhs, rhs);
        todo!()
    },
    String, Byte => {
        let _ = (lhs, rhs);
        todo!()
    },
    Int, String => {
        let _ = (lhs, rhs);
        todo!()
    },
    Byte, String => {
        let _ = (lhs, rhs);
        todo!()
    },
});

impl_simple_bin_op!(interpret_division, node, lhs, rhs, Division, {
    Int, Int => if rhs.is_zero() {
        Err(RuntimeError::Mathematical { pos: node.pos().clone(), why: "division by zero".to_string() })
    } else {
        Ok(Value::Int(lhs / rhs))
    },
    Byte, Byte => if rhs == 0 {
        Err(RuntimeError::Mathematical { pos: node.pos().clone(), why: "division by zero".to_string() })
    } else {
        Ok(Value::Byte(lhs / rhs))
    },
    Float, Float => Ok(Value::Float(lhs / rhs)),
});

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
        BinaryOperationKind::Subtraction => interpret_subtraction(node, state),
        BinaryOperationKind::Multiplication => interpret_multiplication(node, state),
        BinaryOperationKind::Division => interpret_division(node, state),
        BinaryOperationKind::Modulo => todo!(),
        BinaryOperationKind::Exponentiation => todo!(),
    }
}
