//! Contains code related to parsing (converting tokens to an abstract syntax tree (AST))

use std::collections::VecDeque;

use crate::{
    ast::{AstNode, FunctionParameters},
    error::ParseError,
    keyword::Keyword,
    source_range::SourceRange,
    token::Token,
};

/// Simplifies the given AstNode recursively
fn simplify_node(node: AstNode) -> AstNode {
    match node {
        AstNode::Empty => node,
        AstNode::Statements { mut statements } => {
            // Simplify each statement, removing any empty statements
            statements = statements
                .into_iter()
                .filter_map(|node| match node {
                    AstNode::Empty => None,
                    node => Some(simplify_node(node)),
                })
                .collect();

            // An empty list of statements can be simplified to an empty AstNode
            if statements.is_empty() {
                AstNode::Empty
            }
            // A list of one statement can be simplified to just the statement itself
            else if statements.len() == 1 {
                statements.swap_remove(0)
            }
            // No simplifying if the list has two or more statements
            else {
                AstNode::Statements { statements }
            }
        }
        AstNode::FunctionDeclaration {
            name,
            parameters,
            body,
        } => AstNode::FunctionDeclaration {
            name,
            parameters,
            body: Box::new(simplify_node(*body)),
        },
    }
}

/// Parses a function declaration's parameters from a token stream
fn parse_function_declaration_parameters<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
    start_pos: &SourceRange<'source>,
) -> Result<FunctionParameters, ParseError<'source>> {
    match token_stream.front() {
        // Variadic function
        Some(Token::Punctuator(token)) if token.punctuator() == "[" => {
            // Consume the "["
            token_stream.pop_front();

            // Read the parameter identifier
            let parameter_name = match token_stream.pop_front() {
                Some(Token::Ident(token)) => token.ident(),
                Some(token) => {
                    return Err(ParseError::new_unexpected_token(
                        "expected function parameter name".to_string(),
                        token.pos().clone(),
                    ));
                }
                None => {
                    return Err(ParseError::new_unexpected_eof(
                        "incomplete function declaration".to_string(),
                        start_pos.extended_to_end(),
                    ));
                }
            };

            // Expect a closing bracket
            match token_stream.pop_front() {
                Some(Token::Punctuator(token)) if token.punctuator() == "]" => {}
                Some(token) => {
                    return Err(ParseError::new_unexpected_token(
                        "expected closing bracket in function parameters".to_string(),
                        token.pos().clone(),
                    ));
                }
                None => {
                    return Err(ParseError::new_unexpected_eof(
                        "incomplete function declaration".to_string(),
                        start_pos.extended_to_end(),
                    ));
                }
            };

            Ok(FunctionParameters::Variadic {
                parameter_name: parameter_name.to_string(),
            })
        }

        // Nullary (zero parameter) function
        Some(Token::Punctuator(token)) if token.punctuator() == ")" => {
            Ok(FunctionParameters::FixedArity { parameters: vec![] })
        }

        // One-or-more-ary function (technically, multiary means 2 or more)
        Some(Token::Ident(first_parameter_name_token)) => {
            // Read the first parameter
            let mut parameters = vec![first_parameter_name_token.ident().to_string()];
            token_stream.pop_front();

            // Read any subsequent parameters
            loop {
                match token_stream.front() {
                    Some(Token::Punctuator(token)) if token.punctuator() == "," => {
                        // Consume the ","
                        token_stream.pop_front();

                        // Read the next parameter name
                        match token_stream.front() {
                            Some(Token::Ident(next_parameter_token)) => {
                                parameters.push(next_parameter_token.ident().to_string());
                            }
                            // If this was the optional comma after the last
                            // parameter, we're done
                            Some(Token::Punctuator(closing_paren_token))
                                if closing_paren_token.punctuator() == ")" =>
                            {
                                break
                            }
                            Some(token) => {
                                return Err(ParseError::new_unexpected_token(
                                    "invalid function parameter".to_string(),
                                    token.pos().clone(),
                                ));
                            }
                            None => {
                                return Err(ParseError::new_unexpected_eof(
                                    "incomplete function declaration".to_string(),
                                    start_pos.extended_to_end(),
                                ));
                            }
                        };
                    }
                    _ => break,
                }
            }

            Ok(FunctionParameters::FixedArity { parameters })
        }

        // Invalid arguments
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected parameter list in function declaration".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete function declaration".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    }
}

/// Parses a function declaration statement from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a function declaration
/// statement
fn parse_function_declaration<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    // Expect a "fn" token
    let start_pos = match token_stream.pop_front() {
        Some(Token::Keyword(token)) if token.keyword() == Keyword::Fn => token.pos(),
        _ => panic!("invalid function declaration"),
    };

    // Read the function name
    let function_name = match token_stream.pop_front() {
        Some(Token::Ident(token)) => token.ident(),
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected function name".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete function declaration".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    };

    // Expect an opening parenthesis
    match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == "(" => {}
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected opening parenthesis in function declaration".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete function declaration".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    };

    // Parse function parameters
    let parameters = parse_function_declaration_parameters(token_stream, start_pos)?;

    // Expect a closing parenthesis
    match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == ")" => {}
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected closing parenthesis in function declaration".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete function declaration".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    };

    // Expect an opening curly brace
    match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == "{" => {}
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected opening curly brace in function declaration".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete function declaration".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    };

    // Parse function body
    let function_body = parse_statements(token_stream)?;

    // Expect a closing curly brace
    match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == "}" => {}
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected closing curly brace in function declaration".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete function declaration".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    };

    Ok(AstNode::FunctionDeclaration {
        name: function_name.to_string(),
        parameters,
        body: Box::new(function_body),
    })
}

/// Parses a variable declaration statement from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a variable declaration statement
fn parse_variable_declaration<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses an if-else statement from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with an if-else statement
fn parse_if_else_statement<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a simple loop from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a simple loop
fn parse_simple_loop<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a while loop from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a while loop
fn parse_while_loop<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a for loop from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a for loop
fn parse_for_loop<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a match statement from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a match statement
fn parse_match_statement<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses an expression from a token stream
fn parse_expression<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses exactly one statement from a token stream
fn parse_statement<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    match token_stream.front() {
        // Empty statement (EOF)
        None => Ok(AstNode::Empty),

        // Empty statement (ended with a semicolon)
        Some(Token::Punctuator(token)) if token.punctuator() == ";" => Ok(AstNode::Empty),

        // Function declaration
        Some(Token::Keyword(token)) if token.keyword() == Keyword::Fn => {
            parse_function_declaration(token_stream)
        }

        // Variable declaration
        Some(Token::Keyword(token)) if token.keyword() == Keyword::Let => {
            parse_variable_declaration(token_stream)
        }

        // If-else statement
        Some(Token::Keyword(token)) if token.keyword() == Keyword::If => {
            parse_if_else_statement(token_stream)
        }

        // Simple loop
        Some(Token::Keyword(token)) if token.keyword() == Keyword::Loop => {
            parse_simple_loop(token_stream)
        }

        // While loop
        Some(Token::Keyword(token)) if token.keyword() == Keyword::While => {
            parse_while_loop(token_stream)
        }

        // For loop
        Some(Token::Keyword(token)) if token.keyword() == Keyword::For => {
            parse_for_loop(token_stream)
        }

        // Match statement
        Some(Token::Keyword(token)) if token.keyword() == Keyword::Match => {
            parse_match_statement(token_stream)
        }

        // Break statement
        Some(Token::Keyword(token)) if token.keyword() == Keyword::Break => {
            todo!()
        }

        // Continue statement
        Some(Token::Keyword(token)) if token.keyword() == Keyword::Continue => {
            todo!()
        }

        // Return statement
        Some(Token::Keyword(token)) if token.keyword() == Keyword::Return => {
            todo!()
        }

        // Otherwise, assume it's an expression
        _ => parse_expression(token_stream),
    }
}

/// Parses a single group of multiple statements from a token stream
fn parse_statements<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode, ParseError<'source>> {
    let mut statements = Vec::new();

    statements.push(parse_statement(token_stream)?);
    while matches!(token_stream.front(), Some(&Token::Punctuator(token)) if token.punctuator() == ";")
    {
        // Consume the ';'
        token_stream.pop_front();

        // Parse the next statement
        statements.push(parse_statement(token_stream)?);
    }

    Ok(AstNode::Statements { statements })
}

/// Reads a list of tokens and produces an abstract syntax tree
pub fn parse<'token, 'source: 'token>(
    tokens: impl Into<VecDeque<&'token Token<'source>>>,
) -> Result<AstNode, ParseError<'source>> {
    // Convert `tokens` to a VecDeque, since we're going to need to pop from the front often
    let mut tokens: VecDeque<&Token> = tokens.into();

    // A program's AST is just a bunch of statements
    let root = parse_statements(&mut tokens)?;

    // Ensure there are no remaining tokens
    if !tokens.is_empty() {
        return Err(ParseError::UnexpectedToken {
            why: "no tokens allowed after last statement".to_string(),
            pos: tokens[0].pos().clone(),
        });
    }

    // Return the AST (simplified first)
    Ok(simplify_node(root))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let tokens = vec![];

        let ast = parse(tokens).unwrap();

        assert_eq!(ast, AstNode::Empty);
    }

    // TODO much more extensive unit testing
}
