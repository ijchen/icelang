use num_bigint::BigInt;
use num_traits::{FromPrimitive, ToPrimitive};

use super::{
    runtime_result::{NonLinearControlFlow, RuntimeResult},
    *,
};
use crate::{
    ast::AstNodeTypeCast, error::runtime_error::RuntimeError, icelang_type::IcelangType,
    runtime_state::RuntimeState, value::Value,
};

pub fn cast(value: &Value, destination_type: IcelangType) -> Option<Value> {
    match (value, destination_type) {
        // Integer casts
        (Value::Int(value), IcelangType::Byte) => {
            Some(value.try_into().map(Value::Byte).unwrap_or(Value::Null))
        }
        (Value::Byte(value), IcelangType::Int) => Some(Value::Int(BigInt::from(*value))),

        // Float casts
        (Value::Int(value), IcelangType::Float) => Some(Value::Float(value.to_f64().unwrap())),
        (Value::Byte(value), IcelangType::Float) => Some(Value::Float(f64::from(*value))),
        (Value::Float(value), IcelangType::Int) => Some(
            BigInt::from_f64(*value)
                .map(Value::Int)
                .unwrap_or(Value::Null),
        ),

        // To-string casts
        (
            Value::Int(_) | Value::Byte(_) | Value::Float(_) | Value::Bool(_),
            IcelangType::String,
        ) => Some(Value::String(value.icelang_display().into())),

        // From-string casts
        (Value::String(_value), IcelangType::Int) => todo!(),
        (Value::String(_value), IcelangType::Byte) => todo!(),
        (Value::String(_value), IcelangType::Float) => todo!(),

        // Same-type casts are not permitted
        (Value::Int(_), IcelangType::Int) => None,
        (Value::Byte(_), IcelangType::Byte) => None,
        (Value::Float(_), IcelangType::Float) => None,
        (Value::Bool(_), IcelangType::Bool) => None,
        (Value::String(_), IcelangType::String) => None,
        (Value::List(_), IcelangType::List) => None,
        (Value::Dict(_), IcelangType::Dict) => None,
        (Value::Null, IcelangType::Null) => None,

        // Casts to or from null are not permitted
        (Value::Null, _) | (_, IcelangType::Null) => None,

        // Unsupported/nonsense casts
        (Value::Int(_), IcelangType::Bool) => None,
        (Value::Int(_), IcelangType::List) => None,
        (Value::Int(_), IcelangType::Dict) => None,
        (Value::Byte(_), IcelangType::Bool) => None,
        (Value::Byte(_), IcelangType::List) => None,
        (Value::Byte(_), IcelangType::Dict) => None,
        (Value::Float(_), IcelangType::Byte) => None,
        (Value::Float(_), IcelangType::Bool) => None,
        (Value::Float(_), IcelangType::List) => None,
        (Value::Float(_), IcelangType::Dict) => None,
        (Value::Bool(_), IcelangType::Int) => None,
        (Value::Bool(_), IcelangType::Byte) => None,
        (Value::Bool(_), IcelangType::Float) => None,
        (Value::Bool(_), IcelangType::List) => None,
        (Value::Bool(_), IcelangType::Dict) => None,
        (Value::String(_), IcelangType::Bool) => None,
        (Value::String(_), IcelangType::List) => None,
        (Value::String(_), IcelangType::Dict) => None,
        (Value::List(_), IcelangType::Int) => None,
        (Value::List(_), IcelangType::Byte) => None,
        (Value::List(_), IcelangType::Float) => None,
        (Value::List(_), IcelangType::Bool) => None,
        (Value::List(_), IcelangType::String) => None,
        (Value::List(_), IcelangType::Dict) => None,
        (Value::Dict(_), IcelangType::Int) => None,
        (Value::Dict(_), IcelangType::Byte) => None,
        (Value::Dict(_), IcelangType::Float) => None,
        (Value::Dict(_), IcelangType::Bool) => None,
        (Value::Dict(_), IcelangType::String) => None,
        (Value::Dict(_), IcelangType::List) => None,
    }
}

/// Interprets a list literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeListLiteral isn't a valid list literal
pub fn interpret_type_cast<'source>(
    node: &AstNodeTypeCast<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    let value = interpret_expression(node.body(), state)?;
    let destination_type = node.destination_type();

    cast(&value, destination_type).ok_or_else(|| {
        NonLinearControlFlow::RuntimeError(RuntimeError::new_type_error(
            node.pos().clone(),
            state.scope_display_name().to_string(),
            format!(
                "cannot cast from {} to {}",
                value.icelang_type(),
                destination_type
            ),
        ))
    })
}
