use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};

use crate::value::Value;

pub enum OperationError {
    /// A value was an invalid type
    Type {
        /// An explanation of why the type is invalid
        why: String,
    },

    /// A mathematical error occured
    Mathematical {
        /// An explanation of what went wrong
        why: String,
    },
}

macro_rules! impl_simple_bin_op {
    (
        $func_name: ident,
        $lhs: ident,
        $rhs: ident,
        $operation: literal,
        {$(
            $lhs_type: ident,
            $rhs_type: ident => $result: expr
        ),+$(,)?}
    ) => {
        pub fn $func_name<'source>(
            $lhs: Value,
            $rhs: Value,
        ) -> Result<Value, OperationError> {
            match ($lhs, $rhs) {
                $(
                    (Value::$lhs_type($lhs), Value::$rhs_type($rhs)) => $result,
                )+
                ($lhs, $rhs) => Err(
                    OperationError::Type {
                        why: format!(
                            "invalid types for binary operation: {} {} {}",
                            $lhs.icelang_type(),
                            $operation,
                            $rhs.icelang_type(),
                        )
                    }
                ),
            }
        }
    };
}

impl_simple_bin_op!(bitwise_xor, lhs, rhs, "BitwiseXor", {
    Int, Int => Ok(Value::Int(lhs ^ rhs)),
    Byte, Byte => Ok(Value::Byte(lhs ^ rhs)),
});

impl_simple_bin_op!(bitwise_or, lhs, rhs, "BitwiseXor", {
    Int, Int => Ok(Value::Int(lhs | rhs)),
    Byte, Byte => Ok(Value::Byte(lhs | rhs)),
});

impl_simple_bin_op!(bitwise_and, lhs, rhs, "BitwiseAnd", {
    Int, Int => Ok(Value::Int(lhs & rhs)),
    Byte, Byte => Ok(Value::Byte(lhs & rhs)),
});

impl_simple_bin_op!(shift_left, lhs, rhs, "ShiftLeft", {
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

impl_simple_bin_op!(shift_right, lhs, rhs, "ShiftRight", {
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

impl_simple_bin_op!(addition, lhs, rhs, "Addition", {
    Int, Int => Ok(Value::Int(lhs + rhs)),
    Byte, Byte => Ok(Value::Byte(lhs.wrapping_add(rhs))),
    Float, Float => Ok(Value::Float(lhs + rhs)),
    String, String => {
        Ok(Value::String((lhs.to_string() + &*rhs).into()))
    },
});

impl_simple_bin_op!(subtraction, lhs, rhs, "Subtraction", {
    Int, Int => Ok(Value::Int(lhs - rhs)),
    Byte, Byte => Ok(Value::Byte(lhs.wrapping_sub(rhs))),
    Float, Float => Ok(Value::Float(lhs - rhs)),
});

impl_simple_bin_op!(multiplication, lhs, rhs, "Multiplication", {
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

impl_simple_bin_op!(division, lhs, rhs, "Division", {
    Int, Int => if rhs.is_zero() {
        Err(OperationError::Mathematical{
            why: "division by zero".to_string()
        })
    } else {
        Ok(Value::Int(lhs / rhs))
    },
    Byte, Byte => if rhs == 0 {
        Err(OperationError::Mathematical{
            why: "division by zero".to_string()
        })
    } else {
        Ok(Value::Byte(lhs / rhs))
    },
    Float, Float => Ok(Value::Float(lhs / rhs)),
});

impl_simple_bin_op!(modulo, lhs, rhs, "Modulo", {
    Int, Int => if rhs.is_zero() {
        Err(OperationError::Mathematical{
            why: "modulo by zero".to_string()
        })
    } else {
        Ok(Value::Int(((lhs % &rhs) + &rhs) % &rhs))
    },
    Byte, Byte => if rhs == 0 {
        Err(OperationError::Mathematical{
            why: "modulo by zero".to_string()
        })
    } else {
        Ok(Value::Byte(lhs.wrapping_rem_euclid(rhs)))
    },
    Float, Float => Ok(Value::Float(((lhs % rhs) + rhs) % rhs)),
});

impl_simple_bin_op!(exponentiation, lhs, rhs, "Exponentiation", {
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
