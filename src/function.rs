//! Contains code related to icelang functions

use std::{collections::HashMap, fmt::Display};

use crate::{ast::AstNode, source_range::SourceRange};

/// Represents parameters to an icelang function
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FunctionParameters {
    /// A variadic function (one which accepts any number of arguments)
    Variadic {
        /// The identifier for the parameter list parameter
        parameter_name: String,
    },
    /// A "normal" polyadic (fixed-arity) function (one which only accepts a
    /// fixed number of arguments)
    Polyadic {
        /// The parameter identifiers
        parameters: Vec<String>,
    },
}

impl FunctionParameters {
    /// Returns whether or not the arity is variadic
    pub fn is_variadic(&self) -> bool {
        match self {
            FunctionParameters::Variadic { parameter_name: _ } => true,
            FunctionParameters::Polyadic { parameters: _ } => false,
        }
    }

    /// Returns whether or not the arity is polyadic
    pub fn is_polyadic(&self) -> bool {
        match self {
            FunctionParameters::Variadic { parameter_name: _ } => false,
            FunctionParameters::Polyadic { parameters: _ } => true,
        }
    }

    /// Returns the arity of a polyadic FunctionParameters
    ///
    /// # Panics
    /// - If self is not the Polyadic variant
    pub fn get_arity(&self) -> usize {
        match self {
            FunctionParameters::Variadic { parameter_name: _ } => {
                panic!("attempted to get the arity of a variadic FunctionParameters");
            }
            FunctionParameters::Polyadic { parameters } => parameters.len(),
        }
    }
}

impl Display for FunctionParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionParameters::Variadic {
                parameter_name: list_name,
            } => write!(f, "[{list_name}]"),
            FunctionParameters::Polyadic { parameters } => write!(f, "{}", parameters.join(", ")),
        }
    }
}

/// A possibly overloaded icelang function group
#[derive(Debug, Clone)]
pub struct FunctionGroup<'source> {
    variadic_overload: Option<Function<'source>>,
    polyadic_overloads: HashMap<usize, Function<'source>>,
}

impl<'source> FunctionGroup<'source> {
    /// Constructs a new (empty) function group
    pub fn new() -> Self {
        Self {
            variadic_overload: None,
            polyadic_overloads: HashMap::new(),
        }
    }

    /// Gets the function overload for `arg_count` arguments, or None if there
    /// is no overload for `arg_count` arguments
    pub fn get_polyadic_overload(&self, arg_count: usize) -> Option<&Function<'source>> {
        self.polyadic_overloads.get(&arg_count)
    }

    /// Gets the variadic overload, or None if there is no variadic overload
    pub fn get_variadic_overload(&self) -> Option<&Function<'source>> {
        self.variadic_overload.as_ref()
    }

    /// Adds a function overload
    ///
    /// # Panics
    /// - If an overload with the same arity already exists
    pub fn add_overload(&mut self, func: Function<'source>) {
        match &func.parameters {
            FunctionParameters::Variadic { parameter_name: _ } => {
                match self.get_variadic_overload() {
                    Some(_) => panic!("duplicate variadic function definition"),
                    None => self.variadic_overload = Some(func),
                };
            }
            FunctionParameters::Polyadic { parameters } => {
                let arity = parameters.len();
                match self.get_polyadic_overload(arity) {
                    Some(_) => panic!("duplicate polyadic function definition"),
                    None => self.polyadic_overloads.insert(arity, func),
                };
            }
        }
    }
}

impl Default for FunctionGroup<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// A non-builtin icelang function
#[derive(Debug, Clone)]
pub struct Function<'source> {
    parameters: FunctionParameters,
    body: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> Function<'source> {
    /// Constructs a new Function with the given parameters and body
    pub fn new(
        parameters: FunctionParameters,
        body: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            parameters,
            body,
            pos,
        }
    }

    /// Gets the body of the function
    pub fn body(&self) -> &Vec<AstNode<'source>> {
        &self.body
    }

    /// Gets the position of the function in the source code
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}
