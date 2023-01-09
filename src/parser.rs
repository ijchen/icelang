//! Contains code related to parsing (converting tokens to an abstract syntax tree (AST))

use std::collections::VecDeque;

use crate::{
    ast::{
        AssignmentKind, Ast, AstNode, AstNodeAssignment, AstNodeBinaryOperation, AstNodeComparison,
        AstNodeFunctionDeclaration, AstNodeInlineConditional, AstNodeJumpStatement, AstNodeLiteral,
        AstNodeTypeCast, AstNodeUnaryOperation, AstNodeUsageSuffix, AstNodeVariableAccess,
        BinaryOperationKind, ComparisonKind, FunctionParameters, JumpStatementKind,
        UnaryOperationKind, UsageSuffix, UsageSuffixComputedMemberAccess,
        UsageSuffixDotMemberAccess, UsageSuffixFunctionCall,
    },
    error::ParseError,
    keyword::Keyword,
    source_range::SourceRange,
    token::Token,
};

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
                        "expected closing square bracket in function parameters".to_string(),
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
                                token_stream.pop_front();
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
/// - If the token stream doesn't immediately start with a "fn" keyword token
fn parse_function_declaration<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
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

    // Parse function body
    let (body, body_pos) = parse_code_block(token_stream)?;

    Ok(AstNodeFunctionDeclaration::new(
        function_name.to_string(),
        parameters,
        body,
        start_pos.extended_to(&body_pos),
    )
    .into())
}

/// Parses a variable declaration statement from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a variable declaration statement
fn parse_variable_declaration<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses an if-else statement from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with an if-else statement
fn parse_if_else_statement<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a simple loop from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a simple loop
fn parse_simple_loop<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a while loop from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a while loop
fn parse_while_loop<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a for loop from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a for loop
fn parse_for_loop<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a match statement from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a match statement
fn parse_match_statement<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    let _ = token_stream;
    todo!()
}

/// Parses a parenthesized expression from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with an opening parenthesis
fn parse_parenthesized_expression<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    // Expect an opening parenthesis
    let start_pos = match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == "(" => token.pos(),
        _ => panic!("invalid function declaration"),
    };

    // Parse the expression
    let mut expression = parse_expression(token_stream)?;

    // Expect a closing parenthesis
    let end_pos = match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == ")" => token.pos(),
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected closing parenthesis in parenthesized expression".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete parenthesized expression".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    };

    // Update the position of the expression
    let new_pos = start_pos.extended_to(end_pos);
    *expression.pos_mut() = new_pos;

    Ok(expression)
}

/// Parses a type cast expression from a token stream
///
/// # Panics
/// - If the token stream doesn't immediately start with a valid type keyword
fn parse_type_cast_expression<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    // Expect a type keyword
    let (start_pos, new_type) = match token_stream.pop_front() {
        Some(Token::Keyword(token)) if token.keyword().can_be_type() => {
            (token.pos(), token.keyword().icelang_type().unwrap())
        }
        _ => panic!("invalid type cast expression"),
    };

    // Expect an opening parenthesis
    match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == "(" => token.pos(),
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected closing parenthesis in type cast expression".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete type cast expression".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    };

    // Parse the expression
    let expression = parse_expression(token_stream)?;

    // Expect a closing parenthesis
    let end_pos = match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == ")" => token.pos(),
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected closing parenthesis in type cast expression".to_string(),
                token.pos().clone(),
            ));
        }
        None => {
            return Err(ParseError::new_unexpected_eof(
                "incomplete type cast expression".to_string(),
                start_pos.extended_to_end(),
            ));
        }
    };

    Ok(AstNodeTypeCast::new(expression, new_type, start_pos.extended_to(end_pos)).into())
}

/// Parses an atomic expression from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_atomic<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    assert!(!token_stream.is_empty());

    match token_stream.front().unwrap() {
        // Parenthesized expression
        Token::Punctuator(token) if token.punctuator() == "(" => {
            parse_parenthesized_expression(token_stream)
        }

        // Type cast expression
        Token::Keyword(token) if token.keyword().can_be_type() => {
            parse_type_cast_expression(token_stream)
        }

        // Literal
        Token::Literal(token) => {
            // Consume the literal token
            token_stream.pop_front();

            Ok(AstNodeLiteral::new(
                token.raw().to_string(),
                token.icelang_type(),
                token.pos().clone(),
            )
            .into())
        }

        // Identifier
        Token::Ident(token) => {
            // Consume the identifier token
            token_stream.pop_front();

            Ok(AstNodeVariableAccess::new(token.ident().to_string(), token.pos().clone()).into())
        }

        // Anything else is a syntax error
        token => Err(ParseError::UnexpectedToken {
            why: "expected atomic expression".to_string(),
            pos: token.pos().clone(),
        }),
    }
}

/// Parses a usage suffix expression from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_expr_usage_suffix<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    // Parse the root atomic expression
    let root = parse_atomic(token_stream)?;

    let mut suffixes: Vec<UsageSuffix> = Vec::new();

    // Parse any usage suffixes
    loop {
        match token_stream.front() {
            // Dot member access
            Some(Token::Punctuator(token)) if token.punctuator() == "." => {
                let start_pos = token.pos();

                // Consume the "."
                token_stream.pop_front();

                // Expect an identifier
                let (ident, ident_pos) = match token_stream.pop_front() {
                    Some(Token::Ident(token)) => (token.ident(), token.pos()),
                    Some(token) => {
                        return Err(ParseError::new_unexpected_token(
                            "expected identifier in dot member access suffix".to_string(),
                            token.pos().clone(),
                        ));
                    }
                    None => {
                        return Err(ParseError::new_unexpected_eof(
                            "incomplete dot member access suffix".to_string(),
                            root.pos().extended_to_end(),
                        ));
                    }
                };

                // Append the new UsageSuffix
                suffixes.push(
                    UsageSuffixDotMemberAccess::new(
                        ident.to_string(),
                        start_pos.extended_to(ident_pos),
                    )
                    .into(),
                );
            }

            // Computed (square bracket) member access
            Some(Token::Punctuator(token)) if token.punctuator() == "[" => {
                let start_pos = token.pos();

                // Consume the "["
                token_stream.pop_front();

                // Ensure the token stream isn't empty
                if token_stream.is_empty() {
                    return Err(ParseError::new_unexpected_eof(
                        "incomplete computed member access suffix".to_string(),
                        root.pos().extended_to_end(),
                    ));
                };

                // Parse the expression inside the brackets
                let body = parse_expression(token_stream)?;

                // Expect a "]"
                let end_pos = match token_stream.pop_front() {
                    Some(Token::Punctuator(token)) if token.punctuator() == "]" => token.pos(),
                    Some(token) => {
                        return Err(ParseError::new_unexpected_token(
                            "expected closing square bracket in computed member access suffix"
                                .to_string(),
                            token.pos().clone(),
                        ));
                    }
                    None => {
                        return Err(ParseError::new_unexpected_eof(
                            "incomplete computed member access suffix".to_string(),
                            start_pos.extended_to_end(),
                        ));
                    }
                };

                // Append the new UsageSuffix
                suffixes.push(
                    UsageSuffixComputedMemberAccess::new(body, start_pos.extended_to(end_pos))
                        .into(),
                );
            }

            // Function call
            Some(Token::Punctuator(token)) if token.punctuator() == "(" => {
                let start_pos = token.pos();

                // Consume the "("
                token_stream.pop_front();

                // Parse the function arguments
                let mut arguments = Vec::new();
                let end_pos = match token_stream.front() {
                    // No arguments
                    Some(Token::Punctuator(token)) if token.punctuator() == ")" => {
                        // Consume the ")"
                        token_stream.pop_front();

                        token.pos()
                    }

                    // One or more arguments
                    Some(_) => {
                        // Parse the first argument
                        arguments.push(parse_expression(token_stream)?);

                        // Parse any subsequent parameters
                        loop {
                            match token_stream.front() {
                                Some(Token::Punctuator(token)) if token.punctuator() == "," => {
                                    // Consume the ","
                                    token_stream.pop_front();

                                    // Parse the next argument
                                    match token_stream.front() {
                                        // If this was the optional comma after
                                        // the last argument, we're done
                                        Some(Token::Punctuator(closing_paren_token))
                                            if closing_paren_token.punctuator() == ")" =>
                                        {
                                            // Consume the ")"
                                            token_stream.pop_front();

                                            break closing_paren_token.pos();
                                        }
                                        Some(_) => {
                                            // Parse the next argument
                                            arguments.push(parse_expression(token_stream)?);
                                        }
                                        None => {
                                            return Err(ParseError::new_unexpected_eof(
                                                "incomplete function declaration".to_string(),
                                                start_pos.extended_to_end(),
                                            ));
                                        }
                                    };
                                }
                                Some(Token::Punctuator(token)) if token.punctuator() == ")" => {
                                    // Consume the ")"
                                    token_stream.pop_front();

                                    break token.pos();
                                }
                                Some(token) => {
                                    return Err(ParseError::new_unexpected_token(
                                        "unexpected token in function arguments".to_string(),
                                        token.pos().clone(),
                                    ));
                                }
                                None => {
                                    return Err(ParseError::UnexpectedEOF {
                                        why: "expected closing parenthesis in function arguments"
                                            .to_string(),
                                        pos: start_pos.extended_to_end(),
                                    });
                                }
                            }
                        }
                    }

                    // EOF (without a closing parenthesis)
                    None => {
                        return Err(ParseError::UnexpectedEOF {
                            why: "expected closing parenthesis in function arguments".to_string(),
                            pos: start_pos.extended_to_end(),
                        });
                    }
                };

                // Append the new UsageSuffix
                suffixes.push(
                    UsageSuffixFunctionCall::new(arguments, start_pos.extended_to(end_pos)).into(),
                );
            }

            // Anything else is not a usage suffix
            _ => break,
        }
    }

    // Return a new AstNodeUsageSuffix if we have suffixes, otherwise just
    // return the root
    Ok(if suffixes.is_empty() {
        root
    } else {
        AstNodeUsageSuffix::new(root, suffixes).into()
    })
}

/// Parses an exponentiation expression from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_expr_exponentiation<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    assert!(!token_stream.is_empty());

    // Parse the first (and possibly only) operand
    let mut operands = vec![parse_expr_usage_suffix(token_stream)?];

    // Parse any additional operations
    loop {
        match token_stream.front() {
            Some(Token::Punctuator(token)) if token.punctuator() == "**" => {
                let start_pos = token.pos();

                // Consume the "**"
                token_stream.pop_front();

                // Parse the rhs
                operands.push(match token_stream.front() {
                    Some(_) => parse_expr_unary_prefix(token_stream)?,
                    None => {
                        return Err(ParseError::UnexpectedEOF {
                            why: "expected right-hand side of binary operation".to_string(),
                            pos: start_pos.extended_to_end(),
                        });
                    }
                });
            }
            _ => break,
        }
    }

    // Construct the binary operation node(s) from the operands
    // Exponentiation is right-associative, so build from the right
    let mut root = operands.pop().unwrap();
    while let Some(lhs) = operands.pop() {
        root = AstNodeBinaryOperation::new(lhs, root, BinaryOperationKind::Exponentiation).into();
    }

    Ok(root)
}

/// Parses a unary prefix expression from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_expr_unary_prefix<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    assert!(!token_stream.is_empty());

    match token_stream.front().unwrap() {
        Token::Punctuator(token)
            if token.punctuator() == "!"
                || token.punctuator() == "+"
                || token.punctuator() == "-" =>
        {
            // Consume the prefix
            token_stream.pop_front();

            // Ensure the token stream isn't empty
            if token_stream.is_empty() {
                return Err(ParseError::new_unexpected_eof(
                    "incomplete unary prefix operation".to_string(),
                    token.pos().extended_to_end(),
                ));
            };

            // Parse the operand
            let operand = parse_expr_unary_prefix(token_stream)?;

            // Return the new unary operation node
            let pos = token.pos().extended_to(operand.pos());
            let operation = match token.punctuator() {
                "!" => UnaryOperationKind::Not,
                "+" => UnaryOperationKind::Identity,
                "-" => UnaryOperationKind::Negation,
                _ => unreachable!(),
            };
            Ok(AstNodeUnaryOperation::new(operand, operation, pos).into())
        }
        _ => parse_expr_exponentiation(token_stream),
    }
}

macro_rules! left_associative_bin_op {
    ($name:ident, child: $child:ident, $msg:literal, {$($punc_str:literal => $op_kind:expr),+}) => {
        /// Parses
        #[doc = $msg]
        /// expression from a token stream
        ///
        /// # Panics
        /// - If the token stream is empty
        fn $name<'source>(
            token_stream: &mut VecDeque<&Token<'source>>,
        ) -> Result<AstNode<'source>, ParseError<'source>> {
            assert!(!token_stream.is_empty());

            // Parse the first (and possibly only) operand
            let mut root = $child(token_stream)?;

            // Parse any additional operations
            loop {
                match token_stream.front() {
                    Some(Token::Punctuator(token))
                        if $(token.punctuator() == $punc_str)||+ =>
                    {
                        let start_pos = token.pos();

                        // Consume the operator
                        token_stream.pop_front();

                        // Parse the rhs
                        let rhs = match token_stream.front() {
                            Some(_) => $child(token_stream)?,
                            None => {
                                return Err(ParseError::UnexpectedEOF {
                                    why: "expected right-hand side of binary operation".to_string(),
                                    pos: start_pos.extended_to_end(),
                                });
                            }
                        };

                        // Update the root
                        let operation = match token.punctuator() {
                            $($punc_str => $op_kind,)+
                            _ => unreachable!(),
                        };
                        root = AstNodeBinaryOperation::new(root, rhs, operation).into();
                    }
                    _ => break,
                }
            }

            Ok(root)
        }
    };
}
left_associative_bin_op!(
    parse_expr_multiplicative,
    child: parse_expr_unary_prefix,
    "a multiplicative",
    {
        "*" => BinaryOperationKind::Multiplication,
        "/" => BinaryOperationKind::Division,
        "%" => BinaryOperationKind::Modulo
    }
);
left_associative_bin_op!(
    parse_expr_additive,
    child: parse_expr_multiplicative,
    "an additive",
    {
        "+" => BinaryOperationKind::Addition,
        "-" => BinaryOperationKind::Subtraction
    }
);
left_associative_bin_op!(
    parse_expr_bitshift,
    child: parse_expr_additive,
    "a bitshift",
    {
        "<<" => BinaryOperationKind::ShiftLeft,
        ">>" => BinaryOperationKind::ShiftRight
    }
);
left_associative_bin_op!(
    parse_expr_bitwise_and,
    child: parse_expr_bitshift,
    "a bitwise and",
    {
        "&" => BinaryOperationKind::BitwiseAnd
    }
);
left_associative_bin_op!(
    parse_expr_bitwise_xor,
    child: parse_expr_bitwise_and,
    "a bitwise xor",
    {
        "^" => BinaryOperationKind::BitwiseXor
    }
);
left_associative_bin_op!(
    parse_expr_bitwise_or,
    child: parse_expr_bitwise_xor,
    "a bitwise or",
    {
        "|" => BinaryOperationKind::BitwiseOr
    }
);

/// Parses a comparison expression from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_expr_comparison<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    assert!(!token_stream.is_empty());

    // Parse the first operand
    let first = parse_expr_bitwise_or(token_stream)?;

    // Parse any additional comparisons
    let mut comparisons = Vec::new();
    loop {
        match token_stream.front() {
            Some(Token::Punctuator(token))
                if token.punctuator() == "=="
                    || token.punctuator() == "!="
                    || token.punctuator() == "<"
                    || token.punctuator() == ">"
                    || token.punctuator() == "<="
                    || token.punctuator() == ">=" =>
            {
                let start_pos = token.pos();

                // Consume the operator
                token_stream.pop_front();

                // Parse the rhs
                let rhs = match token_stream.front() {
                    Some(_) => parse_expr_bitwise_or(token_stream)?,
                    None => {
                        return Err(ParseError::UnexpectedEOF {
                            why: "expected right-hand side of comparison".to_string(),
                            pos: start_pos.extended_to_end(),
                        });
                    }
                };

                comparisons.push((
                    match token.punctuator() {
                        "==" => ComparisonKind::Equal,
                        "!=" => ComparisonKind::NotEqual,
                        "<" => ComparisonKind::LessThan,
                        ">" => ComparisonKind::GreaterThan,
                        "<=" => ComparisonKind::LessThanOrEqual,
                        ">=" => ComparisonKind::GreaterThanOrEqual,
                        _ => unreachable!(),
                    },
                    rhs,
                ));
            }
            _ => break,
        }
    }

    // If there were no additional comparisons, just return the first operand
    Ok(if comparisons.is_empty() {
        first
    } else {
        AstNodeComparison::new(first, comparisons).into()
    })
}

left_associative_bin_op!(
    parse_expr_logical_and,
    child: parse_expr_comparison,
    "a logical and",
    {
        "&&" => BinaryOperationKind::LogicalAnd
    }
);
left_associative_bin_op!(
    parse_expr_logical_or,
    child: parse_expr_logical_and,
    "a logical or",
    {
        "||" => BinaryOperationKind::LogicalOr
    }
);

/// Parses an inline conditional (often called a
/// ["ternary operator"](https://en.wikipedia.org/wiki/Ternary_conditional_operator))
/// expression from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_expr_inline_cond<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    assert!(!token_stream.is_empty());

    // Parse the condition
    let condition = parse_expr_logical_or(token_stream)?;

    // If this actually isn't an inline conditional, just return the "condition"
    if !matches!(
        token_stream.front(),
        Some(Token::Punctuator(token)) if token.punctuator() == "?"
    ) {
        return Ok(condition);
    }

    // Consume the "?"
    token_stream.pop_front();

    // Ensure the token stream isn't empty
    if token_stream.is_empty() {
        return Err(ParseError::new_unexpected_eof(
            "incomplete inline conditional expression".to_string(),
            condition.pos().extended_to_end(),
        ));
    };

    // Parse the truthy case
    let truthy_case = parse_expr_inline_cond(token_stream)?;

    // Expect a ":"
    match token_stream.pop_front() {
        Some(Token::Punctuator(token)) if token.punctuator() == ":" => {}
        Some(token) => {
            return Err(ParseError::new_unexpected_token(
                "expected colon in inline conditional expression".to_string(),
                token.pos().clone(),
            ))
        }
        None => {
            return Err(ParseError::UnexpectedEOF {
                why: "expected colon in inline conditional expression".to_string(),
                pos: condition.pos().extended_to_end(),
            });
        }
    }

    // Ensure the token stream isn't empty
    if token_stream.is_empty() {
        return Err(ParseError::new_unexpected_eof(
            "incomplete inline conditional expression".to_string(),
            condition.pos().extended_to_end(),
        ));
    };

    // Parse the falsey case
    let falsey_case = parse_expr_inline_cond(token_stream)?;

    // Construct and return the conditional node
    Ok(AstNodeInlineConditional::new(condition, truthy_case, falsey_case).into())
}

/// Parses an assignment expression from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_expr_assignment<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    assert!(!token_stream.is_empty());

    // Parse the left-hand side
    let lhs = parse_expr_inline_cond(token_stream)?;

    // If this actually isn't an assignment expression, just return the "lhs"
    let assignment_kind = match token_stream.front() {
        Some(Token::Punctuator(token)) if token.punctuator() == "=" => AssignmentKind::Normal,
        Some(Token::Punctuator(token)) if token.punctuator() == "+=" => AssignmentKind::Plus,
        Some(Token::Punctuator(token)) if token.punctuator() == "-=" => AssignmentKind::Minus,
        Some(Token::Punctuator(token)) if token.punctuator() == "*=" => AssignmentKind::Times,
        Some(Token::Punctuator(token)) if token.punctuator() == "/=" => AssignmentKind::Div,
        Some(Token::Punctuator(token)) if token.punctuator() == "%=" => AssignmentKind::Mod,
        Some(Token::Punctuator(token)) if token.punctuator() == "**=" => AssignmentKind::Exp,
        Some(Token::Punctuator(token)) if token.punctuator() == "<<=" => AssignmentKind::Shl,
        Some(Token::Punctuator(token)) if token.punctuator() == ">>=" => AssignmentKind::Shr,
        Some(Token::Punctuator(token)) if token.punctuator() == "&=" => AssignmentKind::BitAnd,
        Some(Token::Punctuator(token)) if token.punctuator() == "^=" => AssignmentKind::BitXor,
        Some(Token::Punctuator(token)) if token.punctuator() == "|=" => AssignmentKind::BitOr,
        Some(Token::Punctuator(token)) if token.punctuator() == "&&=" => AssignmentKind::LogAnd,
        Some(Token::Punctuator(token)) if token.punctuator() == "||=" => AssignmentKind::LogOr,
        _ => {
            return Ok(lhs);
        }
    };

    // Consume the assignment operator
    token_stream.pop_front();

    // Ensure the token stream isn't empty
    if token_stream.is_empty() {
        return Err(ParseError::new_unexpected_eof(
            "incomplete assignment expression".to_string(),
            lhs.pos().extended_to_end(),
        ));
    };

    // Parse the right-hand side
    let rhs = parse_expr_assignment(token_stream)?;

    // Construct and return the assignment node
    Ok(AstNodeAssignment::new(lhs, rhs, assignment_kind).into())
}

/// Parses an expression from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_expression<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    assert!(!token_stream.is_empty());

    parse_expr_assignment(token_stream)
}

/// Parses exactly one statement from a token stream
///
/// # Panics
/// - If the token stream is empty
fn parse_statement<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<AstNode<'source>, ParseError<'source>> {
    assert!(!token_stream.is_empty());

    match token_stream.front().unwrap() {
        // Function declaration
        Token::Keyword(token) if token.keyword() == Keyword::Fn => {
            parse_function_declaration(token_stream)
        }

        // Variable declaration
        Token::Keyword(token) if token.keyword() == Keyword::Let => {
            parse_variable_declaration(token_stream)
        }

        // If-else statement
        Token::Keyword(token) if token.keyword() == Keyword::If => {
            parse_if_else_statement(token_stream)
        }

        // Simple loop
        Token::Keyword(token) if token.keyword() == Keyword::Loop => {
            parse_simple_loop(token_stream)
        }

        // While loop
        Token::Keyword(token) if token.keyword() == Keyword::While => {
            parse_while_loop(token_stream)
        }

        // For loop
        Token::Keyword(token) if token.keyword() == Keyword::For => parse_for_loop(token_stream),

        // Match statement
        Token::Keyword(token) if token.keyword() == Keyword::Match => {
            parse_match_statement(token_stream)
        }

        // Break statement
        Token::Keyword(token) if token.keyword() == Keyword::Break => {
            let pos = token.pos().clone();

            // Consume the "break" keyword
            token_stream.pop_front();

            Ok(AstNodeJumpStatement::new(None, JumpStatementKind::Break, pos).into())
        }

        // Continue statement
        Token::Keyword(token) if token.keyword() == Keyword::Continue => {
            let pos = token.pos().clone();

            // Consume the "continue" keyword
            token_stream.pop_front();

            Ok(AstNodeJumpStatement::new(None, JumpStatementKind::Continue, pos).into())
        }

        // Return statement
        Token::Keyword(token) if token.keyword() == Keyword::Return => {
            let pos = token.pos().clone();

            // Consume the "return" keyword
            token_stream.pop_front();

            // Parse the expression following the return keyword
            let body = match token_stream.front() {
                Some(Token::Punctuator(token))
                    if token.punctuator() == ";" || token.punctuator() == "}" =>
                {
                    None
                }
                None => None,
                Some(_) => Some(parse_expression(token_stream)?),
            };

            Ok(AstNodeJumpStatement::new(body, JumpStatementKind::Return, pos).into())
        }

        // Otherwise, assume it's an expression
        _ => parse_expression(token_stream),
    }
}

/// Parses a single code block (which may contain many statements) from a token
/// stream
///
/// # Panics
/// - If the token stream is empty
fn parse_code_block<'source>(
    token_stream: &mut VecDeque<&Token<'source>>,
) -> Result<(Vec<AstNode<'source>>, SourceRange<'source>), ParseError<'source>> {
    assert!(!token_stream.is_empty());

    // Expect an opening curly brace
    let start_pos = match token_stream.pop_front().unwrap() {
        Token::Punctuator(token) if token.punctuator() == "{" => token.pos(),
        token => {
            return Err(ParseError::new_unexpected_token(
                "expected opening curly brace in code block".to_string(),
                token.pos().clone(),
            ));
        }
    };
    let pos;

    // Parse the statements in the code block
    let mut statements = Vec::new();
    loop {
        match token_stream.front() {
            Some(Token::Punctuator(token)) if token.punctuator() == "}" => {
                // Update pos
                pos = start_pos.extended_to(token.pos());

                // Consume the "}"
                token_stream.pop_front();

                // Code block is done
                break;
            }
            Some(_) => {
                // Parse the next statement
                statements.push(parse_statement(token_stream)?);

                match token_stream.front() {
                    Some(Token::Punctuator(token)) if token.punctuator() == "}" => {
                        // Update pos
                        pos = start_pos.extended_to(token.pos());

                        // Consume the "}"
                        token_stream.pop_front();

                        // Code block is done
                        break;
                    }

                    Some(Token::Punctuator(token)) if token.punctuator() == ";" => {
                        // Consume the ";"
                        token_stream.pop_front();

                        continue;
                    }

                    // The next token should only ever be a "}" or ";", so this
                    // is a syntax error
                    Some(token) => {
                        return Err(ParseError::new_unexpected_token(
                            "unexpected token in code block".to_string(),
                            token.pos().clone(),
                        ));
                    }

                    // If that's the end of the token stream, continue and we'll
                    // return a ParseError in the next loop iteration
                    None => continue,
                }
            }
            None => {
                return Err(ParseError::new_unexpected_eof(
                    "incomplete code block (missing closing curly brace)".to_string(),
                    start_pos.extended_to_end(),
                ));
            }
        }
    }

    Ok((statements, pos))
}

/// Reads a list of tokens and produces an abstract syntax tree
pub fn parse<'token, 'source: 'token>(
    tokens: impl Into<VecDeque<&'token Token<'source>>>,
) -> Result<Ast<'source>, ParseError<'source>> {
    // Convert `tokens` to a VecDeque, since we're going to need to pop from the
    // front often
    let mut token_stream: VecDeque<&Token> = tokens.into();

    // A program's AST is just a bunch of statements, so parse them
    let mut statements = Vec::new();
    while !token_stream.is_empty() {
        // Parse the next statement
        statements.push(parse_statement(&mut token_stream)?);

        match token_stream.front() {
            Some(Token::Punctuator(token)) if token.punctuator() == ";" => {
                // Consume the ";"
                token_stream.pop_front();

                continue;
            }

            // If that's the end of the token stream, we're done
            None => break,

            // The next token should only ever be a ";", so this is a syntax
            // error
            Some(token) => {
                return Err(ParseError::new_unexpected_token(
                    // Very generic error message for a very generic error
                    "unexpected token".to_string(),
                    token.pos().clone(),
                ));
            }
        }
    }

    // Ensure there are no remaining tokens
    assert!(token_stream.is_empty());

    // Return the AST
    Ok(Ast { statements })
}

#[cfg(test)]
mod tests {
    // TODO I'd rather that these unit tests don't rely on the lexer, but
    // it would be quite a hassle to write the tokens out by hand. In the
    // future, write something to use the lexer to output the rust code for
    // the vec![] literal, then manually verify and copy-paste them here.
    // use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn parse_empty() {
        let tokens = vec![];

        let ast = parse(tokens).unwrap();

        assert_eq!(ast, Ast { statements: vec![] });
    }

    // TODO much more extensive unit testing
}
