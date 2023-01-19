//! Contains code related to `RuntimeState`s, which represent the entire state
//! of an icelang program during execution.

use std::fmt::Display;

use crate::value::Value;

/// Represents the entire state of an icelang program during execution
#[derive(Clone, Debug)]
pub struct RuntimeState {
    most_recent_value: Value,
}

impl RuntimeState {
    /// Constructs a new default RuntimeState
    pub fn new() -> Self {
        Self {
            most_recent_value: Value::Null,
        }
    }

    /// Returns the most recent value from an expression
    pub fn most_recent_value(&self) -> &Value {
        &self.most_recent_value
    }

    /// Updates the most recent value from an expression
    pub fn update_most_recent_value(&mut self, value: Value) {
        self.most_recent_value = value;
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for RuntimeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Most recent value: {}", self.most_recent_value)
    }
}
