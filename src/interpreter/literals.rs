use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{runtime_result::RuntimeResult, *};

use crate::{
    ast::{AstNodeDictLiteral, AstNodeFormattedStringLiteral, AstNodeListLiteral, AstNodeLiteral},
    runtime_state::RuntimeState,
    value::Value,
};

/// Interprets a list literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeListLiteral isn't a valid list literal
pub fn interpret_literal_list<'source>(
    node: &AstNodeListLiteral<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    let mut list = Vec::with_capacity(node.elements().len());

    for element_node in node.elements() {
        list.push(interpret_expression(element_node, state)?);
    }

    Ok(Value::List(Rc::new(RefCell::new(list))))
}

/// Interprets a dict literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeDictLiteral isn't a valid dict literal
pub fn interpret_literal_dict<'source>(
    node: &AstNodeDictLiteral<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    #[expect(
        clippy::mutable_key_type,
        reason = "current intended behavior is to allow mutable keys - likely needs design work to determine how to handle correctly"
    )]
    let mut dict = HashMap::with_capacity(node.entries().len());

    for (key_node, value_node) in node.entries() {
        let key = interpret_expression(key_node, state)?;
        let value = interpret_expression(value_node, state)?;
        dict.insert(key, value);
    }

    Ok(Value::Dict(Rc::new(RefCell::new(dict))))
}

/// Interprets a formatted string literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeDictLiteral isn't a formatted string literal
pub fn interpret_formatted_string_literal<'source>(
    node: &AstNodeFormattedStringLiteral<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    let mut buffer = String::new();

    for (string_part, replacement_field) in
        std::iter::once(node.start()).chain(node.continuations().into_iter())
    {
        buffer += string_part;
        buffer += &interpret_expression(replacement_field, state)?.icelang_display();
    }
    buffer += node.end();

    Ok(Value::String(buffer.into()))
}

/// Interprets an AstNodeLiteral into a Value
///
/// # Panics
/// - If the literal is invalid
pub fn interpret_literal(node: &AstNodeLiteral) -> Value {
    node.value().deep_copy()
}
