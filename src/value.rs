//! Contains code related to `Value`s, which represent icelang runtime values

use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use num_bigint::BigInt;
use ordered_float::OrderedFloat;

use crate::{
    icelang_std_lib::{IcelangFmt, IcelangFmtArgs},
    icelang_type::IcelangType,
};

/// Represents an icelang runtime value
#[derive(Clone, Debug)]
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
