//! Contains code related to `RuntimeState`s, which represent the entire state
//! of an icelang program during execution.

use std::fmt::Display;

use crate::{
    icelang_std_lib::{IcelangFmt, IcelangFmtArgs},
    symbol_table::SymbolTable,
    value::Value,
};

/// Represents the entire state of an icelang program during execution
#[derive(Clone, Debug)]
pub struct RuntimeState<'source> {
    most_recent_value: Value,
    global_symbol_table: SymbolTable<'source>,
}

impl<'source> RuntimeState<'source> {
    /// Constructs a new default RuntimeState
    pub fn new() -> Self {
        Self {
            most_recent_value: Value::Null,
            global_symbol_table: SymbolTable::new(),
        }
    }

    /// Gets a reference to the global symbol table
    pub fn global_symbol_table(&self) -> &SymbolTable<'source> {
        &self.global_symbol_table
    }

    /// Gets a reference to the global symbol table
    pub fn global_symbol_table_mut(&mut self) -> &mut SymbolTable<'source> {
        &mut self.global_symbol_table
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

impl<'source> Default for RuntimeState<'source> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'source> Display for RuntimeState<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Most recent value: ")?;
        let fmt_args = IcelangFmtArgs { debug: true };
        self.most_recent_value.icelang_fmt(f, &fmt_args)?;
        Ok(())
    }
}
