use std::fmt::{Debug, Display};

use super::ast_node_format::format_as_node;

/// Represents parameters to an icelang function
#[derive(Debug, PartialEq, Eq)]
pub enum FunctionParameters {
    /// A variadic function (one which accepts any number of arguments)
    Variadic {
        /// The identifier for the parameter list parameter
        parameter_name: String,
    },
    /// A "normal" fixed-arity function (one which only accepts a fixed number
    /// of arguments)
    FixedArity {
        /// The parameter identifiers
        parameters: Vec<String>,
    },
}

impl Display for FunctionParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionParameters::Variadic {
                parameter_name: list_name,
            } => write!(f, "[{list_name}]"),
            FunctionParameters::FixedArity { parameters } => write!(f, "{}", parameters.join(", ")),
        }
    }
}

/// Represents a node in an abstract syntax tree (AST)
#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    /// An empty AstNode
    Empty,

    /// A list of multiple statements
    Statements {
        /// The list of statements, in order
        statements: Vec<AstNode>,
    },

    /// A function declaration
    FunctionDeclaration {
        /// The name of the function
        name: String,

        /// The parameter list of the function
        parameters: FunctionParameters,

        /// The body of the function
        body: Box<AstNode>,
    },
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AstNode::Empty => format_as_node("[Empty]", vec![]),
                AstNode::Statements { statements } => format_as_node(
                    "[Statements]",
                    statements.iter().map(AstNode::to_string).collect()
                ),
                AstNode::FunctionDeclaration {
                    name,
                    parameters,
                    body,
                } => format_as_node(
                    &format!("[Function Declaration] fn {name}({parameters})"),
                    vec![body.to_string()]
                ),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_parameters_display_empty() {
        assert_eq!(
            FunctionParameters::FixedArity { parameters: vec![] }.to_string(),
            ""
        );
    }

    #[test]
    fn test_function_parameters_display_unary() {
        assert_eq!(
            FunctionParameters::FixedArity {
                parameters: vec!["num".to_string()]
            }
            .to_string(),
            "num"
        );
    }

    #[test]
    fn test_function_parameters_display_binary() {
        assert_eq!(
            FunctionParameters::FixedArity {
                parameters: vec!["name".to_string(), "age".to_string()]
            }
            .to_string(),
            "name, age"
        );
    }

    #[test]
    fn test_function_parameters_display_septenary() {
        assert_eq!(
            FunctionParameters::FixedArity {
                parameters: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                    "f".to_string(),
                    "g".to_string()
                ]
            }
            .to_string(),
            "a, b, c, d, e, f, g"
        );
    }

    #[test]
    fn test_function_parameters_display_variadic() {
        assert_eq!(
            FunctionParameters::Variadic {
                parameter_name: "my_list".to_string()
            }
            .to_string(),
            "[my_list]"
        );
    }

    #[test]
    fn test_ast_node_display_empty() {
        let node = AstNode::Empty;

        assert_eq!(node.to_string(), "● [Empty]");
    }

    #[test]
    fn test_ast_node_display_statements() {
        // TODO make these not all the same once we have more AstNode kinds
        let node = AstNode::Statements {
            statements: vec![AstNode::Empty, AstNode::Empty, AstNode::Empty],
        };

        assert_eq!(
            node.to_string(),
            "\
● [Statements]
├─● [Empty]
├─● [Empty]
└─● [Empty]"
        );
    }

    #[test]
    fn test_ast_node_display_function_declaration_nullary() {
        // This was my 69th unit test :)
        // TODO make this return the funny number once we have expressions
        let body = Box::new(AstNode::Statements {
            statements: vec![AstNode::Empty],
        });
        let parameters = FunctionParameters::FixedArity { parameters: vec![] };
        let node = AstNode::FunctionDeclaration {
            name: "get_funny_number".to_string(),
            parameters,
            body,
        };

        assert_eq!(
            node.to_string(),
            "\
● [Function Declaration] fn get_funny_number()
└─● [Statements]
  └─● [Empty]"
        );
    }

    #[test]
    fn test_ast_node_display_function_declaration_unary() {
        // TODO make these not all the same once we have more AstNode kinds
        let body = Box::new(AstNode::Statements {
            statements: vec![AstNode::Empty, AstNode::Empty, AstNode::Empty],
        });
        let parameters = FunctionParameters::FixedArity {
            parameters: vec!["num".to_string()],
        };
        let node = AstNode::FunctionDeclaration {
            name: "square".to_string(),
            parameters,
            body,
        };

        assert_eq!(
            node.to_string(),
            "\
● [Function Declaration] fn square(num)
└─● [Statements]
  ├─● [Empty]
  ├─● [Empty]
  └─● [Empty]"
        );
    }

    #[test]
    fn test_ast_node_display_function_declaration_binary() {
        // TODO make these not all the same once we have more AstNode kinds
        let body = Box::new(AstNode::Statements {
            statements: vec![AstNode::Empty, AstNode::Empty, AstNode::Empty],
        });
        let parameters = FunctionParameters::FixedArity {
            parameters: vec!["width".to_string(), "height".to_string()],
        };
        let node = AstNode::FunctionDeclaration {
            name: "calculate_area".to_string(),
            parameters,
            body,
        };

        assert_eq!(
            node.to_string(),
            "\
● [Function Declaration] fn calculate_area(width, height)
└─● [Statements]
  ├─● [Empty]
  ├─● [Empty]
  └─● [Empty]"
        );
    }

    #[test]
    fn test_ast_node_display_function_declaration_variadic() {
        // TODO make these not all the same once we have more AstNode kinds
        let body = Box::new(AstNode::Statements {
            statements: vec![AstNode::Empty, AstNode::Empty, AstNode::Empty],
        });
        let parameters = FunctionParameters::Variadic {
            parameter_name: "numbers".to_string(),
        };
        let node = AstNode::FunctionDeclaration {
            name: "sum".to_string(),
            parameters,
            body,
        };

        assert_eq!(
            node.to_string(),
            "\
● [Function Declaration] fn sum([numbers])
└─● [Statements]
  ├─● [Empty]
  ├─● [Empty]
  └─● [Empty]"
        );
    }
}
