use num_traits::Signed;

use crate::{
    ast::{AstNode, AstNodeIfElseStatement, AstNodeSimpleLoop, AstNodeWhileLoop},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::core::{interpret_expression, interpret_statement};

/// Interprets an AstNodeSimpleLoop
pub fn interpret_simple_loop<'source>(
    simple_loop: &AstNodeSimpleLoop<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<(), RuntimeError<'source>> {
    match simple_loop.condition() {
        Some(condition) => match interpret_expression(condition, state)? {
            Value::Int(iterations) => {
                // TODO what should even happen here?
                if iterations.is_negative() {
                    todo!();
                }

                let mut iterations_remaining = iterations;
                'icelang_loop: while iterations_remaining.is_positive() {
                    state.push_scope();

                    for statement in simple_loop.body() {
                        if let AstNode::JumpStatement(statement) = statement {
                            match statement.jump_kind() {
                                crate::ast::JumpStatementKind::Break => break 'icelang_loop,
                                crate::ast::JumpStatementKind::Continue => break,
                                crate::ast::JumpStatementKind::Return => todo!(),
                            }
                        }

                        interpret_statement(statement, state)?;
                    }

                    state.pop_scope();
                    iterations_remaining -= 1;
                }
            }
            Value::Byte(iterations) => {
                'icelang_loop: for _ in 0..iterations {
                    state.push_scope();

                    for statement in simple_loop.body() {
                        if let AstNode::JumpStatement(statement) = statement {
                            match statement.jump_kind() {
                                crate::ast::JumpStatementKind::Break => break 'icelang_loop,
                                crate::ast::JumpStatementKind::Continue => break,
                                crate::ast::JumpStatementKind::Return => todo!(),
                            }
                        }

                        interpret_statement(statement, state)?;
                    }

                    state.pop_scope();
                }
            }
            _ => todo!(),
        },
        None => 'icelang_loop: loop {
            state.push_scope();

            for statement in simple_loop.body() {
                if let AstNode::JumpStatement(statement) = statement {
                    match statement.jump_kind() {
                        crate::ast::JumpStatementKind::Break => break 'icelang_loop,
                        crate::ast::JumpStatementKind::Continue => break,
                        crate::ast::JumpStatementKind::Return => todo!(),
                    }
                }

                interpret_statement(statement, state)?;
            }

            state.pop_scope();
        },
    }

    Ok(())
}

/// Interprets an AstNodeWhileLoop
pub fn interpret_while_loop<'source>(
    while_loop: &AstNodeWhileLoop<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<(), RuntimeError<'source>> {
    'icelang_loop: loop {
        match interpret_expression(while_loop.condition(), state)? {
            Value::Bool(condition_value) => {
                if !condition_value {
                    break;
                }

                state.push_scope();

                for statement in while_loop.body() {
                    if let AstNode::JumpStatement(statement) = statement {
                        match statement.jump_kind() {
                            crate::ast::JumpStatementKind::Break => break 'icelang_loop,
                            crate::ast::JumpStatementKind::Continue => break,
                            crate::ast::JumpStatementKind::Return => todo!(),
                        }
                    }

                    interpret_statement(statement, state)?;
                }

                state.pop_scope();
            }
            _ => todo!(),
        }
    }

    Ok(())
}

/// Interprets an AstNodeIfElseStatement
pub fn interpret_if_else_statement<'source>(
    if_else_statement: &AstNodeIfElseStatement<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<(), RuntimeError<'source>> {
    for (condition, body) in if_else_statement.conditional_branches() {
        // Evaluate the condition
        let condition_value = interpret_expression(condition, state)?;

        let Value::Bool(condition_value) = condition_value else {
            return Err(RuntimeError::new_type_error(
                condition.pos().clone(),
                state.scope_display_name().to_string(),
                "expected bool in condition".to_string())
            );
        };

        if condition_value {
            state.push_scope();

            for statement in body {
                interpret_statement(statement, state)?;
            }

            state.pop_scope();

            return Ok(());
        }
    }

    if let Some(else_branch_body) = if_else_statement.else_branch() {
        for statement in else_branch_body {
            interpret_statement(statement, state)?;
        }
    }

    Ok(())
}
