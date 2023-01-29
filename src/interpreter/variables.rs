use crate::{
    ast::{AstNode, AstNodeVariableAccess},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::core::interpret_expression;

/// Interprets a variable declaration AstNode
///
/// # Panics
/// - if the AstNode isn't a valid variable declaration
pub fn interpret_variable_declaration<'source>(
    variable_declaration: &AstNode<'source>,
    state: &mut RuntimeState,
) -> Result<(), RuntimeError<'source>> {
    let AstNode::VariableDeclaration(variable_declaration) = variable_declaration else {
        panic!("AstNode was not a variable declaration");
    };

    for (ident, value_expr, pos) in variable_declaration.declarations() {
        let value = match value_expr {
            Some(value_expr) => interpret_expression(value_expr, state)?,
            None => Value::Null,
        };

        if state
            .global_symbol_table_mut()
            .declare_variable(ident.clone(), value)
            .is_none()
        {
            return Err(RuntimeError::new_identifier_already_declared_error(
                pos.clone(),
                ident.clone(),
            ));
        };
    }

    Ok(())
}

/// Interprets a variable access AstNode
pub fn interpret_variable_access<'source>(
    variable_access: &AstNodeVariableAccess<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    match state
        .global_symbol_table()
        .access_variable(variable_access.ident())
    {
        Some(value) => Ok(value.clone()),
        None => Err(RuntimeError::new_undefined_reference_error(
            variable_access.pos().clone(),
            variable_access.ident().to_string(),
        )),
    }
}
