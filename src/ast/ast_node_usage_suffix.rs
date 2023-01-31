use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A variable access AST node
#[derive(Debug)]
pub struct AstNodeUsageSuffix<'source> {
    root: Box<AstNode<'source>>,
    suffix: UsageSuffix<'source>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeUsageSuffix<'source> {
    /// Constructs a new AstNodeUsageSuffix with the given root and suffix
    pub fn new(root: AstNode<'source>, suffix: UsageSuffix<'source>) -> Self {
        let pos = root.pos().extended_to(suffix.pos());
        Self {
            root: Box::new(root),
            suffix,
            pos,
        }
    }

    /// Returns the root of the usage suffix node
    pub fn root(&self) -> &AstNode {
        &self.root
    }

    /// Returns the suffixes of the usage suffix node
    pub fn suffix(&self) -> &UsageSuffix<'source> {
        &self.suffix
    }

    /// Returns the position in the source code of this usage suffix node
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }

    /// Returns a mutable reference to the position in the source code of this
    /// usage suffix node
    pub fn pos_mut(&mut self) -> &mut SourceRange<'source> {
        &mut self.pos
    }
}

impl Display for AstNodeUsageSuffix<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_as_node(
                "[Usage Suffix]",
                vec![
                    format_as_node("Root", vec![self.root.to_string()],),
                    match &self.suffix {
                        UsageSuffix::DotMemberAccess(suffix) => format_as_node(
                            &format!("Dot member access of property \"{}\"", suffix.member()),
                            vec![],
                        ),
                        UsageSuffix::ComputedMemberAccess(suffix) => format_as_node(
                            "Computed member access",
                            vec![suffix.member_node().to_string()],
                        ),
                        UsageSuffix::FunctionCall(suffix) => {
                            if suffix.arguments().is_empty() {
                                format_as_node("Function call (no arguments)", vec![])
                            } else {
                                format_as_node(
                                    "Function call  with arguments:",
                                    suffix.arguments().iter().map(AstNode::to_string).collect(),
                                )
                            }
                        }
                    }
                ]
            )
        )
    }
}

impl PartialEq for AstNodeUsageSuffix<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.suffix == other.suffix
    }
}
impl Eq for AstNodeUsageSuffix<'_> {}
