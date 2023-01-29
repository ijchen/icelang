//! Contains code related to interpreting (reading and executing code
//! represented as an abstract syntax tree (AST))
mod binary_operations;
mod casting;
mod core;
mod literals;
mod variables;

use self::core::interpret_expression;
pub use self::core::{interpret, interpret_with_runtime_state};
use binary_operations::*;
use casting::*;
use literals::*;
