//! Contains code related to interpreting (reading and executing code
//! represented as an abstract syntax tree (AST))

use std::collections::HashMap;

use crate::{
    ast::{Ast, AstNode, AstNodeDictLiteral, AstNodeListLiteral, AstNodeLiteral},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

/// Interprets a list literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeListLiteral isn't a valid list literal
fn interpret_literal_list<'source>(
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
fn interpret_literal_dict<'source>(
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
fn interpret_literal(node: &AstNodeLiteral) -> Value {
    node.value().clone()
}

/// Interprets an expression AstNode
///
/// # Panics
/// - if the AstNode isn't a valid expression
fn interpret_expression<'source>(
    expression: &AstNode<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    match expression {
        AstNode::VariableAccess(_) => todo!(),
        AstNode::Literal(node) => Ok(interpret_literal(node)),
        AstNode::ListLiteral(node) => interpret_literal_list(node, state),
        AstNode::FormattedStringLiteral(_) => todo!(),
        AstNode::DictLiteral(node) => interpret_literal_dict(node, state),
        AstNode::TypeCast(_) => todo!(),
        AstNode::UsageSuffix(_) => todo!(),
        AstNode::BinaryOperation(_) => todo!(),
        AstNode::UnaryOperation(_) => todo!(),
        AstNode::Comparison(_) => todo!(),
        AstNode::InlineConditional(_) => todo!(),
        AstNode::Assignment(_) => todo!(),
        _ => panic!("expected expression"),
    }
}

/// Interprets an AST with the given runtime state
pub fn interpret_with_runtime_state<'source>(
    ast: &Ast<'source>,
    state: &mut RuntimeState,
) -> Result<(), RuntimeError<'source>> {
    for statement in &ast.statements {
        match statement {
            AstNode::FunctionDeclaration(_) => todo!(),
            AstNode::VariableDeclaration(_) => todo!(),
            AstNode::VariableAccess(_) => todo!(),
            AstNode::Literal(_)
            | AstNode::ListLiteral(_)
            | AstNode::FormattedStringLiteral(_)
            | AstNode::DictLiteral(_)
            | AstNode::TypeCast(_)
            | AstNode::UsageSuffix(_)
            | AstNode::BinaryOperation(_)
            | AstNode::UnaryOperation(_)
            | AstNode::Comparison(_)
            | AstNode::InlineConditional(_)
            | AstNode::Assignment(_) => {
                let value = interpret_expression(statement, state)?;

                state.update_most_recent_value(value);
            }
            AstNode::JumpStatement(_) => todo!(),
            AstNode::SimpleLoop(_) => todo!(),
            AstNode::WhileLoop(_) => todo!(),
            AstNode::ForLoop(_) => todo!(),
            AstNode::MatchStatement(_) => todo!(),
            AstNode::IfElseStatement(_) => todo!(),
        }
    }

    Ok(())
}

/// Interprets an AST
pub fn interpret<'source>(ast: &Ast<'source>) -> Result<RuntimeState, RuntimeError<'source>> {
    let mut state = RuntimeState::new();

    interpret_with_runtime_state(ast, &mut state)?;

    Ok(state)
}
