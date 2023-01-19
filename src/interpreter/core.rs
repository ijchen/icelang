use super::*;

use crate::{
    ast::{Ast, AstNode},
    error::runtime_error::RuntimeError,
    runtime_state::RuntimeState,
    value::Value,
};

/// Interprets an expression AstNode
///
/// # Panics
/// - if the AstNode isn't a valid expression
pub fn interpret_expression<'source>(
    expression: &AstNode<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    match expression {
        AstNode::VariableAccess(_) => todo!(),
        AstNode::Literal(node) => Ok(interpret_literal(node)),
        AstNode::ListLiteral(node) => interpret_literal_list(node, state),
        AstNode::FormattedStringLiteral(_) => todo!(),
        AstNode::DictLiteral(node) => interpret_literal_dict(node, state),
        AstNode::TypeCast(node) => interpret_type_cast(node, state),
        AstNode::UsageSuffix(_) => todo!(),
        AstNode::BinaryOperation(node) => interpret_binary_operation(node, state),
        AstNode::UnaryOperation(_) => todo!(),
        AstNode::Comparison(_) => todo!(),
        AstNode::InlineConditional(_) => todo!(),
        AstNode::Assignment(_) => todo!(),
        _ => panic!("expected expression"),
    }
}

/// Interprets an AST with the given runtime state
///
/// # Panics
/// - If the Ast contains any invalid AstNodes
pub fn interpret_with_runtime_state<'source>(
    ast: &Ast<'source>,
    state: &mut RuntimeState,
) -> Result<(), RuntimeError<'source>> {
    state.update_most_recent_value(Value::Null);

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
///
/// # Panics
/// - If the Ast contains any invalid AstNodes
pub fn interpret<'source>(ast: &Ast<'source>) -> Result<RuntimeState, RuntimeError<'source>> {
    let mut state = RuntimeState::new();

    interpret_with_runtime_state(ast, &mut state)?;

    Ok(state)
}
