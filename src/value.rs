//! Contains code related to `Value`s, which represent icelang runtime values

use std::collections::HashMap;

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
