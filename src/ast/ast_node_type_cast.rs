use std::fmt::Display;

use crate::{icelang_type::IcelangType, source_range::SourceRange};

use super::*;
use ast_node_format::format_as_node;

/// A type cast AST node
#[derive(Debug, Clone)]
pub struct AstNodeTypeCast<'source> {
    body: Box<AstNode<'source>>,
    destination_type: IcelangType,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeTypeCast<'source> {
    /// Constructs a new AstNodeTypeCast
    pub fn new(body: AstNode<'source>, new_type: IcelangType, pos: SourceRange<'source>) -> Self {
        Self {
            body: Box::new(body),
            destination_type: new_type,
            pos,
        }
    }

    /// Returns the body of the type cast node (the value to be cast)
    pub fn body(&self) -> &AstNode<'source> {
        &self.body
    }

    /// Returns the destination icelang type of the cast
    pub fn destination_type(&self) -> IcelangType {
        self.destination_type
    }

    /// Returns the position in the source code of this type cast
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// type cast
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl PartialEq for AstNodeTypeCast<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body && self.destination_type == other.destination_type
    }
}
impl Eq for AstNodeTypeCast<'_> {}

impl Display for AstNodeTypeCast<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!("[Type Cast] to {}", self.destination_type),
                vec![self.body.to_string()]
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use crate::value::Value;

    use super::*;

    #[test]
    fn test_ast_node_type_cast_int() {
        let nowhere = SourceRange::new(" ", "", 0, 0);
        let body: AstNode = AstNodeLiteral::new(
            "123".to_string(),
            IcelangType::Int,
            Value::Int(BigInt::from(123)),
            nowhere.clone(),
        )
        .into();
        let node: AstNode = AstNodeTypeCast::new(body, IcelangType::Float, nowhere).into();

        assert_eq!(
            node.to_string(),
            "\
● [Type Cast] to float
└─● [Literal] (int) 123"
        );
    }
}
