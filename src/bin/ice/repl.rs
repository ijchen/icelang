use icelang::lexer;
use rustyline::{error::ReadlineError, Editor};
use typed_arena::Arena;

use crate::debug_info::print_source_info;

const WELCOME_MESSAGE: &str = "\
ice REPL - Welcome!
Heads up: the ice language is still in development - expect changes as the language develops.
enter \"help\" for help, or \"exit\" to exit.";

const HELP_MESSAGE: &str = "\
You can type code directly into the terminal. It will be evaluated, and the resulting value will be printed out.

You can also use the following commands:
help                Display this help message
exit                Exit the REPL
clear               Clear the screen
debug               Enable showing debug information
nodebug             Disable showing debug information";

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
    let input_lines: Arena<String> = Arena::new();

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
                    return;
                }

                println!("KeyboardInterrupt detected - type \"exit\" to exit");
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
                continue;
            }
            "exit" => {
                break;
            }
            "clear" => {
                if clearscreen::clear().is_err() {
                    println!("Failed to clear the screen, sorry!");
                }

                continue;
            }
            "debug" => {
                show_debug_info = true;
                println!("Debug information enabled");

                continue;
            }
            "nodebug" => {
                show_debug_info = false;
                println!("Debug information disabled");

                continue;
            }
            _ => { /* If it wasn't a command, just carry on */ }
        };

        // If debug info is enabled, print source code information
        if show_debug_info {
            println!();
            print_source_info("<stdin>", source_code);
            println!();
        }

        // Lexer
        let tokens = match lexer::tokenize(source_code, "<stdin>") {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Lexer error: {err}");
                continue;
            }
        };

        // If debug info is enabled, print token information
        if show_debug_info {
            println!("Tokens:");
            for token in tokens {
                println!("\t{token}");
            }
            println!();
        }
    }
}
