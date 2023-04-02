//! Contains code related to interpreting (reading and executing code
//! represented as an abstract syntax tree (AST))
mod assignments;
mod binary_operations;
mod casting;
mod comparisons;
mod control_flow;
mod core;
mod functions;
mod inline_conditionals;
mod jump_statement;
mod literals;
mod member_access;
mod operations;
mod runtime_result;
mod unary_operations;
mod variables;

use self::core::interpret_expression;
pub use self::core::{interpret, interpret_with_runtime_state};
pub use self::runtime_result::{NonLinearControlFlow, RuntimeResult};
use binary_operations::*;
use casting::*;
use literals::*;
