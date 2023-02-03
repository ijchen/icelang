use crate::{
    ast::{AstNodeVariableAccess, AstNodeVariableDeclaration},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::core::interpret_expression;

/// Interprets a variable declaration AstNodeVariableDeclaration
pub fn interpret_variable_declaration<'source>(
    variable_declaration: &AstNodeVariableDeclaration<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<(), RuntimeError<'source>> {
    for (ident, value_expr, pos) in variable_declaration.declarations() {
        let value = match value_expr {
            Some(value_expr) => interpret_expression(value_expr, state)?,
            None => Value::Null,
        };

        if state.lookup_local_variable(ident).is_some() {
            return Err(RuntimeError::new_identifier_already_declared_error(
                pos.clone(),
                ident.to_string(),
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
) -> Result<Value, RuntimeError<'source>> {
    match state.lookup_variable(variable_access.ident()) {
        Some(value) => Ok(value.clone()),
        None => Err(RuntimeError::new_undefined_reference_error(
            variable_access.pos().clone(),
            variable_access.ident().to_string(),
        )),
    }
}
