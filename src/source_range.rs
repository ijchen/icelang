use std::fmt::Debug;

/// Represents a contiguous range of characters in some ice source code
pub struct SourceRange<'source> {
    entire_source: &'source str,
    source_file_name: &'source str,
    start_index: usize, // inclusive
    end_index: usize,   // inclusive
}

impl<'source> SourceRange<'source> {
    /// Constructs a new SourceRange
    pub fn new(
        entire_source: &'source str,
        source_file_name: &'source str,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        Self {
            entire_source,
            source_file_name,
            start_index,
            end_index,
        }
    }

    /// Returns the slice from the entire source corresponding to this range
    pub fn read(&self) -> &'source str {
        &self.entire_source[self.start_index..self.end_index + 1]
    }
}

impl Debug for SourceRange<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SourceRange {{ in {} from {} to {}: \"{:?}\" }}",
            self.source_file_name,
            self.start_index,
            self.end_index,
            self.read()
        )
    }
}
