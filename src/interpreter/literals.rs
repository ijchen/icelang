use std::collections::HashMap;

use super::*;

use crate::{
    ast::{AstNodeDictLiteral, AstNodeListLiteral, AstNodeLiteral},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

/// Interprets a list literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeListLiteral isn't a valid list literal
pub fn interpret_literal_list<'source>(
    node: &AstNodeListLiteral<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    let mut list = Vec::with_capacity(node.elements().len());

    for element_node in node.elements() {
        list.push(interpret_expression(element_node, state)?);
    }

    Ok(Value::List(list))
}

/// Interprets a dict literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeDictLiteral isn't a valid dict literal
pub fn interpret_literal_dict<'source>(
    node: &AstNodeDictLiteral<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    let mut dict = HashMap::with_capacity(node.entries().len());

    for (key_node, value_node) in node.entries() {
        let key = interpret_expression(key_node, state)?;
        let value = interpret_expression(value_node, state)?;
        dict.insert(key, value);
    }

    Ok(Value::Dict(dict))
}

/// Interprets an AstNodeLiteral into a Value
///
/// # Panics
/// - If the literal is invalid
pub fn interpret_literal(node: &AstNodeLiteral) -> Value {
    node.value().clone()
}
