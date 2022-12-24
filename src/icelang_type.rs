//! Contains code related to icelang types (the data type of values in icelang)

use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
/// The data type of a value in icelang
pub enum IcelangType {
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

impl Display for IcelangType {
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
        assert_eq!(IcelangType::Int.to_string(), "int");
        assert_eq!(IcelangType::Byte.to_string(), "byte");
        assert_eq!(IcelangType::Float.to_string(), "float");
        assert_eq!(IcelangType::Bool.to_string(), "bool");
        assert_eq!(IcelangType::String.to_string(), "string");
        assert_eq!(IcelangType::List.to_string(), "list");
        assert_eq!(IcelangType::Dict.to_string(), "dict");
        assert_eq!(IcelangType::Null.to_string(), "null");
    }
}
