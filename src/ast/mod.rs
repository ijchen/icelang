//! Contains code related to AST nodes

mod ast_node_format;
mod ast_node_function_declaration;
mod ast_struct;
mod generic_ast_node;

pub use ast_node_function_declaration::{AstNodeFunctionDeclaration, FunctionParameters};
pub use ast_struct::Ast;
pub use generic_ast_node::AstNode;
