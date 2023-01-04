use crate::source_range::SourceRange;

use super::*;

/// A usage suffix
#[derive(Debug, PartialEq, Eq)]
pub enum UsageSuffix<'source> {
    /// A dot member access usage suffix (like `foo.bar`)
    DotMemberAccess(UsageSuffixDotMemberAccess<'source>),

    /// A computed member access usage suffix (like `foo[bar]`)
    ComputedMemberAccess(UsageSuffixComputedMemberAccess<'source>),

    /// A function call usage suffix (like `foo()`)
    FunctionCall(UsageSuffixFunctionCall<'source>),
}

impl<'source> UsageSuffix<'source> {
    /// Returns the position in the source code of this usage suffix
    pub fn pos(&self) -> &SourceRange<'source> {
        match self {
            UsageSuffix::DotMemberAccess(suffix) => suffix.pos(),
            UsageSuffix::ComputedMemberAccess(suffix) => suffix.pos(),
            UsageSuffix::FunctionCall(suffix) => suffix.pos(),
        }
    }
}

#[derive(Debug)]
/// A dot member access usage suffix
pub struct UsageSuffixDotMemberAccess<'source> {
    member: String,
    pos: SourceRange<'source>,
}
impl<'source> UsageSuffixDotMemberAccess<'source> {
    /// Constructs a new UsageSuffixDotMemberAccess
    pub fn new(member: String, pos: SourceRange<'source>) -> Self {
        Self { member, pos }
    }

    /// Returns the member to access
    pub fn member(&self) -> &str {
        &self.member
    }

    /// Returns the position in the source code of this
    /// UsageSuffixDotMemberAccess
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}
impl PartialEq for UsageSuffixDotMemberAccess<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.member == other.member
    }
}
impl Eq for UsageSuffixDotMemberAccess<'_> {}

#[derive(Debug)]
/// A computed member access usage suffix
pub struct UsageSuffixComputedMemberAccess<'source> {
    member_node: AstNode<'source>,
    pos: SourceRange<'source>,
}
impl<'source> UsageSuffixComputedMemberAccess<'source> {
    /// Constructs a new UsageSuffixComputedMemberAccess
    pub fn new(member_node: AstNode<'source>, pos: SourceRange<'source>) -> Self {
        Self { member_node, pos }
    }

    /// Returns the node representing the member to access
    pub fn member_node(&self) -> &AstNode<'source> {
        &self.member_node
    }

    /// Returns the position in the source code of this
    /// UsageSuffixComputedMemberAccess
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}
impl PartialEq for UsageSuffixComputedMemberAccess<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.member_node == other.member_node
    }
}
impl Eq for UsageSuffixComputedMemberAccess<'_> {}

#[derive(Debug)]
/// A function call usage suffix
pub struct UsageSuffixFunctionCall<'source> {
    arguments: Vec<AstNode<'source>>,
    pos: SourceRange<'source>,
}
impl<'source> UsageSuffixFunctionCall<'source> {
    /// Constructs a new UsageSuffixFunctionCall
    pub fn new(arguments: Vec<AstNode<'source>>, pos: SourceRange<'source>) -> Self {
        Self { arguments, pos }
    }

    /// Returns the arguments to the function call
    pub fn arguments(&self) -> &Vec<AstNode<'source>> {
        &self.arguments
    }

    /// Returns the position in the source code of this UsageSuffixFunctionCall
    pub fn pos(&self) -> &SourceRange<'source> {
        &self.pos
    }
}
impl PartialEq for UsageSuffixFunctionCall<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.arguments == other.arguments
    }
}
impl Eq for UsageSuffixFunctionCall<'_> {}

macro_rules! impl_from_specific_usage_suffix {
    ($specific_type:ident, $variant_name:ident) => {
        impl<'source> From<$specific_type<'source>> for UsageSuffix<'source> {
            fn from(token: $specific_type<'source>) -> Self {
                Self::$variant_name(token)
            }
        }
    };
}
impl_from_specific_usage_suffix!(UsageSuffixDotMemberAccess, DotMemberAccess);
impl_from_specific_usage_suffix!(UsageSuffixComputedMemberAccess, ComputedMemberAccess);
impl_from_specific_usage_suffix!(UsageSuffixFunctionCall, FunctionCall);
