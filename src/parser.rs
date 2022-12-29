//! Contains code related to parsing (converting tokens to an abstract syntax tree (AST))

use std::collections::VecDeque;

use crate::{ast::AstNode, error::ParseError, token::Token};

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
    }
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

        _ => todo!(),
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

    // Ensure we used up every token
    if !tokens.is_empty() {
        return Err(ParseError::UnexpectedToken {
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
}
