//! An interpreter for icelang, written in Rust

#![warn(missing_docs)]

mod ice_error;
pub mod ice_type;
pub mod lexer;
mod source_range;
pub mod token;
