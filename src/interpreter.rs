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
/// - If the AstNodeLiteral isn't a valid int literal
fn interpret_literal_int<'source>(node: &AstNodeLiteral) -> Result<Value, RuntimeError<'source>> {
    assert!(node.icelang_type() == IcelangType::Int);

    use std::num::IntErrorKind;
    let value = match str::parse::<i64>(node.raw()) {
        Ok(value) => value,
        Err(err) => match err.kind() {
            IntErrorKind::Empty | IntErrorKind::InvalidDigit => panic!("invalid int literal"),
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => todo!(),
            IntErrorKind::Zero => unreachable!(),
            _ => panic!("Unexpected error parsing integer"),
        },
    };

    Ok(Value::Int(value))
}

/// Interprets a byte literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a valid byte literal
fn interpret_literal_byte<'source>(node: &AstNodeLiteral) -> Result<Value, RuntimeError<'source>> {
    assert!(node.icelang_type() == IcelangType::Byte);

    todo!()
}

/// Interprets a float literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a valid float literal
fn interpret_literal_float<'source>(node: &AstNodeLiteral) -> Result<Value, RuntimeError<'source>> {
    assert!(node.icelang_type() == IcelangType::Float);

    todo!()
}

/// Interprets a bool literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a valid bool literal
fn interpret_literal_bool<'source>(node: &AstNodeLiteral) -> Result<Value, RuntimeError<'source>> {
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
fn interpret_literal_string<'source>(
    node: &AstNodeLiteral,
) -> Result<Value, RuntimeError<'source>> {
    assert!(node.icelang_type() == IcelangType::String);

    todo!()
}

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

/// Interprets a null literal AstNodeLiteral
///
/// # Panics
/// - If the AstNodeLiteral isn't a valid null literal
fn interpret_literal_null<'source>(node: &AstNodeLiteral) -> Result<Value, RuntimeError<'source>> {
    assert!(node.icelang_type() == IcelangType::Null);

    if node.raw() != "null" {
        panic!("invalid null literal")
    }

    Ok(Value::Null)
}

/// Interprets an AstNodeLiteral into a Value
///
/// # Panics
/// - If the literal is invalid
fn interpret_literal<'source>(node: &AstNodeLiteral) -> Result<Value, RuntimeError<'source>> {
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
