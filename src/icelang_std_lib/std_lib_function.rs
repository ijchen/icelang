use crate::{
    interpreter::RuntimeResult, runtime_state::RuntimeState, source_range::SourceRange,
    value::Value,
};

use super::{
    string::{isl_from_codepoint, isl_to_codepoint},
    *,
};

pub enum StdLibFunction {
    // I/O
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

    // Collections
    Len,
    Push,
    Pop,
    PushStart,
    PopStart,
    ContainsKey,
    RemoveEntry,
    Keys,

    // String
    FromCodepoint,
    ToCodepoint,
}

impl StdLibFunction {
    /// Gets the StdLibFunction corresponding to the given identifier if it
    /// exists
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        match identifier {
            // I/O
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

            // Collections
            "len" => Some(Self::Len),
            "push" => Some(Self::Push),
            "pop" => Some(Self::Pop),
            "push_start" => Some(Self::PushStart),
            "pop_start" => Some(Self::PopStart),
            "contains_key" => Some(Self::ContainsKey),
            "remove_entry" => Some(Self::RemoveEntry),
            "keys" => Some(Self::Keys),

            // String
            "from_codepoint" => Some(Self::FromCodepoint),
            "to_codepoint" => Some(Self::ToCodepoint),

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
            // I/O
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

            // Collections
            Self::Len => isl_len,
            Self::Push => isl_push,
            Self::Pop => isl_pop,
            Self::PushStart => isl_push_start,
            Self::PopStart => isl_pop_start,
            Self::ContainsKey => isl_contains_key,
            Self::RemoveEntry => isl_remove_entry,
            Self::Keys => isl_keys,

            // String
            Self::FromCodepoint => isl_from_codepoint,
            Self::ToCodepoint => isl_to_codepoint,
        }
    }
}
