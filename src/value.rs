//! Contains code related to `Value`s, which represent icelang runtime values

use std::collections::HashMap;

use crate::icelang_type::IcelangType;

/// Represents an icelang runtime value
#[derive(Clone, Debug)]
pub enum Value {
    /// An int value
    Int(i64), // TODO use a BigInt

    /// A byte value
    Byte(u8),

    /// A float value
    Float(f64),

    /// A bool value
    Bool(bool),

    /// A string value
    String(String),

    /// A list value
    List(Vec<Value>),

    /// A dict value
    Dict(HashMap<Value, Value>),

    /// A null value
    Null,
}

impl Value {
    /// Gets the icelang type of the value
    pub fn icelang_type(&self) -> IcelangType {
        match self {
            Self::Int(_) => IcelangType::Int,
            Self::Byte(_) => IcelangType::Byte,
            Self::Float(_) => IcelangType::Float,
            Self::Bool(_) => IcelangType::Bool,
            Self::String(_) => IcelangType::String,
            Self::List(_) => IcelangType::List,
            Self::Dict(_) => IcelangType::Dict,
            Self::Null => IcelangType::Null,
        }
    }

    /// Attempts to cast the value to the given type, returning None if the cast
    /// has an invalid source and destination type
    pub fn casted_to(&self, destination_type: IcelangType) -> Option<Value> {
        match (self, destination_type) {
            (Value::Int(value), IcelangType::Byte) => Some(if 0x00 <= *value && *value <= 0xFF {
                Value::Byte(*value as u8)
            } else {
                Value::Null
            }),
            (Value::Int(value), IcelangType::Float) => Some(Value::Float(*value as f64)),
            (Value::Byte(value), IcelangType::Int) => Some(Value::Int(*value as i64)),
            (Value::Byte(value), IcelangType::Float) => Some(Value::Float(*value as f64)),
            (Value::Float(value), IcelangType::Int) => {
                Some(if value.is_infinite() || value.is_nan() {
                    Value::Null
                } else {
                    // TODO remove once ints are arbitrary size
                    if *value as i64 == i64::MAX {
                        todo!();
                    }
                    Value::Int(*value as i64)
                })
            }
            (Value::Int(_), IcelangType::String) => todo!(),
            (Value::Byte(_), IcelangType::String) => todo!(),
            (Value::Float(_), IcelangType::String) => todo!(),
            (Value::Bool(_), IcelangType::String) => todo!(),
            (Value::String(_), IcelangType::Int) => todo!(),
            (Value::String(_), IcelangType::Byte) => todo!(),
            (Value::String(_), IcelangType::Float) => todo!(),
            (_, _) => None,
        }
    }
}
