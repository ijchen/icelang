use crate::{
    ast::AstNodeFunctionDeclaration, error::runtime_error::RuntimeError,
    function::FunctionParameters, runtime_state::RuntimeState,
};

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
