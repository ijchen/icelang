use num_traits::Signed;

use crate::{
    ast::{AstNode, AstNodeSimpleLoop},
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
