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
mod literals;
mod member_access;
mod unary_operations;
mod variables;

use self::core::interpret_expression;
pub use self::core::{interpret, interpret_with_runtime_state};
use binary_operations::*;
use casting::*;
use literals::*;