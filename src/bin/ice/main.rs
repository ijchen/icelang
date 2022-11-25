mod args;
mod repl;

use std::fs;

use clap::Parser;
use repl::enter_repl;

fn interpret_file(file_path: &str, show_debug_info: bool) {
    let Ok(source_code) = fs::read_to_string(file_path) else {
        eprintln!("Failed to read file at path: {file_path}");
        return;
    };

    match show_debug_info {
        // TODO
        true => println!("TODO: Interpret source code (with debug info):\n{source_code}"),
        // TODO
        false => println!("TODO: Interpret source code:\n{source_code}"),
    };
}

fn main() {
    // Parse command-line arguments
    let args = args::Args::parse();

    // If there was a file path, interpret it. If not, enter the read-eval-print
    // loop (REPL)
    match args.file_path {
        Some(file_path) => interpret_file(&file_path, args.debug_info),
        None => enter_repl(args.debug_info),
    };
}
