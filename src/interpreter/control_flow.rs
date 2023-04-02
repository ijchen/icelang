use num_traits::Signed;

use crate::{
    ast::{
        AstNode, AstNodeForLoop, AstNodeIfElseStatement, AstNodeMatchStatement, AstNodeSimpleLoop,
        AstNodeWhileLoop,
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
) -> RuntimeResult<'source, ()> {
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

/// Interprets an AstNodeForLoop
pub fn interpret_for_loop<'source>(
    for_loop: &AstNodeForLoop<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, ()> {
    let Value::List(iterable) = interpret_expression(for_loop.iterable(), state)? else {
        todo!();
    };
    // Take a snapshot of the list as it is at the start of the loop - mutations
    // of the iterated list shouldn't be reflected in the for loop's iterations
    let iterable: Vec<Value> = iterable.borrow().clone();

    'icelang_loop: for value in iterable {
        state.push_scope();

        state.declare_variable(for_loop.ident().to_string(), value);

        for statement in for_loop.body() {
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
                "expected bool in condition".to_string())
            ));
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
