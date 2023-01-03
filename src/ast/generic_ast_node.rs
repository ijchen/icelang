use std::fmt::{Debug, Display};

use crate::source_range::SourceRange;

use super::*;

/// Represents a node in an abstract syntax tree (AST)
#[derive(Debug, PartialEq, Eq)]
pub enum AstNode<'source> {
    /// A function declaration
    FunctionDeclaration(AstNodeFunctionDeclaration<'source>),
    /// A variable access node
    VariableAccess(AstNodeVariableAccess<'source>),
    /// A literal
    Literal(AstNodeLiteral<'source>),
}

impl<'source> AstNode<'source> {
    /// Returns the position in the source code of this AST node
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            AstNode::FunctionDeclaration(node) => node.pos(),
            AstNode::VariableAccess(node) => node.pos(),
            AstNode::Literal(node) => node.pos(),
        }
    }
    /// Returns a mutable reference to the position in the source code of this
    /// AST node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        match self {
            AstNode::FunctionDeclaration(node) => node.pos_mut(),
            AstNode::VariableAccess(node) => node.pos_mut(),
            AstNode::Literal(node) => node.pos_mut(),
        }
    }
}

impl Display for AstNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AstNode::FunctionDeclaration(node) => node.to_string(),
                AstNode::VariableAccess(node) => node.to_string(),
                AstNode::Literal(node) => node.to_string(),
            }
        )
    }
}

macro_rules! impl_from_specific_ast_node {
    ($specific_type:ident, $variant_name:ident) => {
        impl<'source> From<$specific_type<'source>> for AstNode<'source> {
            fn from(node: $specific_type<'source>) -> Self {
                Self::$variant_name(node)
            }
        }
    };
}
impl_from_specific_ast_node!(AstNodeFunctionDeclaration, FunctionDeclaration);
impl_from_specific_ast_node!(AstNodeVariableAccess, VariableAccess);
impl_from_specific_ast_node!(AstNodeLiteral, Literal);
