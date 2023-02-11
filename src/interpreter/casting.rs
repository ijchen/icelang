use num_bigint::BigInt;
use num_traits::{FromPrimitive, ToPrimitive};

use super::*;
use crate::{
    ast::AstNodeTypeCast, error::runtime_error::RuntimeError, icelang_type::IcelangType,
    runtime_state::RuntimeState, value::Value,
};

/// Interprets a list literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeListLiteral isn't a valid list literal
pub fn interpret_type_cast<'source>(
    node: &AstNodeTypeCast<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    let value = interpret_expression(node.body(), state)?;

    match (&value, node.new_type()) {
        (Value::Int(value), IcelangType::Byte) => Ok(if let Ok(byte) = value.try_into() {
            Value::Byte(byte)
        } else {
            Value::Null
        }),
        (Value::Int(value), IcelangType::Float) => Ok(Value::Float(value.to_f64().unwrap())),
        (Value::Byte(value), IcelangType::Int) => Ok(Value::Int(BigInt::from(*value))),
        (Value::Byte(value), IcelangType::Float) => Ok(Value::Float(*value as f64)),
        (Value::Float(value), IcelangType::Int) => Ok(if value.is_infinite() || value.is_nan() {
            Value::Null
        } else {
            Value::Int(BigInt::from_f64(*value).unwrap())
        }),
        (
            Value::Int(_) | Value::Byte(_) | Value::Float(_) | Value::Bool(_),
            IcelangType::String,
        ) => Ok(Value::String(value.icelang_display().into())),
        (Value::String(_), IcelangType::Int) => todo!(),
        (Value::String(_), IcelangType::Byte) => todo!(),
        (Value::String(_), IcelangType::Float) => todo!(),
        (value, new_type) => Err(RuntimeError::new_type_error(
            node.pos().clone(),
            state.scope_display_name().to_string(),
            format!("cannot cast from {} to {}", value.icelang_type(), new_type),
        )),
    }
}
