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
    /// A list literal node
    ListLiteral(AstNodeListLiteral<'source>),
    /// A formatted string literal node
    FormattedStringLiteral(AstNodeFormattedStringLiteral<'source>),
    /// A dict literal node
    DictLiteral(AstNodeDictLiteral<'source>),
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
    /// A simple (`loop`) loop node
    SimpleLoop(AstNodeSimpleLoop<'source>),
    /// A while loop node
    WhileLoop(AstNodeWhileLoop<'source>),
    /// A for loop node
    ForLoop(AstNodeForLoop<'source>),
    /// A match statement node
    MatchStatement(AstNodeMatchStatement<'source>),
    /// An if-else statement node
    IfElseStatement(AstNodeIfElseStatement<'source>),
}

impl<'source> AstNode<'source> {
    /// Returns the position in the source code of this AST node
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            AstNode::FunctionDeclaration(node) => node.pos(),
            AstNode::VariableDeclaration(node) => node.pos(),
            AstNode::VariableAccess(node) => node.pos(),
            AstNode::Literal(node) => node.pos(),
            AstNode::FormattedStringLiteral(node) => node.pos(),
            AstNode::ListLiteral(node) => node.pos(),
            AstNode::DictLiteral(node) => node.pos(),
            AstNode::TypeCast(node) => node.pos(),
            AstNode::UsageSuffix(node) => node.pos(),
            AstNode::BinaryOperation(node) => node.pos(),
            AstNode::UnaryOperation(node) => node.pos(),
            AstNode::Comparison(node) => node.pos(),
            AstNode::InlineConditional(node) => node.pos(),
            AstNode::Assignment(node) => node.pos(),
            AstNode::JumpStatement(node) => node.pos(),
            AstNode::SimpleLoop(node) => node.pos(),
            AstNode::WhileLoop(node) => node.pos(),
            AstNode::ForLoop(node) => node.pos(),
            AstNode::MatchStatement(node) => node.pos(),
            AstNode::IfElseStatement(node) => node.pos(),
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
            AstNode::FormattedStringLiteral(node) => node.pos_mut(),
            AstNode::ListLiteral(node) => node.pos_mut(),
            AstNode::DictLiteral(node) => node.pos_mut(),
            AstNode::TypeCast(node) => node.pos_mut(),
            AstNode::UsageSuffix(node) => node.pos_mut(),
            AstNode::BinaryOperation(node) => node.pos_mut(),
            AstNode::UnaryOperation(node) => node.pos_mut(),
            AstNode::Comparison(node) => node.pos_mut(),
            AstNode::InlineConditional(node) => node.pos_mut(),
            AstNode::Assignment(node) => node.pos_mut(),
            AstNode::JumpStatement(node) => node.pos_mut(),
            AstNode::SimpleLoop(node) => node.pos_mut(),
            AstNode::WhileLoop(node) => node.pos_mut(),
            AstNode::ForLoop(node) => node.pos_mut(),
            AstNode::MatchStatement(node) => node.pos_mut(),
            AstNode::IfElseStatement(node) => node.pos_mut(),
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
                AstNode::FormattedStringLiteral(node) => node.to_string(),
                AstNode::ListLiteral(node) => node.to_string(),
                AstNode::DictLiteral(node) => node.to_string(),
                AstNode::TypeCast(node) => node.to_string(),
                AstNode::UsageSuffix(node) => node.to_string(),
                AstNode::BinaryOperation(node) => node.to_string(),
                AstNode::UnaryOperation(node) => node.to_string(),
                AstNode::Comparison(node) => node.to_string(),
                AstNode::InlineConditional(node) => node.to_string(),
                AstNode::Assignment(node) => node.to_string(),
                AstNode::JumpStatement(node) => node.to_string(),
                AstNode::SimpleLoop(node) => node.to_string(),
                AstNode::WhileLoop(node) => node.to_string(),
                AstNode::ForLoop(node) => node.to_string(),
                AstNode::MatchStatement(node) => node.to_string(),
                AstNode::IfElseStatement(node) => node.to_string(),
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
impl_from_specific_ast_node!(AstNodeListLiteral, ListLiteral);
impl_from_specific_ast_node!(AstNodeFormattedStringLiteral, FormattedStringLiteral);
impl_from_specific_ast_node!(AstNodeDictLiteral, DictLiteral);
impl_from_specific_ast_node!(AstNodeTypeCast, TypeCast);
impl_from_specific_ast_node!(AstNodeUsageSuffix, UsageSuffix);
impl_from_specific_ast_node!(AstNodeBinaryOperation, BinaryOperation);
impl_from_specific_ast_node!(AstNodeUnaryOperation, UnaryOperation);
impl_from_specific_ast_node!(AstNodeComparison, Comparison);
impl_from_specific_ast_node!(AstNodeInlineConditional, InlineConditional);
impl_from_specific_ast_node!(AstNodeAssignment, Assignment);
impl_from_specific_ast_node!(AstNodeJumpStatement, JumpStatement);
impl_from_specific_ast_node!(AstNodeSimpleLoop, SimpleLoop);
impl_from_specific_ast_node!(AstNodeWhileLoop, WhileLoop);
impl_from_specific_ast_node!(AstNodeForLoop, ForLoop);
impl_from_specific_ast_node!(AstNodeMatchStatement, MatchStatement);
impl_from_specific_ast_node!(AstNodeIfElseStatement, IfElseStatement);
