//! Contains code related to `RuntimeState`s, which represent the entire state
//! of an icelang program during execution.

/// Represents the entire state of an icelang program during execution
#[derive(Clone, Debug)]
pub struct RuntimeState {}

impl RuntimeState {
    /// Constructs a new default RuntimeState
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}
