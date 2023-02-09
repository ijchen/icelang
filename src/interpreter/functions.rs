use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{AstNode, AstNodeFunctionCall, AstNodeFunctionDeclaration, JumpStatementKind},
    error::runtime_error::RuntimeError,
    function::FunctionParameters,
    icelang_std_lib::StdLibFunction,
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

    // Ensure none of the parameter names are the same
    if let FunctionParameters::Polyadic { parameters } = function_declaration.parameters() {
        for (i, (parameter_1_name, parameter_1_pos)) in parameters.iter().enumerate() {
            for (parameter_2_name, _) in parameters.iter().take(i) {
                if parameter_1_name == parameter_2_name {
                    return Err(RuntimeError::new_identifier_already_declared_error(
                        parameter_1_pos.clone(),
                        parameter_1_name.to_string(),
                    ));
                }
            }
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

    // If the function is a standard library function, intercept the function
    // call and handle that as a special case
    if let Some(std_lib_function) = StdLibFunction::from_identifier(function_name) {
        let arguments = function_call_node
            .arguments()
            .iter()
            .map(|node| interpret_expression(node, state))
            .collect::<Result<_, _>>()?;
        return std_lib_function.call(arguments, function_call_node.pos(), state);
    }

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

    // Push a new stack frame
    state.push_stack_frame();

    let mut return_value = Value::Null;

    // Bind the arguments to local variables
    match function.parameters() {
        FunctionParameters::Variadic { parameter_name } => {
            let parameters = function_call_node
                .arguments()
                .iter()
                .map(|node| interpret_expression(node, state))
                .collect::<Result<_, _>>()?;
            state.declare_variable(
                parameter_name.0.to_string(),
                Value::List(Rc::new(RefCell::new(parameters))),
            );
        }
        FunctionParameters::Polyadic { parameters } => {
            assert_eq!(function_call_node.arguments().len(), parameters.len());

            for (parameter_name, argument_node) in
                parameters.iter().zip(function_call_node.arguments().iter())
            {
                // TODO ensure there are no duplicate parameter names

                let value = interpret_expression(argument_node, state)?;

                state.declare_variable(parameter_name.0.clone(), value);
            }
        }
    }

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
