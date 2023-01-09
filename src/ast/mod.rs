//! Contains code related to AST nodes

mod ast_node_binary_operation;
mod ast_node_comparison;
mod ast_node_format;
mod ast_node_function_declaration;
mod ast_node_inline_conditional;
mod ast_node_literal;
mod ast_node_type_cast;
mod ast_node_unary_operation;
mod ast_node_usage_suffix;
mod ast_node_variable_access;
mod ast_struct;
mod binary_operation_kind;
mod comparison_kind;
mod generic_ast_node;
mod unary_operation_kind;
mod usage_suffix;

pub use ast_node_binary_operation::AstNodeBinaryOperation;
pub use ast_node_comparison::AstNodeComparison;
pub use ast_node_function_declaration::{AstNodeFunctionDeclaration, FunctionParameters};
pub use ast_node_inline_conditional::AstNodeInlineConditional;
pub use ast_node_literal::AstNodeLiteral;
pub use ast_node_type_cast::AstNodeTypeCast;
pub use ast_node_unary_operation::AstNodeUnaryOperation;
pub use ast_node_usage_suffix::AstNodeUsageSuffix;
pub use ast_node_variable_access::AstNodeVariableAccess;
pub use ast_struct::Ast;
pub use binary_operation_kind::BinaryOperationKind;
pub use comparison_kind::ComparisonKind;
pub use generic_ast_node::AstNode;
pub use unary_operation_kind::UnaryOperationKind;
pub use usage_suffix::{
    UsageSuffix, UsageSuffixComputedMemberAccess, UsageSuffixDotMemberAccess,
    UsageSuffixFunctionCall,
};
