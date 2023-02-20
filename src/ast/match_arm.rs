use crate::source_range::SourceRange;

use super::*;

/// One arm in a match statement
#[derive(Debug, Clone)]
pub struct MatchArm<'source> {
    pattern: Box<AstNode<'source>>,
    body: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> MatchArm<'source> {
    /// Constructs a new MatchArm from the given pattern, body, and pos
    pub fn new(
        pattern: AstNode<'source>,
        body: Vec<AstNode<'source>>,
        pos: SourceRange<'source>,
    ) -> Self {
        Self {
            pattern: Box::new(pattern),
            body,
            pos,
        }
    }

    /// Returns the pattern of the match arm
    pub fn pattern(&self) -> &AstNode<'source> {
        &self.pattern
    }

    /// Returns the body of the match arm
    pub fn body(&self) -> &Vec<AstNode<'source>> {
        &self.body
    }

    /// Returns the position in the source code of this match arm
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// match arm
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl PartialEq for MatchArm<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern && self.body == other.body
    }
}
impl Eq for MatchArm<'_> {}
