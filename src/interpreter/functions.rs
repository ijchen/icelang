use crate::{
    ast::{AstNode, AstNodeFunctionCall, AstNodeFunctionDeclaration, JumpStatementKind},
    error::runtime_error::RuntimeError,
    function::FunctionParameters,
    interpreter::core::interpret_expression,
    runtime_state::RuntimeState,
    value::Value,
};

use super::core::interpret_statement;

/// Interprets an AstNodeFunctionDeclaration
pub fn interpret_function_declaration<'source>(
    function_declaration: &AstNodeFunctionDeclaration<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<(), RuntimeError<'source>> {
    let identifier = function_declaration.name().to_string();
    let parameters = function_declaration.parameters().clone();
    let body = function_declaration.body().clone();
    let pos = function_declaration.pos().clone();

    if let Some(function_group) = state.lookup_local_function(&identifier) {
        if match &parameters {
            FunctionParameters::Variadic { parameter_name: _ } => {
                function_group.get_variadic_overload().is_some()
            }
            FunctionParameters::Polyadic { parameters } => function_group
                .get_polyadic_overload(parameters.len())
                .is_some(),
        } {
            return Err(RuntimeError::new_identifier_already_declared_error(
                pos, identifier,
            ));
        }
    }

    state.declare_function(identifier, parameters, body, pos);

    Ok(())
}

/// Interprets an AstNodeFunctionCall
pub fn interpret_function_call<'source>(
    function_call_node: &AstNodeFunctionCall<'source>,
    state: &mut RuntimeState<'source>,
) -> Result<Value, RuntimeError<'source>> {
    // TODO do this cleanly
    let AstNode::VariableAccess(ident_node) = function_call_node.root() else {
        todo!()
    };
    let function_name = ident_node.ident();

    // TODO just temporary
    if function_name == "println" {
        assert_eq!(function_call_node.arguments().len(), 1);

        let value = interpret_expression(&function_call_node.arguments()[0], state)?;

        println!("{}", value.icelang_display());

        return Ok(Value::Null);
    }

    // Push a new stack frame
    state.push_stack_frame();

    let Some(function_group) = state.lookup_function(function_name) else {
        return Err(RuntimeError::new_undefined_reference_error(
            function_call_node.pos().clone(),
            function_name.to_string(),
        ));
    };

    let function = function_group
        .get_polyadic_overload(function_call_node.arguments().len())
        .or_else(|| function_group.get_variadic_overload())
        .ok_or_else(|| todo!())?
        // TODO look into ways to avoid this clone - FWIW, I don't think it is
        // avoidable. It is right now (once declared, a function overload can't
        // be modified) but this is likely to change once first-class functions
        // are supported, and then they will be values which probably *have* to
        // be cloned
        .clone();

    let mut return_value = Value::Null;

    for statement in function.body() {
        if let AstNode::JumpStatement(node) = statement {
            if node.jump_kind() == JumpStatementKind::Return {
                if let Some(body) = node.body() {
                    return_value = interpret_expression(body, state)?;
                    break;
                }
            }
        }

        interpret_statement(statement, state).map_err(|_| todo!())?;
    }

    // Pop the stack frame
    state.pop_stack_frame();

    Ok(return_value)
}
