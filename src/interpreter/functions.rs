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

use super::{
    core::interpret_statement,
    runtime_result::{NonLinearControlFlow, RuntimeResult},
};

/// Interprets an AstNodeFunctionDeclaration
pub fn interpret_function_declaration<'source>(
    function_declaration: &AstNodeFunctionDeclaration<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, ()> {
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
            return Err(NonLinearControlFlow::RuntimeError(
                RuntimeError::new_identifier_already_declared_error(
                    pos,
                    state.scope_display_name().to_string(),
                    identifier,
                ),
            ));
        }
    }

    // Ensure none of the parameter names are the same
    if let FunctionParameters::Polyadic { parameters } = function_declaration.parameters() {
        for (i, (parameter_1_name, parameter_1_pos)) in parameters.iter().enumerate() {
            for (parameter_2_name, _) in parameters.iter().take(i) {
                if parameter_1_name == parameter_2_name {
                    return Err(NonLinearControlFlow::RuntimeError(
                        RuntimeError::new_identifier_already_declared_error(
                            parameter_1_pos.clone(),
                            state.scope_display_name().to_string(),
                            parameter_1_name.to_string(),
                        ),
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
) -> RuntimeResult<'source, Value> {
    let AstNode::VariableAccess(ident_node) = function_call_node.root() else {
        return Err(NonLinearControlFlow::RuntimeError(
            RuntimeError::new_called_non_function_error(
                function_call_node.pos().clone(),
                state.scope_display_name().to_string()
            )
        ));
    };
    let function_name = ident_node.ident();

    // If the function is a standard library function, intercept the function
    // call and handle that as a special case
    if let Some(std_lib_function) = StdLibFunction::from_identifier(function_name) {
        // Evaluate the arguments
        let arguments = function_call_node
            .arguments()
            .iter()
            .map(|node| interpret_expression(node, state))
            .collect::<Result<_, _>>()?;

        // Push a new stack frame
        state.push_stack_frame(format!("{function_name}(...)"));

        // Call the function
        let return_value =
            std_lib_function.as_fn_pointer()(arguments, function_call_node.pos(), state);

        // Pop the stack frame
        state.pop_stack_frame();

        return return_value;
    }

    let Some(function_group) = state.lookup_function(function_name) else {
        return Err(NonLinearControlFlow::RuntimeError(RuntimeError::new_undefined_reference_error(
            function_call_node.pos().clone(),
            state.scope_display_name().to_string(),
            function_name.to_string(),
        )));
    };

    let function = function_group
        .get_polyadic_overload(function_call_node.arguments().len())
        .or_else(|| function_group.get_variadic_overload())
        .ok_or_else(|| {
            NonLinearControlFlow::RuntimeError(RuntimeError::new_invalid_overload_error(
                function_call_node.pos().clone(),
                state.scope_display_name().to_string(),
                function_name.to_string(),
                function_call_node.arguments().len(),
            ))
        })?
        // TODO look into ways to avoid this clone - FWIW, I don't think it is
        // avoidable. It is right now (once declared, a function overload can't
        // be modified) but this is likely to change once first-class functions
        // are supported, and then they will be values which probably *have* to
        // be cloned
        .clone();

    // Evaluate the arguments
    let arguments: Vec<Value> = function_call_node
        .arguments()
        .iter()
        .map(|argument_node| interpret_expression(argument_node, state))
        .collect::<Result<_, _>>()?;

    // Push a new stack frame
    state.push_stack_frame(format!("{function_name}({})", function.parameters()));

    let mut return_value = Value::Null;

    // Bind the arguments to local variables
    match function.parameters() {
        FunctionParameters::Variadic { parameter_name } => {
            state.declare_variable(
                parameter_name.0.to_string(),
                Value::List(Rc::new(RefCell::new(arguments))),
            );
        }
        FunctionParameters::Polyadic { parameters } => {
            assert_eq!(arguments.len(), parameters.len());

            for (parameter_name, argument_value) in parameters.iter().zip(arguments.into_iter()) {
                state.declare_variable(parameter_name.0.clone(), argument_value);
            }
        }
    }

    for statement in function.body() {
        match interpret_statement(statement, state) {
            Ok(()) => {}
            Err(NonLinearControlFlow::JumpStatement(jump_statement)) => match jump_statement.kind()
            {
                JumpStatementKind::Return => {
                    return_value = jump_statement.into_value().unwrap_or(Value::Null);
                    break;
                }
                jump_kind => {
                    let mut err = RuntimeError::new_invalid_jump_statement_error(
                        jump_statement.pos().clone(),
                        state.scope_display_name().to_string(),
                        jump_kind,
                        "a function".to_string(),
                    );
                    state.pop_stack_frame();
                    err.stack_trace_mut().add_bottom(
                        state.scope_display_name().to_string(),
                        function_call_node.pos().clone(),
                    );
                    return Err(NonLinearControlFlow::RuntimeError(err));
                }
            },
            Err(NonLinearControlFlow::RuntimeError(mut err)) => {
                state.pop_stack_frame();
                err.stack_trace_mut().add_bottom(
                    state.scope_display_name().to_string(),
                    function_call_node.pos().clone(),
                );
                return Err(NonLinearControlFlow::RuntimeError(err));
            }
        }
    }

    // Pop the stack frame
    state.pop_stack_frame();

    Ok(return_value)
}
