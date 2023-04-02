use crate::{
    interpreter::RuntimeResult, runtime_state::RuntimeState, source_range::SourceRange,
    value::Value,
};

use super::*;

pub enum StdLibFunction {
    Print,
    Println,
    Eprint,
    Eprintln,
    Input,
}

impl StdLibFunction {
    /// Gets the StdLibFunction corresponding to the given identifier if it
    /// exists
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        match identifier {
            "print" => Some(Self::Print),
            "println" => Some(Self::Println),
            "eprint" => Some(Self::Eprint),
            "eprintln" => Some(Self::Eprintln),
            "input" => Some(Self::Input),
            _ => None,
        }
    }

    /// Calls a StdLibFunction with the given arguments and RuntimeState
    pub fn call<'source>(
        &self,
        arguments: Vec<Value>,
        pos: &SourceRange<'source>,
        state: &mut RuntimeState<'source>,
    ) -> RuntimeResult<'source, Value> {
        let return_value = match self {
            StdLibFunction::Print => isl_print(arguments, pos, state),
            StdLibFunction::Println => isl_println(arguments, pos, state),
            StdLibFunction::Eprint => isl_eprint(arguments, pos, state),
            StdLibFunction::Eprintln => isl_eprintln(arguments, pos, state),
            StdLibFunction::Input => isl_input(arguments, pos, state),
        }?;

        Ok(return_value)
    }
}
