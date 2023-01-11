use std::fmt::{Debug, Display};

use crate::source_range::SourceRange;

use super::*;

/// Represents a node in an abstract syntax tree (AST)
#[derive(Debug, PartialEq, Eq)]
pub enum AstNode<'source> {
    /// A function declaration node
    FunctionDeclaration(AstNodeFunctionDeclaration<'source>),
    /// A variable declaration node
    VariableDeclaration(AstNodeVariableDeclaration<'source>),
    /// A variable access node
    VariableAccess(AstNodeVariableAccess<'source>),
    /// A literal node
    Literal(AstNodeLiteral<'source>),
    /// A type cast node
    TypeCast(AstNodeTypeCast<'source>),
    /// A usage suffix node
    UsageSuffix(AstNodeUsageSuffix<'source>),
    /// A binary operation node
    BinaryOperation(AstNodeBinaryOperation<'source>),
    /// A unary operation node
    UnaryOperation(AstNodeUnaryOperation<'source>),
    /// A comparison node
    Comparison(AstNodeComparison<'source>),
    /// An inline conditional node
    InlineConditional(AstNodeInlineConditional<'source>),
    /// An assignment node
    Assignment(AstNodeAssignment<'source>),
    /// A jump statement node
    JumpStatement(AstNodeJumpStatement<'source>),
}

impl<'source> AstNode<'source> {
    /// Returns the position in the source code of this AST node
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            AstNode::FunctionDeclaration(node) => node.pos(),
            AstNode::VariableDeclaration(node) => node.pos(),
            AstNode::VariableAccess(node) => node.pos(),
            AstNode::Literal(node) => node.pos(),
            AstNode::TypeCast(node) => node.pos(),
            AstNode::UsageSuffix(node) => node.pos(),
            AstNode::BinaryOperation(node) => node.pos(),
            AstNode::UnaryOperation(node) => node.pos(),
            AstNode::Comparison(node) => node.pos(),
            AstNode::InlineConditional(node) => node.pos(),
            AstNode::Assignment(node) => node.pos(),
            AstNode::JumpStatement(node) => node.pos(),
        }
    }
    /// Returns a mutable reference to the position in the source code of this
    /// AST node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        match self {
            AstNode::FunctionDeclaration(node) => node.pos_mut(),
            AstNode::VariableDeclaration(node) => node.pos_mut(),
            AstNode::VariableAccess(node) => node.pos_mut(),
            AstNode::Literal(node) => node.pos_mut(),
            AstNode::TypeCast(node) => node.pos_mut(),
            AstNode::UsageSuffix(node) => node.pos_mut(),
            AstNode::BinaryOperation(node) => node.pos_mut(),
            AstNode::UnaryOperation(node) => node.pos_mut(),
            AstNode::Comparison(node) => node.pos_mut(),
            AstNode::InlineConditional(node) => node.pos_mut(),
            AstNode::Assignment(node) => node.pos_mut(),
            AstNode::JumpStatement(node) => node.pos_mut(),
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
                AstNode::VariableDeclaration(node) => node.to_string(),
                AstNode::VariableAccess(node) => node.to_string(),
                AstNode::Literal(node) => node.to_string(),
                AstNode::TypeCast(node) => node.to_string(),
                AstNode::UsageSuffix(node) => node.to_string(),
                AstNode::BinaryOperation(node) => node.to_string(),
                AstNode::UnaryOperation(node) => node.to_string(),
                AstNode::Comparison(node) => node.to_string(),
                AstNode::InlineConditional(node) => node.to_string(),
                AstNode::Assignment(node) => node.to_string(),
                AstNode::JumpStatement(node) => node.to_string(),
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
impl_from_specific_ast_node!(AstNodeVariableDeclaration, VariableDeclaration);
impl_from_specific_ast_node!(AstNodeVariableAccess, VariableAccess);
impl_from_specific_ast_node!(AstNodeLiteral, Literal);
impl_from_specific_ast_node!(AstNodeTypeCast, TypeCast);
impl_from_specific_ast_node!(AstNodeUsageSuffix, UsageSuffix);
impl_from_specific_ast_node!(AstNodeBinaryOperation, BinaryOperation);
impl_from_specific_ast_node!(AstNodeUnaryOperation, UnaryOperation);
impl_from_specific_ast_node!(AstNodeComparison, Comparison);
impl_from_specific_ast_node!(AstNodeInlineConditional, InlineConditional);
impl_from_specific_ast_node!(AstNodeAssignment, Assignment);
impl_from_specific_ast_node!(AstNodeJumpStatement, JumpStatement);
