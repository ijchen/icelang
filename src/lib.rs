//! An interpreter for icelang, written in Rust

#![warn(missing_docs, missing_debug_implementations)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod ast;
mod call_stack;
pub mod error;
pub mod function;
mod icelang_std_lib;
pub mod icelang_type;
pub mod interpreter;
pub mod keyword;
pub mod lexer;
pub mod parser;
pub mod runtime_state;
pub mod source_range;
mod symbol_table;
mod test_utils;
pub mod token;
pub mod value;
