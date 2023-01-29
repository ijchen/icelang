use std::collections::VecDeque;

use icelang::{interpreter, lexer, parser, runtime_state::RuntimeState};
use rustyline::{error::ReadlineError, Editor};
use typed_arena::Arena;

use crate::debug_info::print_source_info;

const WELCOME_MESSAGE: &str = "\
icelang REPL - Welcome!
Heads up: icelang is still in development - expect changes as the language \
develops.
enter \"help\" for help, or \"exit\" to exit.";

const HELP_MESSAGE: &str = "\
You can type code directly into the terminal. It will be evaluated, and the \
resulting value will be printed out.

You can also use the following commands:
help                Display this help message
exit                Exit the REPL
clear               Clear the screen
restart             Restarts the REPL
debug               Toggles showing debug information";

const SOURCE_NAME: &str = "<stdin>";

pub fn enter_repl(mut show_debug_info: bool) {
    // Initialize readline editor
    let Ok(mut readline_editor) = Editor::<()>::new() else {
        eprintln!("Failed to initialize REPL terminal");
        return;
    };
    let mut last_input_was_ctrl_c = false;

    // Initialize the saved input lines arena
    // An arena is necessary since the interpreter state may maintain references
    // to slices from source code entered in the past, and the lifetime of those
    // references must be valid for the entire lifetime of the interpreter state
    let mut input_lines: Arena<String> = Arena::new();

    // Initialize a runtime state to use persistently in the REPL
    let mut state = RuntimeState::new();

    // Show welcome message
    println!("{WELCOME_MESSAGE}");

    // REPL
    loop {
        // Read in a string from stdin, or display an error message if something
        // goes wrong
        let source_code = match readline_editor.readline(">>> ") {
            Ok(line) => {
                readline_editor.add_history_entry(&line);
                last_input_was_ctrl_c = false;
                input_lines.alloc(line)
            }
            Err(ReadlineError::Interrupted) => {
                if last_input_was_ctrl_c {
                    println!("Nevermind, it seems like you're in a hurry. Goodbye!");
                    println!();
                    return;
                }

                println!("KeyboardInterrupt detected - type \"exit\" to exit");
                println!();
                last_input_was_ctrl_c = true;

                continue;
            }
            Err(_) => {
                eprintln!("Error reading input");
                eprintln!();
                last_input_was_ctrl_c = false;

                continue;
            }
        };

        // Handle custom commands
        match source_code.as_str() {
            "help" => {
                println!("{HELP_MESSAGE}");
                println!();
                continue;
            }
            "exit" => {
                break;
            }
            "clear" => {
                if clearscreen::clear().is_err() {
                    println!("Failed to clear the screen, sorry!");
                    println!();
                }

                continue;
            }
            "restart" => {
                input_lines = Arena::new();
                state = RuntimeState::new();

                // Show welcome message again
                println!();
                println!("{WELCOME_MESSAGE}");

                continue;
            }
            "debug" => {
                show_debug_info = !show_debug_info;
                println!(
                    "Debug information {}",
                    if show_debug_info {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );
                println!();

                continue;
            }
            _ => { /* If it wasn't a command, just carry on */ }
        };

        // If debug info is enabled, print source code information
        if show_debug_info {
            println!();
            print_source_info(SOURCE_NAME, source_code);
            println!();
        }

        // Lexer
        let tokens = match lexer::tokenize(source_code, SOURCE_NAME) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("{err}");
                println!();
                continue;
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
                println!();
                continue;
            }
        };

        // If debug info is enabled, print the AST
        if show_debug_info {
            println!("AST:");
            println!("{ast}");
            println!();
        }

        // Interpreting
        match interpreter::interpret_with_runtime_state(&ast, &mut state) {
            Ok(()) => {
                println!("{}", state.most_recent_value().icelang_debug());
                println!();
            }
            Err(err) => {
                println!("{err}");
                println!();
            }
        }

        // If debug info is enabled, print the runtime state
        if show_debug_info {
            println!("Interpreter State:");
            println!("{state}");
            println!();
        }
    }
}
