use crate::{
    ast::{AstNodeVariableAccess, AstNodeVariableDeclaration},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::{
    core::interpret_expression,
    runtime_result::{NonLinearControlFlow, RuntimeResult},
};

/// Interprets a variable declaration AstNodeVariableDeclaration
pub fn interpret_variable_declaration<'source>(
    variable_declaration: &AstNodeVariableDeclaration<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, ()> {
    for (ident, value_expr, pos) in variable_declaration.declarations() {
        let value = match value_expr {
            Some(value_expr) => interpret_expression(value_expr, state)?,
            None => Value::Null,
        };

        if state.lookup_local_variable(ident).is_some() {
            return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_identifier_already_declared_error(
                    pos.clone(),
                    state.scope_display_name().to_string(),
                    ident.to_string(),
                ),
            ));
        }

        state.declare_variable(ident.to_string(), value);
    }

    Ok(())
}

/// Interprets a variable access AstNode
pub fn interpret_variable_access<'source>(
    variable_access: &AstNodeVariableAccess<'source>,
    state: &mut RuntimeState,
) -> RuntimeResult<'source, Value> {
    match state.lookup_variable(variable_access.ident()) {
        Some(value) => Ok(value.reference_copy()),
        None => Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_undefined_reference_error(
                variable_access.pos().clone(),
                state.scope_display_name().to_string(),
                variable_access.ident().to_string(),
            ),
        )),
    }
}
