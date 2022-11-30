use std::fmt::Display;

use crate::source_range::SourceRange;

#[derive(Clone, Copy)]
pub enum IceErrorType {
    Syntax,
    Runtime,
}

pub struct StackTrace<'source> {
    sources: Vec<(String, SourceRange<'source>)>,
}

impl<'source> StackTrace<'source> {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn add_top(&mut self, scope_name: String, pos: SourceRange<'source>) {
        self.sources.push((scope_name, pos));
    }
}

impl Display for StackTrace<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Stack trace:")?;

        for source in self.sources.iter().rev() {
            writeln!(f, "^ {} {}", source.0, source.1)?;
        }

        std::fmt::Result::Ok(())
    }
}

fn display(
    f: &mut std::fmt::Formatter<'_>,
    error_type: IceErrorType,
    description: &str,
    pos: &SourceRange<'_>,
    stack_trace: Option<StackTrace>,
) -> std::fmt::Result {
    todo!()
}
