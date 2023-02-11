use crate::{
    ast::AstNodeInlineConditional, error::runtime_error::RuntimeError, runtime_state::RuntimeState,
    value::Value,
};

use super::core::interpret_expression;

/// Interprets an expression AstNodeInlineConditional
///
/// # Panics
/// - if the AstNode isn't a valid expression
pub fn interpret_inline_conditional<'source>(
    node: &AstNodeInlineConditional<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    let condition = interpret_expression(node.condition(), state)?;

    if let Value::Bool(condition) = condition {
        interpret_expression(
            if condition {
                node.truthy_case()
            } else {
                node.falsey_case()
            },
            state,
        )
    } else {
        Err(RuntimeError::new_type_error(
            node.condition().pos().clone(),
            state.scope_display_name().to_string(),
            "expected bool in inline conditional".to_string(),
        ))
    }
}
