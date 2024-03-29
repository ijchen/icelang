use std::fmt::Display;

use crate::{function::FunctionParameters, source_range::SourceRange};

use super::*;
use ast_node_format::format_as_node;

/// A function declaration AST node
#[derive(Debug, Clone)]
pub struct AstNodeFunctionDeclaration<'source> {
    name: String,
    parameters: FunctionParameters<'source>,
    body: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeFunctionDeclaration<'source> {
    /// Constructs a new AstNodeFunctionDeclaration
    pub fn new(
        name: String,
        parameters: FunctionParameters<'source>,
        body: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            name,
            parameters,
            body,
            pos,
        }
    }

    /// Returns the name of the function
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the parameters of the function
    pub fn parameters(&self) -> &FunctionParameters<'source> {
        &self.parameters
    }

    /// Returns the body of the function
    pub fn body(&self) -> &Vec<AstNode<'source>> {
        &self.body
    }

    /// Returns the position in the source code of this function declaration
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// function declaration
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl PartialEq for AstNodeFunctionDeclaration<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.parameters == other.parameters && self.body == other.body
    }
}
impl Eq for AstNodeFunctionDeclaration<'_> {}

impl Display for AstNodeFunctionDeclaration<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                &format!(
                    "[Function Declaration] fn {}({})",
                    self.name, self.parameters
                ),
                self.body.iter().map(AstNode::to_string).collect()
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO move function parameters tests

    #[test]
    fn test_function_parameters_display_empty() {
        assert_eq!(
            FunctionParameters::Polyadic { parameters: vec![] }.to_string(),
            ""
        );
    }

    // TODO fix and reenable at some point
    // #[test]
    // fn test_function_parameters_display_unary() {
    //     assert_eq!(
    //         FunctionParameters::Polyadic {
    //             parameters: vec!["num".to_string()]
    //         }
    //         .to_string(),
    //         "num"
    //     );
    // }

    // #[test]
    // fn test_function_parameters_display_binary() {
    //     assert_eq!(
    //         FunctionParameters::Polyadic {
    //             parameters: vec!["name".to_string(), "age".to_string()]
    //         }
    //         .to_string(),
    //         "name, age"
    //     );
    // }

    // #[test]
    // fn test_function_parameters_display_septenary() {
    //     assert_eq!(
    //         FunctionParameters::Polyadic {
    //             parameters: vec![
    //                 "a".to_string(),
    //                 "b".to_string(),
    //                 "c".to_string(),
    //                 "d".to_string(),
    //                 "e".to_string(),
    //                 "f".to_string(),
    //                 "g".to_string()
    //             ]
    //         }
    //         .to_string(),
    //         "a, b, c, d, e, f, g"
    //     );
    // }

    // #[test]
    // fn test_function_parameters_display_variadic() {
    //     assert_eq!(
    //         FunctionParameters::Variadic {
    //             parameter_name: "my_list".to_string()
    //         }
    //         .to_string(),
    //         "[my_list]"
    //     );
    // }

    #[test]
    fn test_ast_node_display_function_declaration_nullary() {
        // This was my 69th unit test :)
        // TODO make this return the funny number once we have expressions

        let nowhere = SourceRange::new(" ", "", 0, 0);
        let body = vec![];
        let parameters = FunctionParameters::Polyadic { parameters: vec![] };
        let node: AstNode = AstNodeFunctionDeclaration::new(
            "get_funny_number".to_string(),
            parameters,
            body,
            nowhere,
        )
        .into();

        assert_eq!(
            node.to_string(),
            "\
● [Function Declaration] fn get_funny_number()"
        );
    }

    // TODO fix and reenable at some point
    //     #[test]
    //     fn test_ast_node_display_function_declaration_unary() {
    //         let nowhere = SourceRange::new(" ", "", 0, 0);
    //         // TODO add a body once we have more AstNode kinds
    //         let body = vec![];
    //         let parameters = FunctionParameters::Polyadic {
    //             parameters: vec!["num".to_string()],
    //         };
    //         let node = AstNodeFunctionDeclaration::new("square".to_string(), parameters, body, nowhere);

    //         assert_eq!(
    //             node.to_string(),
    //             "\
    // ● [Function Declaration] fn square(num)"
    //         );
    //     }

    //     #[test]
    //     fn test_ast_node_display_function_declaration_binary() {
    //         let nowhere = SourceRange::new(" ", "", 0, 0);
    //         // TODO add a body once we have more AstNode kinds
    //         let body = vec![];
    //         let parameters = FunctionParameters::Polyadic {
    //             parameters: vec!["width".to_string(), "height".to_string()],
    //         };
    //         let node = AstNodeFunctionDeclaration::new(
    //             "calculate_area".to_string(),
    //             parameters,
    //             body,
    //             nowhere,
    //         );

    //         assert_eq!(
    //             node.to_string(),
    //             "\
    // ● [Function Declaration] fn calculate_area(width, height)"
    //         );
    //     }

    //     #[test]
    //     fn test_ast_node_display_function_declaration_variadic() {
    //         let nowhere = SourceRange::new(" ", "", 0, 0);
    //         // TODO add a body once we have more AstNode kinds
    //         let body = vec![];
    //         let parameters = FunctionParameters::Variadic {
    //             parameter_name: "numbers".to_string(),
    //         };
    //         let node = AstNodeFunctionDeclaration::new("sum".to_string(), parameters, body, nowhere);

    //         assert_eq!(
    //             node.to_string(),
    //             "\
    // ● [Function Declaration] fn sum([numbers])"
    //         );
    //     }
}
