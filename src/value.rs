//! Contains code related to `Value`s, which represent icelang runtime values

use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use num_bigint::BigInt;
use ordered_float::OrderedFloat;

use crate::{
    icelang_std_lib::{IcelangFmt, IcelangFmtArgs},
    icelang_type::IcelangType,
};

/// Represents an icelang runtime value
#[derive(Debug)]
pub enum Value {
    /// An int value
    Int(BigInt),

    /// A byte value
    Byte(u8),

    /// A float value
    Float(f64),

    /// A bool value
    Bool(bool),

    /// A string value
    String(Rc<str>),

    /// A list value
    List(Rc<RefCell<Vec<Value>>>),

    /// A dict value
    Dict(Rc<RefCell<HashMap<Value, Value>>>),

    /// A null value
    Null,
}

impl Value {
    /// Performs a reference copy of the value. This means that the two values
    /// are copied by-reference, and (for the non-copy types) changes to the
    /// original will be reflected in the copy, and vice-versa
    pub fn reference_copy(&self) -> Value {
        match self {
            Value::Int(value) => Value::Int(value.clone()),
            Value::Byte(value) => Value::Byte(*value),
            Value::Float(value) => Value::Float(*value),
            Value::Bool(value) => Value::Bool(*value),
            Value::String(value) => Value::String(value.clone()),
            Value::List(value) => Value::List(value.clone()),
            Value::Dict(value) => Value::Dict(value.clone()),
            Value::Null => Value::Null,
        }
    }

    /// Performs a shallow copy of the value. This means that the value is
    /// copied, and changes to the original will not be reflected in the copy
    /// (and vice-versa). However, since this is a shallow copy, changes to any
    /// of the values *stored inside* the original will be reflected in the
    /// copy, and vice-versa. This only matters for types that contain other
    /// values, like `list`s or `dict`s
    pub fn shallow_copy(&self) -> Value {
        match self {
            Value::Int(value) => Value::Int(value.clone()),
            Value::Byte(value) => Value::Byte(*value),
            Value::Float(value) => Value::Float(*value),
            Value::Bool(value) => Value::Bool(*value),
            Value::String(value) => Value::String(value.clone()),
            Value::List(value) => Value::List(Rc::new(RefCell::new(
                value
                    .borrow()
                    .iter()
                    .map(|value| value.reference_copy())
                    .collect(),
            ))),
            Value::Dict(value) => Value::Dict(Rc::new(RefCell::new(
                value
                    .borrow()
                    .iter()
                    .map(|(k, v)| (k.reference_copy(), v.reference_copy()))
                    .collect(),
            ))),
            Value::Null => Value::Null,
        }
    }

    /// Performs a deep copy of the value. This means that the value is copied,
    /// and changes to the original will not be reflected in the copy (and
    /// vice-versa). Additionally, since this is a deep copy, changes to any
    /// of the values stored inside the original will **not** be reflected in
    /// the copy, and vice-versa. This only matters for types that contain other
    /// values, like `list`s or `dict`s
    pub fn deep_copy(&self) -> Value {
        match self {
            Value::Int(value) => Value::Int(value.clone()),
            Value::Byte(value) => Value::Byte(*value),
            Value::Float(value) => Value::Float(*value),
            Value::Bool(value) => Value::Bool(*value),
            Value::String(value) => Value::String(value.clone()),
            Value::List(value) => Value::List(Rc::new(RefCell::new(
                value
                    .borrow()
                    .iter()
                    .map(|value| value.deep_copy())
                    .collect(),
            ))),
            Value::Dict(value) => Value::Dict(Rc::new(RefCell::new(
                value
                    .borrow()
                    .iter()
                    .map(|(k, v)| (k.deep_copy(), v.deep_copy()))
                    .collect(),
            ))),
            Value::Null => Value::Null,
        }
    }

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

    /// Returns a human-readable stringified version of this value
    pub fn icelang_display(&self) -> String {
        let mut buffer = String::new();

        let fmt_args = Default::default();
        self.icelang_fmt(&mut buffer, &fmt_args).unwrap();

        buffer
    }

    /// Returns a human-readable debug stringified version of this value
    pub fn icelang_debug(&self) -> String {
        let mut buffer = String::new();

        let fmt_args = IcelangFmtArgs { debug: true };
        self.icelang_fmt(&mut buffer, &fmt_args).unwrap();

        buffer
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::Byte(lhs), Self::Byte(rhs)) => lhs == rhs,
            (Self::Float(lhs), Self::Float(rhs)) => lhs.is_nan() && rhs.is_nan() || lhs == rhs,
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::List(lhs), Self::List(rhs)) => lhs == rhs,
            (Self::Dict(lhs), Self::Dict(rhs)) => {
                lhs.borrow().len() == rhs.borrow().len()
                    && lhs
                        .borrow()
                        .iter()
                        .all(|(key, value)| rhs.borrow().get(key) == Some(value))
            }
            (Self::Null, Self::Null) => true,
            (_, _) => false,
        }
    }
}
impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Value::Int(value) => value.hash(state),
            Value::Byte(value) => value.hash(state),
            Value::Float(value) => {
                OrderedFloat(*value).hash(state);
                let _ = value.to_bits();
            }
            Value::Bool(value) => value.hash(state),
            Value::String(value) => value.hash(state),
            Value::List(value) => value.borrow().hash(state),
            Value::Dict(value) => {
                value.borrow().len().hash(state);
                for (key, value) in value.borrow().iter() {
                    key.hash(state);
                    value.hash(state);
                }
            }
            Value::Null => {}
        };
    }
}
