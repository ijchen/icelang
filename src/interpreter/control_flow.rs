use num_traits::Signed;

use crate::{
    ast::{
        AstNodeForLoop, AstNodeIfElseStatement, AstNodeMatchStatement, AstNodeSimpleLoop,
        AstNodeWhileLoop, JumpStatementKind,
    },
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::{
    core::{interpret_expression, interpret_statement},
    runtime_result::{NonLinearControlFlow, RuntimeResult},
};

/// Interprets an AstNodeSimpleLoop
pub fn interpret_simple_loop<'source>(
    simple_loop: &AstNodeSimpleLoop<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, ()> {
    match simple_loop.condition() {
        Some(condition) => match interpret_expression(condition, state)? {
            // TODO DRY this up (and all the loop bodies in general - there's a
            // lot of repeated code)
            Value::Int(iterations) => {
                // TODO what should even happen here?
                // Hi from future me: this should *definitely* be a RuntimeError
                if iterations.is_negative() {
                    todo!();
                }

                let mut iterations_remaining = iterations;
                'icelang_loop: while iterations_remaining.is_positive() {
                    state.push_scope();

                    for statement in simple_loop.body() {
                        match interpret_statement(statement, state) {
                            Ok(()) => {}
                            Err(NonLinearControlFlow::JumpStatement(jump_statement)) => {
                                match jump_statement.kind() {
                                    JumpStatementKind::Break => break 'icelang_loop,
                                    JumpStatementKind::Continue => break,
                                    JumpStatementKind::Return => {
                                        return Err(NonLinearControlFlow::JumpStatement(
                                            jump_statement,
                                        ));
                                    }
                                }
                            }
                            Err(NonLinearControlFlow::RuntimeError(err)) => {
                                return Err(NonLinearControlFlow::RuntimeError(err))
                            }
                        }
                    }

                    state.pop_scope();
                    iterations_remaining -= 1;
                }
            }
            Value::Byte(iterations) => {
                'icelang_loop: for _ in 0..iterations {
                    state.push_scope();

                    for statement in simple_loop.body() {
                        match interpret_statement(statement, state) {
                            Ok(()) => {}
                            Err(NonLinearControlFlow::JumpStatement(jump_statement)) => {
                                match jump_statement.kind() {
                                    JumpStatementKind::Break => break 'icelang_loop,
                                    JumpStatementKind::Continue => break,
                                    JumpStatementKind::Return => {
                                        return Err(NonLinearControlFlow::JumpStatement(
                                            jump_statement,
                                        ));
                                    }
                                }
                            }
                            Err(NonLinearControlFlow::RuntimeError(err)) => {
                                return Err(NonLinearControlFlow::RuntimeError(err))
                            }
                        }
                    }

                    state.pop_scope();
                }
            }
            _ => todo!(),
        },
        None => 'icelang_loop: loop {
            state.push_scope();

            for statement in simple_loop.body() {
                match interpret_statement(statement, state) {
                    Ok(()) => {}
                    Err(NonLinearControlFlow::JumpStatement(jump_statement)) => {
                        match jump_statement.kind() {
                            JumpStatementKind::Break => break 'icelang_loop,
                            JumpStatementKind::Continue => break,
                            JumpStatementKind::Return => {
                                return Err(NonLinearControlFlow::JumpStatement(jump_statement));
                            }
                        }
                    }
                    Err(NonLinearControlFlow::RuntimeError(err)) => {
                        return Err(NonLinearControlFlow::RuntimeError(err))
                    }
                }
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
) -> RuntimeResult<'source, ()> {
    'icelang_loop: loop {
        match interpret_expression(while_loop.condition(), state)? {
            Value::Bool(condition_value) => {
                if !condition_value {
                    break;
                }

                state.push_scope();

                for statement in while_loop.body() {
                    match interpret_statement(statement, state) {
                        Ok(()) => {}
                        Err(NonLinearControlFlow::JumpStatement(jump_statement)) => {
                            match jump_statement.kind() {
                                JumpStatementKind::Break => break 'icelang_loop,
                                JumpStatementKind::Continue => break,
                                JumpStatementKind::Return => {
                                    return Err(NonLinearControlFlow::JumpStatement(
                                        jump_statement,
                                    ));
                                }
                            }
                        }
                        Err(NonLinearControlFlow::RuntimeError(err)) => {
                            return Err(NonLinearControlFlow::RuntimeError(err))
                        }
                    }
                }

                state.pop_scope();
            }
            _ => todo!(),
        }
    }

    Ok(())
}

/// Interprets an AstNodeForLoop
pub fn interpret_for_loop<'source>(
    for_loop: &AstNodeForLoop<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, ()> {
    let iterable: Vec<Value> = match interpret_expression(for_loop.iterable(), state)? {
        // Iterating through a string visits each character
        Value::String(string) => string
            .chars()
            .map(|c| Value::String(c.to_string().into()))
            .collect(),

        // If we're iterating over a list, take a snapshot of the list as it is
        // at the start of the loop - mutations of the iterated list shouldn't
        // be reflected in the for loop's iterations
        Value::List(list) => list.borrow().iter().map(Value::reference_copy).collect(),

        value => {
            return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_type_error(
                    for_loop.pos().clone(),
                    state.scope_display_name().to_string(),
                    format!(
                        "expected string or list, got value of type {}",
                        value.icelang_type()
                    ),
                ),
            ))
        }
    };

    'icelang_loop: for value in iterable {
        state.push_scope();

        state.declare_variable(for_loop.ident().to_string(), value);

        for statement in for_loop.body() {
            match interpret_statement(statement, state) {
                Ok(()) => {}
                Err(NonLinearControlFlow::JumpStatement(jump_statement)) => {
                    match jump_statement.kind() {
                        JumpStatementKind::Break => break 'icelang_loop,
                        JumpStatementKind::Continue => break,
                        JumpStatementKind::Return => {
                            return Err(NonLinearControlFlow::JumpStatement(jump_statement));
                        }
                    }
                }
                Err(NonLinearControlFlow::RuntimeError(err)) => {
                    return Err(NonLinearControlFlow::RuntimeError(err))
                }
            }
        }

        state.pop_scope();
    }

    Ok(())
}

/// Interprets an AstNodeIfElseStatement
pub fn interpret_if_else_statement<'source>(
    if_else_statement: &AstNodeIfElseStatement<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, ()> {
    for (condition, body) in if_else_statement.conditional_branches() {
        // Evaluate the condition
        let condition_value = interpret_expression(condition, state)?;

        let Value::Bool(condition_value) = condition_value else {
            return Err(NonLinearControlFlow::RuntimeError(RuntimeError::new_type_error(
                condition.pos().clone(),
                state.scope_display_name().to_string(),
                format!("expected bool, got value of type {}", condition_value.icelang_type())
            )));
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

/// Interprets an AstNodeWhileLoop
pub fn interpret_match_statement<'source>(
    match_statement: &AstNodeMatchStatement<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, ()> {
    let matched_value = interpret_expression(match_statement.matched_expression(), state)?;

    for arm in match_statement.arms() {
        let pattern_value = interpret_expression(arm.pattern(), state)?;
        // TODO use icelang eq instead of Rust eq
        if matched_value == pattern_value {
            state.push_scope();

            for statement in arm.body() {
                interpret_statement(statement, state)?;
            }

            state.pop_scope();

            break;
        }
    }

    Ok(())
}
