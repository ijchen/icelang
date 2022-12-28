use std::{collections::VecDeque, fmt::Display};

use static_assertions::const_assert;

use crate::source_range::SourceRange;

#[derive(Clone, Copy, Debug)]
pub enum IcelangErrorKind {
    Syntax,
    // TODO remove once this is used
    #[allow(dead_code)]
    Runtime,
}

impl Display for IcelangErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax => write!(f, "Syntax"),
            Self::Runtime => write!(f, "Runtime"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StackTrace<'source> {
    sources: VecDeque<(String, SourceRange<'source>)>,
}

impl<'source> StackTrace<'source> {
    // TODO remove once this is used
    #[allow(dead_code)]
    /// Constructs a new (empty) StackTrace
    pub fn new() -> Self {
        Self {
            sources: VecDeque::new(),
        }
    }

    // TODO remove once this is used
    #[allow(dead_code)]
    /// Adds a stack frame to the top of the StackTrace
    pub fn add_top(&mut self, source_fn_display_name: String, source_range: SourceRange<'source>) {
        self.sources
            .push_front((source_fn_display_name, source_range));
    }

    // TODO remove once this is used
    #[allow(dead_code)]
    /// Adds a stack frame to the bottom of the StackTrace
    pub fn add_bottom(
        &mut self,
        source_fn_display_name: String,
        source_range: SourceRange<'source>,
    ) {
        self.sources
            .push_back((source_fn_display_name, source_range));
    }
}

impl Display for StackTrace<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Stack trace (most recent call at the top):")?;

        if self.sources.is_empty() {
            writeln!(f, "<empty>")
        } else {
            for (source_fn_display_name, source_range) in self.sources.iter() {
                // TODO trim source_fn_display_name if necessary to respect
                // MAX_LEN (don't forget to update unit tests)
                writeln!(f, "^ {source_fn_display_name} {source_range}")?;
            }

            Ok(())
        }
    }
}

const PREFIX: &str = "| ";
const MAX_LEN: usize = 80;
const_assert!(PREFIX.len() < MAX_LEN);

fn write_header(
    f: &mut impl std::fmt::Write,
    error_kind: IcelangErrorKind,
    description: &str,
) -> std::fmt::Result {
    const MULTILINE_HEADER_INDENT: &str = "  ";
    const_assert!(PREFIX.len() + MULTILINE_HEADER_INDENT.len() < MAX_LEN);
    let mut header_buff = String::with_capacity(MAX_LEN);
    header_buff.push_str(&format!("{error_kind} Error: "));
    let mut multiline_header = false;
    let mut index = 0;
    let chars: Vec<char> = description.chars().collect();
    while index < chars.len() {
        if chars[index] == '\r' && chars.get(index + 1) == Some(&'\n') {
            index += 1;
        }
        if chars[index] == '\n' {
            index += 1;
            multiline_header = true;
            writeln!(f, "{header_buff}")?;
            header_buff.clear();
            header_buff.push_str(PREFIX);
            header_buff.push_str(MULTILINE_HEADER_INDENT);
            continue;
        }

        if header_buff.len() >= MAX_LEN {
            assert!(header_buff.len() == MAX_LEN);
            multiline_header = true;
            writeln!(f, "{header_buff}")?;
            header_buff.clear();
            header_buff.push_str(PREFIX);
            header_buff.push_str(MULTILINE_HEADER_INDENT);
        }

        header_buff.push(chars[index]);
        index += 1;
    }
    if header_buff.len() > PREFIX.len() {
        writeln!(f, "{header_buff}")?;
    }

    if multiline_header {
        writeln!(f, "{PREFIX}")?;
    }

    Ok(())
}

fn write_source_highlight(f: &mut impl std::fmt::Write, pos: &SourceRange) -> std::fmt::Result {
    // TODO Strip leading whitespace on a line ("        error();" becomes "error();")

    // Convenience variables
    let start_line_number = pos.start_line();
    let original_start_column = pos.start_col() - 1;
    let original_end_column = pos.end_col() - 1;

    // Get the line with the error
    let original_line: &str = pos
        .entire_source()
        .lines()
        .nth(start_line_number - 1)
        .unwrap();

    // Since tabs usually render as more than one character, they would misalign
    // the highlight. To work around this, we will replace all tab characters
    // with four spaces, adjusting the start and end columns accordingly
    let adj_line = original_line.replace('\t', "    ");
    let adj_start_column = original_start_column
        + original_line
            .chars()
            .take(original_start_column)
            .filter(|&c| c == '\t')
            .count()
            * 3;
    let adj_end_column = original_end_column
        + original_line
            .chars()
            .take(original_end_column + 1)
            .filter(|&c| c == '\t')
            .count()
            * 3;

    // Constant definitions that control the appearance and structure of error messages
    const TRIM_CHARS: &str = "..."; // Indicates that the end of the line was trimmed off
    const CUTOUT_SEP: &str = " ... "; // Indicates that code between the start of the line and the error was cut out
    const ARR_CHAR: &str = "^"; // The arrows that point out the error
    const SPILL_CHARS: &str = ">>>"; // Go beneath the trim characters to indicates that the error itself spills past the end of the line
    const SPACING_CHAR: &str = " "; // Fills "blank space" to line up the symbols in the arrow line
    const MAX_LEN_AFTER_PREFIX: usize = MAX_LEN - PREFIX.len(); // The maximum length of the line (excluding the error message prefix)
    const CUTOUT_LINE_START_LEN: usize = 20; // The number of characters to show at the start of the error line before the cut-out
    const CUTOUT_LEADUP_LEN: usize = 15; // The number of characters to show after the cut-out but before the start of the error

    const_assert!(TRIM_CHARS.len() == SPILL_CHARS.len());
    const_assert!(SPACING_CHAR.len() == 1);
    const_assert!(
        MAX_LEN_AFTER_PREFIX
            >= CUTOUT_LINE_START_LEN + CUTOUT_SEP.len() + CUTOUT_LEADUP_LEN + TRIM_CHARS.len()
    );

    // Calculate frequently-used lengths and positions
    let len_line: usize = adj_line.len(); // The length of the line (after modifications like stripping leading whitespace)
    let len_before: usize = adj_start_column; // The number of characters before the error
    let len_err: usize = adj_end_column - adj_start_column + 1; // The number of characters in the error
    let len_after = len_line - adj_end_column - 1; // The number of characters in the line after the error

    // String to output
    let out_line: String; // The line of code with the error
    let out_err: String; // The arrows beneath pointing out the error

    // If the line fits without modification, perfect!
    if len_line <= MAX_LEN_AFTER_PREFIX {
        out_line = adj_line;
        out_err = format!(
            "{}{}",
            SPACING_CHAR.repeat(len_before),
            ARR_CHAR.repeat(len_err)
        );
    }
    // If the line won't fit without modification, can we just trim the end off?
    else if len_before + len_err + TRIM_CHARS.len() <= MAX_LEN_AFTER_PREFIX {
        out_line = format!(
            "{}{TRIM_CHARS}",
            &adj_line
                .chars()
                .take(MAX_LEN_AFTER_PREFIX - TRIM_CHARS.len())
                .collect::<String>()
        );
        out_err = format!(
            "{}{}{}{TRIM_CHARS}",
            SPACING_CHAR.repeat(len_before),
            ARR_CHAR.repeat(len_err),
            SPACING_CHAR.repeat(MAX_LEN_AFTER_PREFIX - (len_before + len_err + TRIM_CHARS.len()))
        );
    }
    // If the line won't fit with just the end trimmed, can we fit the error with just a cutout
    // from the middle?
    else if CUTOUT_LINE_START_LEN + CUTOUT_SEP.len() + CUTOUT_LEADUP_LEN + len_err + len_after
        <= MAX_LEN_AFTER_PREFIX
    {
        out_line = format!(
            "{}{CUTOUT_SEP}{}",
            &adj_line
                .chars()
                .take(CUTOUT_LINE_START_LEN)
                .collect::<String>(),
            &adj_line
                .chars()
                .skip(len_before - CUTOUT_LEADUP_LEN)
                .collect::<String>(),
        );
        out_err = format!(
            "{}{CUTOUT_SEP}{}{}",
            SPACING_CHAR.repeat(CUTOUT_LINE_START_LEN),
            SPACING_CHAR.repeat(CUTOUT_LEADUP_LEN),
            ARR_CHAR.repeat(len_err)
        );
    }
    // Our last chance to avoid the error itself spilling over is to cutout AND trim the end...
    // will doing that still fit the error itself?
    else if CUTOUT_LINE_START_LEN
        + CUTOUT_SEP.len()
        + CUTOUT_LEADUP_LEN
        + len_err
        + TRIM_CHARS.len()
        <= MAX_LEN_AFTER_PREFIX
    {
        let len_shown_after_err = MAX_LEN_AFTER_PREFIX
            - (CUTOUT_LINE_START_LEN
                + CUTOUT_SEP.len()
                + CUTOUT_LEADUP_LEN
                + len_err
                + TRIM_CHARS.len());
        out_line = format!(
            "{}{CUTOUT_SEP}{}{TRIM_CHARS}",
            &adj_line
                .chars()
                .take(CUTOUT_LINE_START_LEN)
                .collect::<String>(),
            &adj_line
                .chars()
                .skip(len_before - CUTOUT_LEADUP_LEN)
                .take(CUTOUT_LEADUP_LEN + len_err + len_shown_after_err)
                .collect::<String>(),
        );
        out_err = format!(
            "{}{CUTOUT_SEP}{}{}{}{TRIM_CHARS}",
            SPACING_CHAR.repeat(CUTOUT_LINE_START_LEN),
            SPACING_CHAR.repeat(CUTOUT_LEADUP_LEN),
            ARR_CHAR.repeat(len_err),
            SPACING_CHAR.repeat(len_shown_after_err)
        );
    }
    // There is no way to avoid the error itself spilling past the end of the line. Can we at
    // least avoid a cutout between the start of the line and the error itself?
    else if len_before <= CUTOUT_LINE_START_LEN + CUTOUT_SEP.len() + CUTOUT_LEADUP_LEN {
        out_line = format!(
            "{}{TRIM_CHARS}",
            &adj_line
                .chars()
                .take(MAX_LEN_AFTER_PREFIX - TRIM_CHARS.len())
                .collect::<String>(),
        );
        out_err = format!(
            "{}{}{}",
            SPACING_CHAR.repeat(len_before),
            ARR_CHAR.repeat(MAX_LEN_AFTER_PREFIX - (len_before + TRIM_CHARS.len())),
            SPILL_CHARS
        );
    }
    // This is the worst-case scenario. We need to cut out some code between the start of the
    // line and the start of the error, and the error will *still* spill past the end of the
    // line. Ohh well. This is the point at which the icelang programmer should probably refactor
    // their code.
    else {
        let len_of_shown_err = MAX_LEN_AFTER_PREFIX
            - (CUTOUT_LINE_START_LEN + CUTOUT_SEP.len() + CUTOUT_LEADUP_LEN + TRIM_CHARS.len());
        out_line = format!(
            "{}{CUTOUT_SEP}{}{TRIM_CHARS}",
            &adj_line
                .chars()
                .take(CUTOUT_LINE_START_LEN)
                .collect::<String>(),
            &adj_line
                .chars()
                .skip(len_before - CUTOUT_LEADUP_LEN)
                .take(CUTOUT_LEADUP_LEN + len_of_shown_err)
                .collect::<String>(),
        );
        out_err = format!(
            "{}{CUTOUT_SEP}{}{}{}",
            SPACING_CHAR.repeat(CUTOUT_LINE_START_LEN),
            SPACING_CHAR.repeat(CUTOUT_LEADUP_LEN),
            ARR_CHAR.repeat(len_of_shown_err),
            SPILL_CHARS
        );
    }

    write!(f, "{PREFIX}{out_line}\n{PREFIX}{out_err}")
}

pub fn write_error(
    f: &mut impl std::fmt::Write,
    error_kind: IcelangErrorKind,
    description: &str,
    pos: &SourceRange<'_>,
    stack_trace: Option<&StackTrace>,
) -> std::fmt::Result {
    // Error message header
    write_header(f, error_kind, description)?;

    // Error location
    writeln!(f, "{PREFIX}{pos}")?;
    writeln!(f, "{PREFIX}")?;

    // Error source highlight
    write_source_highlight(f, pos)?;

    // Stack trace (optional)
    if let Some(stack_trace) = stack_trace {
        writeln!(f)?;
        write!(f, "{PREFIX}")?;
        for stack_trace_line in stack_trace.to_string().lines() {
            writeln!(f)?;
            write!(f, "{PREFIX}{stack_trace_line}")?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icelang_error_kind_display() {
        assert_eq!(IcelangErrorKind::Syntax.to_string(), "Syntax");
        assert_eq!(IcelangErrorKind::Runtime.to_string(), "Runtime");
    }

    #[test]
    fn test_stack_trace_display_empty() {
        let my_stack_trace = StackTrace::new();

        assert_eq!(
            my_stack_trace.to_string(),
            "\
            Stack trace (most recent call at the top):\n\
            <empty>\n"
        );
    }

    #[test]
    fn test_stack_trace_display_single() {
        let mut my_stack_trace = StackTrace::new();

        let main_ice = (
            "main.ice",
            "\
fn foo() {
    for i in range(10) {
        println(31.0 / bar(i ** 2));
    }
}

fn bar(n) {
    return float(64 - n);
}

fn bat() {
    baz(6);

    println(\"Hello, world!\");

    baz(21);

    println(\"Hello again :)\");
}

fn baz(num) {
    if num > 10 {
        foo();
    }
}

bat();
",
        );

        my_stack_trace.add_top(
            "<global>".to_string(),
            SourceRange::new(main_ice.1, main_ice.0, 282, 286),
        );

        assert_eq!(
            my_stack_trace.to_string(),
            "\
            Stack trace (most recent call at the top):\n\
            ^ <global> main.ice line 27, col 1 to 5\n"
        );
    }

    #[test]
    fn test_stack_trace_display_multi() {
        let mut my_stack_trace = StackTrace::new();

        let main_ice = (
            "main.ice",
            "\
fn foo() {
    for i in range(10) {
        println(31.0 / bar(i ** 2));
    }
}

fn bar(n) {
    return float(64 - n);
}

fn bat() {
    baz(6);

    println(\"Hello, world!\");

    baz(21);

    println(\"Hello again :)\");
}

fn baz(num) {
    if num > 10 {
        foo();
    }
}

bat();
",
        );

        my_stack_trace.add_top(
            "<global>".to_string(),
            SourceRange::new(main_ice.1, main_ice.0, 282, 286),
        );
        my_stack_trace.add_top(
            "bat()".to_string(),
            SourceRange::new(main_ice.1, main_ice.0, 182, 188),
        );
        my_stack_trace.add_top(
            "baz(num)".to_string(),
            SourceRange::new(main_ice.1, main_ice.0, 266, 270),
        );
        my_stack_trace.add_top(
            "foo()".to_string(),
            SourceRange::new(main_ice.1, main_ice.0, 52, 69),
        );

        assert_eq!(
            my_stack_trace.to_string(),
            "\
            Stack trace (most recent call at the top):\n\
            ^ foo() main.ice line 3, col 17 to 34\n\
            ^ baz(num) main.ice line 23, col 9 to 13\n\
            ^ bat() main.ice line 16, col 5 to 11\n\
            ^ <global> main.ice line 27, col 1 to 5\n"
        );
    }

    #[test]
    fn test_write_header_single_line() {
        let mut header1 = String::with_capacity(27);
        write_header(&mut header1, IcelangErrorKind::Syntax, "Uh oh stinky").unwrap();
        assert_eq!(header1, "Syntax Error: Uh oh stinky\n");

        let mut header2 = String::with_capacity(57);
        write_header(
            &mut header2,
            IcelangErrorKind::Runtime,
            "I'm sorry Dave, I'm afraid I can't do that",
        )
        .unwrap();
        assert_eq!(
            header2,
            "Runtime Error: I'm sorry Dave, I'm afraid I can't do that\n"
        );

        let mut header3 = String::with_capacity(81);
        write_header(
            &mut header3,
            IcelangErrorKind::Runtime,
            "This is a pretty long message, but not *too* long (it's 80 chars)",
        )
        .unwrap();
        assert_eq!(
            header3,
            "Runtime Error: This is a pretty long message, but not *too* long (it's 80 chars)\n"
        );
    }

    #[test]
    fn test_write_header_embedded_newline() {
        let mut header1 = String::with_capacity(52);
        write_header(
            &mut header1,
            IcelangErrorKind::Syntax,
            "Uh oh stinky\nwith a newline...",
        )
        .unwrap();
        assert_eq!(
            header1,
            "Syntax Error: Uh oh stinky\n|   with a newline...\n| \n"
        );

        let mut header2 = String::with_capacity(83);
        write_header(
            &mut header2,
            IcelangErrorKind::Runtime,
            "I'm sorry Dave,\nI'm afraid\r\nI can't\ndo\rthat\n\r\n",
        )
        .unwrap();
        assert_eq!(
            header2,
            "Runtime Error: I'm sorry Dave,\n|   I'm afraid\n|   I can't\n|   do\rthat\n|   \n|   \n| \n"
        );
    }

    #[test]
    fn test_write_header_too_long() {
        let mut header1 = String::with_capacity(90);
        write_header(
            &mut header1,
            IcelangErrorKind::Runtime,
            "This is a pretty long message. In fact its just over 80 characters",
        )
        .unwrap();
        assert_eq!(
            header1,
            "Runtime Error: This is a pretty long message. In fact its just over 80 character\n|   s\n| \n"
        );

        let mut header2 = String::with_capacity(357);
        write_header(
            &mut header2,
            IcelangErrorKind::Syntax,
            "\
Here is my super long message. I hope you like it! I am using it t\
o unit test the multiline header error formatting for my programming languag\
e called 'icelang'. This long message will be split into many lines (this ->\
<- is where the third newline will be put!). Anyway, I hope you liked my mes\
sage. Have a great day :D",
        )
        .unwrap();
        assert_eq!(
            header2,
            "\
Syntax Error: Here is my super long message. I hope you like it! I am using it t
|   o unit test the multiline header error formatting for my programming languag
|   e called 'icelang'. This long message will be split into many lines (this ->
|   <- is where the third newline will be put!). Anyway, I hope you liked my mes
|   sage. Have a great day :D
| 
"
        );
    }

    #[test]
    fn test_write_header_too_long_and_embedded_newline() {
        let mut header1 = String::with_capacity(134);
        write_header(
            &mut header1,
            IcelangErrorKind::Syntax,
            "This has both a newline here:\nand a line that is too long (this one!) In fact, it's just over 80 characters",
        )
        .unwrap();
        assert_eq!(
            header1,
            "Syntax Error: This has both a newline here:\n|   and a line that is too long (this one!) In fact, it's just over 80 character\n|   s\n| \n"
        );

        let mut header2 = String::with_capacity(389);
        write_header(
            &mut header2,
            IcelangErrorKind::Syntax,
            "\
Here is my super long message. I hope you like it! I am using it t\
o unit test the multiline header error formatting for my programming languag\
e called 'icelang'. This long message will be split into many lines (this ->\
<- is where the third newline will be put!). Anyway, I hope you
liked\r\nmy message.\n
Have a great day :D",
        )
        .unwrap();
        assert_eq!(
            header2,
            "\
Syntax Error: Here is my super long message. I hope you like it! I am using it t
|   o unit test the multiline header error formatting for my programming languag
|   e called 'icelang'. This long message will be split into many lines (this ->
|   <- is where the third newline will be put!). Anyway, I hope you
|   liked
|   my message.
|   
|   Have a great day :D
| 
"
        );
    }

    #[test]
    fn test_write_source_highlight_simple() {
        let main_ice = (
            "main.ice",
            "\
fn foo() {
    for i in range(10) {
        println(31.0 / bar(i ** 2));
    }
}

fn bar(n) {
    return float(64 - n);
}

fn bat() {
    baz(6);

    println(\"Hello, world!\");

    baz(21);

    println(\"Hello again :)\");
}

fn baz(num) {
    if num > 10 {
        foo();
    }
}

bat();
",
        );

        let mut source_highlight = String::with_capacity(75);
        write_source_highlight(
            &mut source_highlight,
            &SourceRange::new(main_ice.1, main_ice.0, 52, 69),
        )
        .unwrap();

        assert_eq!(
            source_highlight,
            "\
|         println(31.0 / bar(i ** 2));
|                 ^^^^^^^^^^^^^^^^^^"
        );
    }

    #[test]
    fn test_write_source_highlight_single_start_tab() {
        let main_ice = ("main.ice", "\tprintln(\"Hello, world!\");");

        let mut source_highlight = String::with_capacity(61);
        write_source_highlight(
            &mut source_highlight,
            &SourceRange::new(main_ice.1, main_ice.0, 9, 23),
        )
        .unwrap();

        assert_eq!(
            source_highlight,
            "\
|     println(\"Hello, world!\");
|             ^^^^^^^^^^^^^^^"
        );
    }

    #[test]
    fn test_write_source_highlight_multiple_tabs() {
        let main_ice = ("main.ice", "\t\t  \t println\t(\"Hello,\t\\t\tworld!\"\t);");

        let mut source_highlight = String::with_capacity(113);
        write_source_highlight(
            &mut source_highlight,
            &SourceRange::new(main_ice.1, main_ice.0, 15, 32),
        )
        .unwrap();

        assert_eq!(
            source_highlight,
            "\
|                println    (\"Hello,    \\t    world!\"    );
|                            ^^^^^^^^^^^^^^^^^^^^^^^^"
        );
    }

    #[test]
    fn test_write_source_highlight_trim() {
        let main_ice = ("main.ice", "\tprintln(2 + 2); // This comment goes on for a long time, and will be trimmed around here (so the rest of this won't even be shown)");

        let mut source_highlight = String::with_capacity(161);
        write_source_highlight(
            &mut source_highlight,
            &SourceRange::new(main_ice.1, main_ice.0, 9, 13),
        )
        .unwrap();

        assert_eq!(
            source_highlight,
            "\
|     println(2 + 2); // This comment goes on for a long time, and will be tr...
|             ^^^^^                                                          ..."
        );
    }

    #[test]
    fn test_write_source_highlight_cutout() {
        let main_ice = ("main.ice", "\tprintln(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 100 / 0 + -1);");

        let mut source_highlight = String::with_capacity(106);
        write_source_highlight(
            &mut source_highlight,
            &SourceRange::new(main_ice.1, main_ice.0, 75, 81),
        )
        .unwrap();

        assert_eq!(
            source_highlight,
            "\
|     println(1 + 2 +  ... 13 + 14 + 15 + 100 / 0 + -1);
|                      ...                ^^^^^^^"
        );
    }

    #[test]
    fn test_write_source_highlight_trim_and_cutout() {
        let main_ice = ("main.ice", "\tprintln(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 100 / 0 + -1); // This comment is too long and will get cut off");

        let mut source_highlight = String::with_capacity(161);
        write_source_highlight(
            &mut source_highlight,
            &SourceRange::new(main_ice.1, main_ice.0, 75, 81),
        )
        .unwrap();

        assert_eq!(
            source_highlight,
            "\
|     println(1 + 2 +  ... 13 + 14 + 15 + 100 / 0 + -1); // This comment is t...
|                      ...                ^^^^^^^                            ..."
        );
    }

    #[test]
    fn test_write_source_highlight_spill() {
        let main_ice = ("main.ice", "println(100 / (0 - 0 + 1 - 1 + 2 - 2 + 3 - 3 + 4 - 4 + 5 - 5 + 6 - 6 + 7 - 7 + 8 - 8));");

        let mut source_highlight = String::with_capacity(161);
        write_source_highlight(
            &mut source_highlight,
            &SourceRange::new(main_ice.1, main_ice.0, 8, 84),
        )
        .unwrap();

        assert_eq!(
            source_highlight,
            "\
| println(100 / (0 - 0 + 1 - 1 + 2 - 2 + 3 - 3 + 4 - 4 + 5 - 5 + 6 - 6 + 7 - ...
|         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^>>>"
        );
    }

    #[test]
    fn test_write_source_highlight_err_spill_and_cutout() {
        let main_ice = ("main.ice", "println(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 100 / (0 - 0 + 1 - 1 + 2 - 2 + 3 - 3 + 4 - 4 + 5 - 5 + 6 - 6 + 7 - 7 + 8 - 8));");

        let mut source_highlight = String::with_capacity(161);
        write_source_highlight(
            &mut source_highlight,
            &SourceRange::new(main_ice.1, main_ice.0, 74, 150),
        )
        .unwrap();

        assert_eq!(
            source_highlight,
            "\
| println(1 + 2 + 3 +  ... 13 + 14 + 15 + 100 / (0 - 0 + 1 - 1 + 2 - 2 + 3 - ...
|                      ...                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^>>>"
        );
    }

    #[test]
    fn test_write_error_1() {
        let source = "\
fn foo() {
\tlet num = 0;
\tlet value = \"Hello, world!\\mNew line\";

\treturn value + \"hi\";
}
";
        let mut err = String::with_capacity(170);
        let err_kind = IcelangErrorKind::Syntax;
        let description = "invalid escape sequence in string literal";
        let pos = SourceRange::new(source, "main.ice", 52, 53);
        write_error(&mut err, err_kind, description, &pos, None).unwrap();
        assert_eq!(err, "\
Syntax Error: invalid escape sequence in string literal
| main.ice line 3, col 28 to 29
| 
|     let value = \"Hello, world!\\mNew line\";
|                               ^^");
    }

    #[test]
    fn test_write_error_2() {
        let source = "\
fn foo() {
\tlet num = 0;
\tlet value = (41 - 2 * 3) 17 * 2;

\treturn value * 3;
}           
";
        let mut err = String::with_capacity(154);
        let err_kind = IcelangErrorKind::Syntax;
        let description = "unexpected token";
        let pos = SourceRange::new(source, "unexpected_seventeen.ice", 51, 52);
        write_error(&mut err, err_kind, description, &pos, None).unwrap();
        assert_eq!(err, "\
Syntax Error: unexpected token
| unexpected_seventeen.ice line 3, col 27 to 28
| 
|     let value = (41 - 2 * 3) 17 * 2;
|                              ^^");
    }

    #[test]
    fn test_write_error_3() {
        let source = "\
fn foo() {
\tlet num = 0;
\tlet value = (2 + 4 - 2) / num + 1;

\treturn value * 3;
}

fn bar(num) {
\treturn f\"foo returned {foo()}, bar got {num}.\";
}

println(bar(87));       
";
        let source_file_name = "main.ice";
        let mut err = String::with_capacity(317);
        let err_kind = IcelangErrorKind::Runtime;
        let description = "division by zero";
        let pos = SourceRange::new(source, source_file_name, 38, 54);
        let mut stack_trace = StackTrace::new();
        stack_trace.add_bottom("foo()".to_string(), SourceRange::new(source, source_file_name, 38, 54));
        stack_trace.add_bottom("bar(num)".to_string(), SourceRange::new(source, source_file_name, 122, 126));
        stack_trace.add_bottom("<global>".to_string(), SourceRange::new(source, source_file_name, 158, 164));
        write_error(&mut err, err_kind, description, &pos, Some(&stack_trace)).unwrap();
        assert_eq!(err, "\
Runtime Error: division by zero
| main.ice line 3, col 14 to 30
| 
|     let value = (2 + 4 - 2) / num + 1;
|                 ^^^^^^^^^^^^^^^^^
| 
| Stack trace (most recent call at the top):
| ^ foo() main.ice line 3, col 14 to 30
| ^ bar(num) main.ice line 9, col 25 to 29
| ^ <global> main.ice line 12, col 9 to 15");
    }
}
