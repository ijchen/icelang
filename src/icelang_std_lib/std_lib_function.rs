use crate::{error::runtime_error::RuntimeError, runtime_state::RuntimeState, value::Value};

pub enum StdLibFunction {
    Println,
}

impl StdLibFunction {
    /// Gets the StdLibFunction corresponding to the given identifier if it
    /// exists
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        match identifier {
            "println" => Some(Self::Println),
            _ => None,
        }
    }

    /// Calls a StdLibFunction with the given arguments and RuntimeState
    pub fn call<'source>(
        &self,
        arguments: Vec<Value>,
        state: &mut RuntimeState<'source>,
    ) -> Result<Value, RuntimeError<'source>> {
        // Push a new stack frame
        state.push_stack_frame();

        let return_value = match self {
            Self::Println => match arguments.len() {
                0 => {
                    println!();

                    Value::Null
                }
                1 => {
                    println!("{}", arguments[0].icelang_display());

                    Value::Null
                }
                _ => todo!(),
            },
        };

        // Pop the stack frame
        state.pop_stack_frame();

        Ok(return_value)
    }
}
