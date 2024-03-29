mod args;
mod debug_info;
mod repl;

use std::{collections::VecDeque, fs};

use clap::Parser;
use icelang::{interpreter, lexer, parser};
use repl::enter_repl;

use crate::debug_info::print_source_info;

fn interpret_file(file_path: &str, show_debug_info: bool) {
    let Ok(source_code) = fs::read_to_string(file_path) else {
        eprintln!("Couldn't read file \"{file_path}\"");
        return;
    };

    // If debug info is enabled, print source code information
    if show_debug_info {
        println!();
        print_source_info(file_path, &source_code);
        println!();
    }

    // Lexer
    let tokens = match lexer::tokenize(&source_code, file_path) {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // If debug info is enabled, print token information
    if show_debug_info {
        println!("Tokens:");
        for token in &tokens {
            println!("\t{token}");
        }
        println!();
    }

    // Parsing
    let ast = match parser::parse(tokens.iter().collect::<VecDeque<_>>()) {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // If debug info is enabled, print the AST
    if show_debug_info {
        println!("AST:");
        println!("{ast}");
        println!();
    }

    // Interpreting
    let state = match interpreter::interpret(&ast) {
        Ok(state) => state,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // If debug info is enabled, print the runtime state
    if show_debug_info {
        println!("Interpreter State:");
        println!("{state}");
        println!();
    }
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
