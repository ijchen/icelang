//! An interpreter for icelang, written in Rust

#![warn(missing_docs)]

pub mod ast;
pub mod error;
pub mod icelang_type;
pub mod keyword;
pub mod lexer;
pub mod parser;
pub mod source_range;
mod test_utils;
pub mod token;
