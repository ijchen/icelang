//! Contains code related to interpreting (reading and executing code
//! represented as an abstract syntax tree (AST))

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
    node: &AstNodeListLiteral,
) -> Result<Value, RuntimeError<'source>> {
    let _ = node;
    todo!()
}

/// Interprets a dict literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeDictLiteral isn't a valid dict literal
fn interpret_literal_dict<'source>(
    node: &AstNodeDictLiteral,
) -> Result<Value, RuntimeError<'source>> {
    let _ = node;
    todo!()
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
pub fn interpret_expression<'source>(
    expression: &AstNode<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source>> {
    let _ = state; // TODO
    match expression {
        AstNode::VariableAccess(_) => todo!(),
        AstNode::Literal(node) => Ok(interpret_literal(node)),
        AstNode::ListLiteral(node) => interpret_literal_list(node),
        AstNode::FormattedStringLiteral(_) => todo!(),
        AstNode::DictLiteral(node) => interpret_literal_dict(node),
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

/// Interprets an AST
pub fn interpret<'source>(ast: &Ast<'source>) -> Result<RuntimeState, RuntimeError<'source>> {
    let mut state = RuntimeState::new();

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
                let value = interpret_expression(statement, &mut state)?;

                println!("{value:?}");

                let _ = value;
                todo!()
            }
            AstNode::JumpStatement(_) => todo!(),
            AstNode::SimpleLoop(_) => todo!(),
            AstNode::WhileLoop(_) => todo!(),
            AstNode::ForLoop(_) => todo!(),
            AstNode::MatchStatement(_) => todo!(),
            AstNode::IfElseStatement(_) => todo!(),
        }
    }

    Ok(state)
}
