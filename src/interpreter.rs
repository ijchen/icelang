//! Contains code related to interpreting (reading and executing code
//! represented as an abstract syntax tree (AST))

use crate::{
    ast::{Ast, AstNode, AstNodeDictLiteral, AstNodeListLiteral, AstNodeLiteral},
    error::runtime_error::RuntimeError,
    icelang_type::IcelangType,
    runtime_state::RuntimeState,
    value::Value,
};

/// Interprets an int literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't an int
fn interpret_literal_int<'source, 'ast>(
    node: &AstNodeLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    assert!(node.icelang_type() == IcelangType::Int);

    todo!()
}

/// Interprets a byte literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a byte
fn interpret_literal_byte<'source, 'ast>(
    node: &AstNodeLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    assert!(node.icelang_type() == IcelangType::Byte);

    todo!()
}

/// Interprets a float literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a float
fn interpret_literal_float<'source, 'ast>(
    node: &AstNodeLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    assert!(node.icelang_type() == IcelangType::Float);

    todo!()
}

/// Interprets a bool literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a bool
fn interpret_literal_bool<'source, 'ast>(
    node: &AstNodeLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    assert!(node.icelang_type() == IcelangType::Bool);

    match node.raw() {
        "true" => Ok(Value::Bool(true)),
        "false" => Ok(Value::Bool(false)),
        _ => todo!(),
    }
}

/// Interprets a string literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a string
fn interpret_literal_string<'source, 'ast>(
    node: &AstNodeLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    assert!(node.icelang_type() == IcelangType::String);

    todo!()
}

/// Interprets a list literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a list
fn interpret_literal_list<'source, 'ast>(
    node: &AstNodeListLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    let _ = node;
    todo!()
}

/// Interprets a dict literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a dict
fn interpret_literal_dict<'source, 'ast>(
    node: &AstNodeDictLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    let _ = node;
    todo!()
}

/// Interprets a null literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a null
fn interpret_literal_null<'source, 'ast>(
    node: &AstNodeLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    assert!(node.icelang_type() == IcelangType::Null);

    if node.raw() != "null" {
        todo!();
    }

    Ok(Value::Null)
}

/// Interprets an AstNodeLiteral into a Value
fn interpret_literal<'source, 'ast>(
    node: &AstNodeLiteral,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    match node.icelang_type() {
        IcelangType::Int => interpret_literal_int(node),
        IcelangType::Byte => interpret_literal_byte(node),
        IcelangType::Float => interpret_literal_float(node),
        IcelangType::Bool => interpret_literal_bool(node),
        IcelangType::String => interpret_literal_string(node),
        IcelangType::List => todo!(),
        IcelangType::Dict => todo!(),
        IcelangType::Null => interpret_literal_null(node),
    }
}

/// Interprets an expression AstNode
pub fn interpret_expression<'source, 'ast>(
    expression: &'ast AstNode<'source>,
    state: &mut RuntimeState,
) -> Result<Value, RuntimeError<'source, 'ast>> {
    let _ = state; // TODO
    match expression {
        AstNode::VariableAccess(_) => todo!(),
        AstNode::Literal(node) => interpret_literal(node),
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
        node => Err(RuntimeError::new_malformed_ast_node_error(
            &node,
            "expected expression".to_string(),
        )),
    }
}

/// Interprets an AST
pub fn interpret<'source, 'ast>(
    ast: &'ast Ast<'source>,
) -> Result<RuntimeState, RuntimeError<'source, 'ast>> {
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
