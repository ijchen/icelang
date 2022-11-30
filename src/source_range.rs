use std::fmt::{Debug, Display};

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
        assert!(start_index <= end_index);
        assert!(end_index < entire_source.len());

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

    /// Returns the (1-indexed) line number that a character (by index) is on
    fn line_of(&self, index: usize) -> usize {
        self.entire_source
            .chars()
            .take(index)
            .filter(|&c| c == '\n')
            .count()
            + 1
    }

    /// Returns the (1-indexed) line number that this range starts on
    pub fn start_line(&self) -> usize {
        self.line_of(self.start_index)
    }

    /// Returns the (1-indexed) line number that this range ends on
    pub fn end_line(&self) -> usize {
        self.line_of(self.end_index)
    }

    /// Returns the (1-indexed) column number that a character (by index) is on
    fn col_of(&self, index: usize) -> usize {
        // TODO this could probably be optimized

        let mut col = 1;
        let mut curr_index = 0;
        while curr_index < index {
            if self.entire_source.chars().nth(curr_index).unwrap() == '\n' {
                if self.entire_source.chars().nth(curr_index + 1) == Some('\r') {
                    curr_index += 1;
                }
                col = 1;
            } else {
                col += 1;
            }

            curr_index += 1;
        }

        col
    }

    /// Returns the (1-indexed) column number that this range starts on
    pub fn start_col(&self) -> usize {
        self.col_of(self.start_index)
    }

    /// Returns the (1-indexed) column number that this range ends on
    pub fn end_col(&self) -> usize {
        self.col_of(self.end_index)
    }
}

impl Debug for SourceRange<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceRange {{ {self}: {:?} }}", self.read())
    }
}

impl Display for SourceRange<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_line = self.start_line();
        let end_line = self.end_line();
        let start_col = self.start_col();
        let end_col = self.end_col();

        match (start_line == end_line, start_col == end_col) {
            (true, true) => write!(
                f,
                "{} line {}, col {}",
                self.source_file_name, start_line, start_col
            ),
            (true, false) => write!(
                f,
                "{} line {}, col {} to {}",
                self.source_file_name, start_line, start_col, end_col
            ),
            (false, _) => write!(
                f,
                "{} line {}, col {} to line {}, col {}",
                self.source_file_name, start_line, start_col, end_line, end_col
            ),
        }
    }
}
