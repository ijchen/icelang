use crate::{ast::JumpStatementKind, source_range::SourceRange, value::Value};

#[derive(Debug)]
pub struct JumpStatement<'source> {
    kind: JumpStatementKind,
    value: Option<Value>,
    pos: SourceRange<'source>,
}

impl<'source> JumpStatement<'source> {
    /// Constructs a new JumpStatement with the given kind, optional value, and
    /// pos
    pub fn new(kind: JumpStatementKind, value: Option<Value>, pos: SourceRange<'source>) -> Self {
        Self { kind, value, pos }
    }

    /// Gets the kind of the JumpStatement
    pub fn kind(&self) -> JumpStatementKind {
        self.kind
    }

    /// Gets the stored value, consuming `self`
    pub fn into_value(self) -> Option<Value> {
        self.value
    }

    /// Gets the position in the source code of the jump statement
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}

impl Clone for JumpStatement<'_> {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            value: self.value.as_ref().map(Value::deep_copy),
            pos: self.pos.clone(),
        }
    }
}
