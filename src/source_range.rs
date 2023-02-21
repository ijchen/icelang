//! Contains code related to source ranges

use std::fmt::{Debug, Display};

/// Represents a contiguous range of characters in some icelang source code
#[derive(Clone)]
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

    /// Returns the entire source that this error occurred in
    pub fn entire_source(&self) -> &'source str {
        self.entire_source
    }

    /// Returns the file name of the source that this error occurred in
    pub fn source_file_name(&self) -> &'source str {
        self.source_file_name
    }

    /// Returns the slice from the entire source corresponding to this range
    pub fn read(&self) -> &'source str {
        let start_byte_index = self
            .entire_source
            .char_indices()
            .nth(self.start_index)
            .unwrap()
            .0;
        let end_byte_index = self
            .entire_source
            .char_indices()
            .map(|(byte_index, _)| byte_index)
            .nth(self.end_index + 1)
            .unwrap_or(self.entire_source.len())
            - 1;
        &self.entire_source[start_byte_index..=end_byte_index]
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
        let mut col = 1;
        for (curr_index, ch) in self.entire_source.chars().enumerate() {
            if curr_index == index {
                return col;
            }

            if ch == '\n' {
                col = 1;
            } else {
                col += 1;
            }
        }

        panic!("Index out of bounds");
    }

    /// Returns the (1-indexed) column number that this range starts on
    pub fn start_col(&self) -> usize {
        self.col_of(self.start_index)
    }

    /// Returns the (1-indexed) column number that this range ends on
    pub fn end_col(&self) -> usize {
        self.col_of(self.end_index)
    }

    /// Extends the end of this SourceRange to match the end position of the
    /// other SourceRange
    pub fn extend_to(&mut self, other: &SourceRange) {
        self.end_index = other.end_index;
    }

    /// Returns a clone of this SourceRange with the end position extended to
    /// the end of the source
    pub fn extended_to_end(&self) -> Self {
        SourceRange {
            entire_source: self.entire_source,
            source_file_name: self.source_file_name,
            start_index: self.start_index,
            end_index: self.entire_source.chars().count() - 1,
        }
    }

    /// Returns a clone of this SourceRange with the end position extended to
    /// match the end of the other SourceRange
    pub fn extended_to(&self, other: &SourceRange) -> Self {
        SourceRange {
            entire_source: self.entire_source,
            source_file_name: self.source_file_name,
            start_index: self.start_index,
            end_index: other.end_index,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        SourceRange::new("print(\"Hello, world!\");\n", "hello.ice", 6, 20);
    }

    #[test]
    #[should_panic]
    fn test_new_start_after_end() {
        SourceRange::new("print(\"Hello, world!\");\n", "hello.ice", 20, 6);
    }

    #[test]
    #[should_panic]
    fn test_new_end_after_source() {
        SourceRange::new("print(\"Hello, world!\");\n", "hello.ice", 14, 24);
    }

    #[test]
    fn test_read() {
        let srs = [
            (("a", "a.ice", 0, 0), "a"),
            (("abcdefgh", "alphabet.ice", 2, 6), "cdefg"),
            (("a★bc★efgh", "alphabet.ice", 3, 7), "c★efg"),
            (("a★bc★", "alphabet.ice", 1, 4), "★bc★"),
            (
                ("print(\"Hello, world!\");\n", "hello.ice", 6, 20),
                "\"Hello, world!\"",
            ),
            (
                (
                    r#"// A simple program to print the first 10 Fibonacci numbers

let a = 0;
let b = 1;

loop 10 {
    println(a);

    let c = a;
    a = b;
    b += c;
};
"#,
                    "fib.ice",
                    98,
                    104,
                ),
                "println",
            ),
        ];

        for ((code, file, s, e), val) in srs {
            let sr = SourceRange::new(code, file, s, e);

            assert_eq!(sr.read(), val);
        }
    }

    #[test]
    fn test_lines() {
        let srs = [
            (("a", "a.ice", 0, 0), (1, 1)),
            (
                (
                    "abc\nd\refghij\r\nkl\nm\nno\n\r\n\r\npqr\nstuvw\nxyz\n",
                    "alphabet.ice",
                    15,
                    20,
                ),
                (3, 5),
            ),
            (
                (
                    r#"// A simple program to print the first 10 Fibonacci numbers

let a = 0;
let b = 1;

loop 10 {
    println(a);

    let c = a;
    a = b;
    b += c;
};
"#,
                    "fib.ice",
                    98,
                    104,
                ),
                (7, 7),
            ),
        ];

        for ((code, file, s, e), val) in srs {
            let sr = SourceRange::new(code, file, s, e);

            assert_eq!((sr.start_line(), sr.end_line()), val);
        }
    }

    #[test]
    fn test_cols() {
        let srs = [
            (("a", "a.ice", 0, 0), (1, 1)),
            (
                (
                    "abc\nd\refghij\r\nkl\nm\nno\n\r\n\r\npqr\nstuvw\nxyz\n",
                    "alphabet.ice",
                    9,
                    15,
                ),
                (6, 2),
            ),
            (
                (
                    r#"// A simple program to print the first 10 Fibonacci numbers

let a = 0;
let b = 1;

loop 10 {
    println(a);

    let c = a;
    a = b;
    b += c;
};
"#,
                    "fib.ice",
                    98,
                    134,
                ),
                (5, 9),
            ),
        ];

        for ((code, file, s, e), val) in srs {
            let sr = SourceRange::new(code, file, s, e);

            assert_eq!((sr.start_col(), sr.end_col()), val);
        }
    }

    #[test]
    fn test_debug() {
        let srs = [
            (
                ("a", "a.ice", 0, 0),
                "SourceRange { a.ice line 1, col 1: \"a\" }",
            ),
            (
                (
                    "abc\nd\refghij\r\nkl\nm\nno\n\r\n\r\npqr\nstuvw\nxyz\n",
                    "alphabet.ice",
                    9,
                    15,
                ),
                "SourceRange { alphabet.ice line 2, col 6 to line 3, col 2: \"hij\\r\\nkl\" }",
            ),
            (
                (
                    r#"// A simple program to print the first 10 Fibonacci numbers

let a = 0;
let b = 1;

loop 10 {
    println(a);

    let c = a;
    a = b;
    b += c;
};
"#,
                    "fib.ice",
                    98,
                    104,
                ),
                "SourceRange { fib.ice line 7, col 5 to 11: \"println\" }",
            ),
        ];

        for ((code, file, s, e), val) in srs {
            let sr = SourceRange::new(code, file, s, e);

            assert_eq!(format!("{sr:?}"), val);
        }
    }

    #[test]
    fn test_display() {
        let srs = [
            (("a", "a.ice", 0, 0), "a.ice line 1, col 1"),
            (
                (
                    "abc\nd\refghij\r\nkl\nm\nno\n\r\n\r\npqr\nstuvw\nxyz\n",
                    "alphabet.ice",
                    9,
                    15,
                ),
                "alphabet.ice line 2, col 6 to line 3, col 2",
            ),
            (
                (
                    r#"// A simple program to print the first 10 Fibonacci numbers

let a = 0;
let b = 1;

loop 10 {
    println(a);

    let c = a;
    a = b;
    b += c;
};
"#,
                    "fib.ice",
                    98,
                    104,
                ),
                "fib.ice line 7, col 5 to 11",
            ),
        ];

        for ((code, file, s, e), val) in srs {
            let sr = SourceRange::new(code, file, s, e);

            assert_eq!(format!("{sr}"), val);
        }
    }
}
