use crate::{
    ast::{AstNodeUnaryOperation, UnaryOperationKind},
    error::runtime_error::RuntimeError,
    interpreter::{
        core::interpret_expression,
        runtime_result::{NonLinearControlFlow, RuntimeResult},
    },
    runtime_state::RuntimeState,
    value::Value,
};

macro_rules! impl_simple_unary_op {
    (
        $func_name: ident,
        $node: ident,
        $state: ident,
        $operand: ident,
        $op_kind: ident,
        {$(
            $operand_type: ident => $result:expr
        ),+$(,)?}
    ) => {
        fn $func_name<'source>(
            $node: &AstNodeUnaryOperation<'source>,
            $state: &mut RuntimeState<'source>,
        ) -> RuntimeResult<'source, Value> {
            assert!($node.operation() == UnaryOperationKind::$op_kind);

            let operand = interpret_expression($node.operand(), $state)?;

            match operand {
                $(
                    Value::$operand_type($operand) => $result,
                )+
                $operand => Err(NonLinearControlFlow::RuntimeError(RuntimeError::new_type_error(
                    $node.pos().clone(),
                    $state.scope_display_name().to_string(),
                    format!(
                        "invalid types for unary operation: {}{}",
                        $node.operation(),
                        $operand.icelang_type(),
                    ),
                ))),
            }
        }
    };
}

impl_simple_unary_op!(interpret_not, node, state, operand, Not, {
    Int => Ok(Value::Int(!operand)),
    Byte => Ok(Value::Byte(!operand)),
    Bool => Ok(Value::Bool(!operand)),
});

impl_simple_unary_op!(interpret_identity, node, state, operand, Identity, {
    Int => Ok(Value::Int(operand)),
    Byte => Ok(Value::Byte(operand)),
    Float => Ok(Value::Float(operand)),
});

impl_simple_unary_op!(interpret_negation, node, state, operand, Negation, {
    Int => Ok(Value::Int(-operand)),
    Float => Ok(Value::Float(-operand)),
});

/// Interprets an AstNodeUnaryOperation
///
/// # Panics
/// - If the AstNodeUnaryOperation isn't a valid binary operation node
pub fn interpret_unary_operation<'source>(
    node: &AstNodeUnaryOperation<'source>,
    state: &mut RuntimeState<'source>,
) -> RuntimeResult<'source, Value> {
    match node.operation() {
        UnaryOperationKind::Not => interpret_not(node, state),
        UnaryOperationKind::Identity => interpret_identity(node, state),
        UnaryOperationKind::Negation => interpret_negation(node, state),
    }
}
