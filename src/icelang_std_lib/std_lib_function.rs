use crate::{
    interpreter::RuntimeResult, runtime_state::RuntimeState, source_range::SourceRange,
    value::Value,
};

use super::*;

pub enum StdLibFunction {
    Args,
    Print,
    Println,
    Eprint,
    Eprintln,
    Input,
    ReadFile,
    ReadFileBin,
    WriteFile,
    WriteFileBin,
}

impl StdLibFunction {
    /// Gets the StdLibFunction corresponding to the given identifier if it
    /// exists
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        match identifier {
            "args" => Some(Self::Args),
            "print" => Some(Self::Print),
            "println" => Some(Self::Println),
            "eprint" => Some(Self::Eprint),
            "eprintln" => Some(Self::Eprintln),
            "input" => Some(Self::Input),
            "read_file" => Some(Self::ReadFile),
            "read_file_bin" => Some(Self::ReadFileBin),
            "write_file" => Some(Self::WriteFile),
            "write_file_bin" => Some(Self::WriteFileBin),
            _ => None,
        }
    }

    /// Gets the Rust function corresponding to the icelang stdlib function
    pub fn as_fn_pointer(
        &self,
    ) -> for<'source> fn(
        Vec<Value>,
        &SourceRange<'source>,
        &mut RuntimeState<'source>,
    ) -> RuntimeResult<'source, Value> {
        match self {
            Self::Args => isl_args,
            Self::Print => isl_print,
            Self::Println => isl_println,
            Self::Eprint => isl_eprint,
            Self::Eprintln => isl_eprintln,
            Self::Input => isl_input,
            Self::ReadFile => isl_read_file,
            Self::ReadFileBin => isl_read_file_bin,
            Self::WriteFile => isl_write_file,
            Self::WriteFileBin => isl_write_file_bin,
        }
    }
}
