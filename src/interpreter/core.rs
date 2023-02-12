use super::{
    assignments::interpret_assignment,
    comparisons::interpret_comparison,
    functions::{interpret_function_call, interpret_function_declaration},
    inline_conditionals::interpret_inline_conditional,
    member_access::{interpret_computed_member_access, interpret_dot_member_access},
    unary_operations::interpret_unary_operation,
    variables::{interpret_variable_access, interpret_variable_declaration},
    *,
};

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
    state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    match expression {
        AstNode::VariableAccess(node) => interpret_variable_access(node, state),
        AstNode::Literal(node) => Ok(interpret_literal(node)),
        AstNode::ListLiteral(node) => interpret_literal_list(node, state),
        AstNode::FormattedStringLiteral(node) => interpret_formatted_string_literal(node, state),
        AstNode::DictLiteral(node) => interpret_literal_dict(node, state),
        AstNode::TypeCast(node) => interpret_type_cast(node, state),
        AstNode::DotMemberAccess(node) => interpret_dot_member_access(node, state),
        AstNode::ComputedMemberAccess(node) => interpret_computed_member_access(node, state),
        AstNode::FunctionCall(node) => interpret_function_call(node, state),
        AstNode::BinaryOperation(node) => interpret_binary_operation(node, state),
        AstNode::UnaryOperation(node) => interpret_unary_operation(node, state),
        AstNode::Comparison(node) => interpret_comparison(node, state),
        AstNode::InlineConditional(node) => interpret_inline_conditional(node, state),
        AstNode::Assignment(node) => interpret_assignment(node, state),
        _ => panic!("expected expression"),
    }
}

/// Interprets a statement
///
/// # Panics
/// - If the statement is invalid in any way
pub fn interpret_statement<'source>(
    statement: &AstNode<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<(), RuntimeError<'source>> {
    match statement {
        AstNode::FunctionDeclaration(function_declaration) => {
            interpret_function_declaration(function_declaration, state)
        }
        AstNode::VariableDeclaration(variable_declaration) => {
            interpret_variable_declaration(variable_declaration, state)
        }
        AstNode::VariableAccess(_)
        | AstNode::Literal(_)
        | AstNode::ListLiteral(_)
        | AstNode::FormattedStringLiteral(_)
        | AstNode::DictLiteral(_)
        | AstNode::TypeCast(_)
        | AstNode::DotMemberAccess(_)
        | AstNode::ComputedMemberAccess(_)
        | AstNode::FunctionCall(_)
        | AstNode::BinaryOperation(_)
        | AstNode::UnaryOperation(_)
        | AstNode::Comparison(_)
        | AstNode::InlineConditional(_)
        | AstNode::Assignment(_) => {
            let value = interpret_expression(statement, state)?;

            state.update_most_recent_value(value);

            Ok(())
        }
        AstNode::JumpStatement(_) => todo!(),
        AstNode::SimpleLoop(_) => todo!(),
        AstNode::WhileLoop(_) => todo!(),
        AstNode::ForLoop(_) => todo!(),
        AstNode::MatchStatement(_) => todo!(),
        AstNode::IfElseStatement(_) => todo!(),
    }
}

/// Interprets an AST with the given runtime state
///
/// # Panics
/// - If the Ast contains any invalid AstNodes
pub fn interpret_with_runtime_state<'source>(
    ast: &Ast<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<(), RuntimeError<'source>> {
    state.update_most_recent_value(Value::Null);

    for statement in &ast.statements {
        interpret_statement(statement, state)?;
    }

    Ok(())
}

/// Interprets an AST
///
/// # Panics
/// - If the Ast contains any invalid AstNodes
pub fn interpret<'source>(
    ast: &Ast<'source>,
) -> Result<RuntimeState<'source>, RuntimeError<'source>> {
    let mut state = RuntimeState::new();

    interpret_with_runtime_state(ast, &mut state)?;

    Ok(state)
}
