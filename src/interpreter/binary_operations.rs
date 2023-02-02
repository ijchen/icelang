use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};

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
            state: &mut RuntimeState<'source>,
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

impl_simple_bin_op!(interpret_logical_or, node, lhs, rhs, LogicalOr, {
    Bool, Bool => Ok(Value::Bool(lhs || rhs)),
});

impl_simple_bin_op!(interpret_logical_and, node, lhs, rhs, LogicalAnd, {
    Bool, Bool => Ok(Value::Bool(lhs && rhs)),
});

impl_simple_bin_op!(interpret_bitwise_xor, node, lhs, rhs, BitwiseXor, {
    Int, Int => Ok(Value::Int(lhs ^ rhs)),
    Byte, Byte => Ok(Value::Byte(lhs ^ rhs)),
});

impl_simple_bin_op!(interpret_bitwise_or, node, lhs, rhs, BitwiseXor, {
    Int, Int => Ok(Value::Int(lhs | rhs)),
    Byte, Byte => Ok(Value::Byte(lhs | rhs)),
});

impl_simple_bin_op!(interpret_bitwise_and, node, lhs, rhs, BitwiseAnd, {
    Int, Int => Ok(Value::Int(lhs & rhs)),
    Byte, Byte => Ok(Value::Byte(lhs & rhs)),
});

impl_simple_bin_op!(interpret_shift_left, node, lhs, rhs, ShiftLeft, {
    Int, Int => {
        let mut rhs = rhs;
        let mut lhs = lhs;
        while rhs > BigInt::from(u32::MAX) {
            rhs -= u32::MAX;
            lhs <<= u32::MAX;
        }
        lhs <<= rhs.to_u32().unwrap();
        Ok(Value::Int(lhs))
    },
    Byte, Byte => {
        if rhs >= u8::BITS as u8 {
            Ok(Value::Byte(0))
        }
        else {
            Ok(Value::Byte(lhs << rhs))
        }
    },
});

impl_simple_bin_op!(interpret_shift_right, node, lhs, rhs, ShiftRight, {
    Int, Int => {
        let mut rhs = rhs;
        let mut lhs = lhs;
        while rhs > BigInt::from(u32::MAX) {
            rhs -= u32::MAX;
            lhs >>= u32::MAX;
        }
        lhs >>= rhs.to_u32().unwrap();
        Ok(Value::Int(lhs))
    },
    Byte, Byte => {
        if rhs >= u8::BITS as u8 {
            Ok(Value::Byte(0))
        }
        else {
            Ok(Value::Byte(lhs >> rhs))
        }
    },
});

impl_simple_bin_op!(interpret_addition, node, lhs, rhs, Addition, {
    Int, Int => Ok(Value::Int(lhs + rhs)),
    Byte, Byte => Ok(Value::Byte(lhs.wrapping_add(rhs))),
    Float, Float => Ok(Value::Float(lhs + rhs)),
    String, String => {
        Ok(Value::String((lhs.to_string() + &*rhs).into()))
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
        // TODO this can panic if the output string is too large or rhs doesn't
        // fit in a usize
        Ok(Value::String((lhs.repeat(rhs.to_usize().unwrap())).into()))
    },
    String, Byte => {
        // TODO this can panic if the output string is too large
        Ok(Value::String((lhs.repeat(rhs as usize)).into()))
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

impl_simple_bin_op!(interpret_modulo, node, lhs, rhs, Modulo, {
    Int, Int => if rhs.is_zero() {
        Err(RuntimeError::Mathematical { pos: node.pos().clone(), why: "modulo by zero".to_string() })
    } else {
        Ok(Value::Int(((lhs % &rhs) + &rhs) % &rhs))
    },
    Byte, Byte => if rhs == 0 {
        Err(RuntimeError::Mathematical { pos: node.pos().clone(), why: "modulo by zero".to_string() })
    } else {
        Ok(Value::Byte(lhs.wrapping_rem_euclid(rhs)))
    },
    Float, Float => Ok(Value::Float(((lhs % rhs) + rhs) % rhs)),
});

impl_simple_bin_op!(interpret_exponentiation, node, lhs, rhs, Exponentiation, {
    Int, Int => {
        let mut rhs = rhs;
        let mut result = BigInt::from(1u8);
        while rhs > BigInt::from(u32::MAX) {
            rhs -= u32::MAX;
            result *= lhs.pow(u32::MAX);
        }
        result *= lhs.pow(rhs.to_u32().unwrap());
        Ok(Value::Int(result))
    },
    Byte, Byte => Ok(Value::Byte(lhs.wrapping_pow(rhs as u32))),
    Float, Float => Ok(Value::Float(lhs.powf(rhs))),
});

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
