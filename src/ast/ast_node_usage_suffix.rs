use std::fmt::Display;

use crate::source_range::SourceRange;

use super::{ast_node_format::format_as_node, *};

/// A variable access AST node
#[derive(Debug)]
pub struct AstNodeUsageSuffix<'source> {
    root: Box<AstNode<'source>>,
    suffixes: Vec<UsageSuffix<'source>>,
    pos: SourceRange<'source>,
}

impl<'source> AstNodeUsageSuffix<'source> {
    /// Constructs a new AstNodeUsageSuffix with the given root and suffixes
    pub fn new(root: AstNode<'source>, suffixes: Vec<UsageSuffix<'source>>) -> Self {
        let pos = if suffixes.is_empty() {
            root.pos().clone()
        } else {
            root.pos().extended_to(suffixes[suffixes.len() - 1].pos())
        };
        Self {
            root: Box::new(root),
            suffixes,
            pos,
        }
    }

    /// Appends a suffix to this AstNodeUsageSuffix, updating the position as
    /// well
    pub fn append_suffix(&mut self, suffix: UsageSuffix<'source>) {
        self.pos.extend_to(suffix.pos());
        self.suffixes.push(suffix);
    }

    /// Returns the root of the usage suffix node
    pub fn root(&self) -> &AstNode {
        &self.root
    }

    /// Returns the suffixes of the usage suffix node
    pub fn suffixes(&self) -> &Vec<UsageSuffix<'source>> {
        &self.suffixes
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
        let mut node_children = Vec::with_capacity(1 + self.suffixes.len());
        node_children.push(format_as_node("Root", vec![self.root.to_string()]));
        for suffix in &self.suffixes {
            node_children.push(match suffix {
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
            });
        }
        write!(f, "{}", format_as_node("[Usage Suffix]", node_children))
    }
}

impl PartialEq for AstNodeUsageSuffix<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.suffixes == other.suffixes
    }
}
impl Eq for AstNodeBinaryOperation<'_> {}
