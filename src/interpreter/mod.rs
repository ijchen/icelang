//! Contains code related to interpreting (reading and executing code
//! represented as an abstract syntax tree (AST))
mod binary_operations;
mod core;
mod literals;

use self::core::interpret_expression;
pub use self::core::{interpret, interpret_with_runtime_state};
use binary_operations::*;
use literals::*;
