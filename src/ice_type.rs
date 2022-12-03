//! Contains code related to ice types (the data type of values in ice)

use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
/// The data type of a value in ice
pub enum IceType {
    /// A signed, arbitrary precision integer
    Int,
    /// A single byte (8 bit value) interpreted as an unsigned integer in the
    /// range 0 to 255 (both inclusive)
    Byte,
    /// A floating point number with 64 bits of precision
    Float,
    /// A true or false value
    Bool,
    /// A resizable UTF-8 encoded string
    String,
    /// A resizable collection of values
    List,
    /// A resizable dictionary mapping keys to values
    Dict,
    /// A "nothing" value, representing the absence of a value
    Null,
}

impl Display for IceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Int => "int",
                Self::Byte => "byte",
                Self::Float => "float",
                Self::Bool => "bool",
                Self::String => "string",
                Self::List => "list",
                Self::Dict => "dict",
                Self::Null => "null",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(IceType::Int.to_string(), "int");
        assert_eq!(IceType::Byte.to_string(), "byte");
        assert_eq!(IceType::Float.to_string(), "float");
        assert_eq!(IceType::Bool.to_string(), "bool");
        assert_eq!(IceType::String.to_string(), "string");
        assert_eq!(IceType::List.to_string(), "list");
        assert_eq!(IceType::Dict.to_string(), "dict");
        assert_eq!(IceType::Null.to_string(), "null");
    }
}
