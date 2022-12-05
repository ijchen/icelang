use std::fmt::Display;

use static_assertions::const_assert;

use crate::source_range::SourceRange;

#[derive(Clone, Copy)]
pub enum IceErrorType {
    Syntax,
}

impl Display for IceErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax => write!(f, "Syntax"),
        }
    }
}

pub struct StackTrace<'source> {
    sources: Vec<(String, SourceRange<'source>)>,
}

impl Display for StackTrace<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Stack trace:")?;

        for source in self.sources.iter() {
            writeln!(f, "^ {} {}", source.0, source.1)?;
        }

        Ok(())
    }
}

const PREFIX: &str = "| ";
const MAX_LEN: usize = 80;
const_assert!(PREFIX.len() < MAX_LEN);

fn display_header(
    f: &mut impl std::fmt::Write,
    error_type: IceErrorType,
    description: &str,
) -> std::fmt::Result {
    const MULTILINE_HEADER_INDENT: &str = "  ";
    const_assert!(PREFIX.len() + MULTILINE_HEADER_INDENT.len() < MAX_LEN);
    let mut header_buff = String::with_capacity(MAX_LEN);
    header_buff.push_str(&format!("{error_type} Error: "));
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

fn display_source_highlight(f: &mut impl std::fmt::Write, pos: &SourceRange) -> std::fmt::Result {
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
            .take(original_start_column + 1)
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

pub fn display(
    f: &mut impl std::fmt::Write,
    error_type: IceErrorType,
    description: &str,
    pos: &SourceRange<'_>,
    stack_trace: Option<StackTrace>,
) -> std::fmt::Result {
    // Error message header
    display_header(f, error_type, description)?;

    // Error location
    writeln!(f, "{PREFIX}{pos}")?;
    writeln!(f, "{PREFIX}")?;

    // Error source highlight
    display_source_highlight(f, pos)?;

    // Stack trace (optional)
    if let Some(_unused_todo) = stack_trace {
        todo!()
    }

    Ok(())
}
