//! An interpreter for icelang, written in Rust

#![warn(missing_docs)]

mod icelang_error;
pub mod icelang_type;
pub mod keyword;
pub mod lexer;
mod source_range;
pub mod token;
