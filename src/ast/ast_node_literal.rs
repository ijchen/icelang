use std::fmt::Display;

use crate::{icelang_type::IcelangType, source_range::SourceRange, value::Value};

use super::*;
use ast_node_format::format_as_node;

/// A literal AST node
#[derive(Debug)]
pub struct AstNodeLiteral<'source> {
    raw: String,
    icelang_type: IcelangType,
    value: Value,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeLiteral<'source> {
    /// Constructs a new AstNodeLiteral
    pub fn new(
        raw: String,
        icelang_type: IcelangType,
        value: Value,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            raw,
            icelang_type,
            value,
            pos,
        }
    }

    /// Returns the raw version of the literal (as it appeared in the source)
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the icelang type of the literal
    pub fn icelang_type(&self) -> IcelangType {
        self.icelang_type
    }

    /// Returns the value of the literal
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Returns the position in the source code of this variable access
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// variable access
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Clone for AstNodeLiteral<'_> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            icelang_type: self.icelang_type,
            value: self.value.deep_copy(),
            pos: self.pos.clone(),
        }
    }
}

impl PartialEq for AstNodeLiteral<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw && self.icelang_type == other.icelang_type
    }
}
impl Eq for AstNodeLiteral<'_> {}

impl Display for AstNodeLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!("[Literal] ({}) {}", self.icelang_type, self.raw),
                vec![]
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use super::*;

    #[test]
    fn test_ast_node_display_literal_int() {
        let nowhere = SourceRange::new(" ", "", 0, 0);
        let node: AstNode = AstNodeLiteral::new(
            "123".to_string(),
            IcelangType::Int,
            Value::Int(BigInt::from(123)),
            nowhere,
        )
        .into();

        assert_eq!(node.to_string(), "● [Literal] (int) 123");
    }

    #[test]
    fn test_ast_node_display_literal_float_nan() {
        let nowhere = SourceRange::new(" ", "", 0, 0);
        let node: AstNode = AstNodeLiteral::new(
            "NaN".to_string(),
            IcelangType::Float,
            Value::Float(f64::NAN),
            nowhere,
        )
        .into();

        assert_eq!(node.to_string(), "● [Literal] (float) NaN");
    }
}
