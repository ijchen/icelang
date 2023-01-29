use std::rc::Rc;

use crate::{
    ast::{AstNodeComparison, ComparisonKind},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

use super::core::interpret_expression;

/// Interprets an AstNodeComparison
///
/// # Panics
/// - If the node is invalid
pub fn interpret_comparison<'source>(
    node: &AstNodeComparison<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    // This is repeated often... or, *was* repeated often :)
    macro_rules! invalid_types {
        ($lhs: ident, $kind: ident, $rhs: ident) => {
            return Err(RuntimeError::Type {
                pos: node.pos().clone(),
                why: format!(
                    "invalid types for comparison: {} {} {}",
                    $lhs.icelang_type(),
                    $kind,
                    $rhs.icelang_type(),
                ),
            })
        };
    }

    // Parse the first (left-most) value in the comparison
    let mut lhs = interpret_expression(node.first(), state)?;

    // Perform the comparisons
    for (kind, node) in node.comparisons() {
        // Parse the rhs
        let rhs = interpret_expression(node, state)?;

        // Perform the comparison
        let sub_comparison = match (&lhs, &rhs) {
            (Value::Int(lhs_value), Value::Int(rhs_value)) => match kind {
                ComparisonKind::Equal => lhs_value == rhs_value,
                ComparisonKind::NotEqual => lhs_value != rhs_value,
                ComparisonKind::LessThan => lhs_value < rhs_value,
                ComparisonKind::GreaterThan => lhs_value > rhs_value,
                ComparisonKind::LessThanOrEqual => lhs_value <= rhs_value,
                ComparisonKind::GreaterThanOrEqual => lhs_value >= rhs_value,
            },

            (Value::Byte(lhs_value), Value::Byte(rhs_value)) => match kind {
                ComparisonKind::Equal => lhs_value == rhs_value,
                ComparisonKind::NotEqual => lhs_value != rhs_value,
                ComparisonKind::LessThan => lhs_value < rhs_value,
                ComparisonKind::GreaterThan => lhs_value > rhs_value,
                ComparisonKind::LessThanOrEqual => lhs_value <= rhs_value,
                ComparisonKind::GreaterThanOrEqual => lhs_value >= rhs_value,
            },

            // TODO ensure these are doing exactly what I want them to
            // which is *probably* just match IEEE-754, which is *probably* what
            // Rust does
            (Value::Float(lhs_value), Value::Float(rhs_value)) => match kind {
                ComparisonKind::Equal => lhs_value == rhs_value,
                ComparisonKind::NotEqual => lhs_value != rhs_value,
                ComparisonKind::LessThan => lhs_value < rhs_value,
                ComparisonKind::GreaterThan => lhs_value > rhs_value,
                ComparisonKind::LessThanOrEqual => lhs_value <= rhs_value,
                ComparisonKind::GreaterThanOrEqual => lhs_value >= rhs_value,
            },

            (Value::Bool(lhs_value), Value::Bool(rhs_value)) => match kind {
                // This, fun fact, is logical XNOR
                ComparisonKind::Equal => lhs_value == rhs_value,

                // This, fun fact, is logical XOR
                ComparisonKind::NotEqual => lhs_value != rhs_value,

                kind => invalid_types!(lhs, kind, rhs),
            },

            (Value::String(lhs_value), Value::String(rhs_value)) => match kind {
                ComparisonKind::Equal => lhs_value == rhs_value,
                ComparisonKind::NotEqual => lhs_value != rhs_value,
                kind => invalid_types!(lhs, kind, rhs),
            },

            // Lists are compared by reference equality
            (Value::List(lhs_value), Value::List(rhs_value)) => match kind {
                ComparisonKind::Equal => Rc::ptr_eq(lhs_value, rhs_value),
                ComparisonKind::NotEqual => !Rc::ptr_eq(lhs_value, rhs_value),
                kind => invalid_types!(lhs, kind, rhs),
            },

            // Dicts are compared by reference equality
            (Value::Dict(lhs_value), Value::Dict(rhs_value)) => match kind {
                ComparisonKind::Equal => Rc::ptr_eq(lhs_value, rhs_value),
                ComparisonKind::NotEqual => !Rc::ptr_eq(lhs_value, rhs_value),
                kind => invalid_types!(lhs, kind, rhs),
            },

            (Value::Null, Value::Null) => match kind {
                ComparisonKind::Equal => true,
                ComparisonKind::NotEqual => false,
                kind => invalid_types!(lhs, kind, rhs),
            },

            // Different types cannot be compared
            (lhs, rhs) => invalid_types!(lhs, kind, rhs),
        };

        // If this comparison was false, short-circuit and return false
        if !sub_comparison {
            return Ok(Value::Bool(false));
        }

        // The old rhs is the new lhs
        lhs = rhs;
    }

    // If no comparison was false, the whole thing is true
    Ok(Value::Bool(true))
}
