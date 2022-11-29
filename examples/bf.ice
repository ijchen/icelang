// A brainfuck interpreter, written in ice
// https://en.wikipedia.org/wiki/Brainfuck

// Executes a brainfuck program
fn execute(program) {
    // Initialize the program state
    let data = [];
    for _ in range(30000) {
        push(data, 8x00);
    }
    let data_ptr = 0;
    let instruction_ptr = 0;
    let bracket_stack = [];
    let stdin_buff = [];

    // Execute the program
    while instruction_ptr < len(program) {
        // Execute the current instruction
        match program[instruction_ptr] {
            ">" => {
                data_ptr += 1;
                while data_ptr >= len(data) {
                    push(data, 0);
                }
            },
            "<" => {
                data_ptr -= 1;
                if data_ptr < 0 {
                    println();
                    println("Something went wrong: memory underflow");
                    return;
                }
            },
            "+" => { data[data_ptr] += 8d1 },
            "-" => { data[data_ptr] -= 8d1 },
            "." => { print(from_codepoint(data[data_ptr])) },
            "," => {
                if stdin_buff != null && len(stdin_buff) == 0 {
                    let next_line = input();
                    if next_line == null {
                        stdin = null;
                    }
                    else {
                        for character in next_line {
                            push(stdin_buff, character);
                        }
                        push(stdin_buff, "\n");
                    }
                }
                // If we've reached the end of stdin, leave the data unchanged
                if stdin_buff != null {
                    let next_char = to_codepoint(pop_start(stdin_buff));
                    if next_char > 255 {
                        println();
                        println("Something went wrong: non-ASCII input character");
                        return;
                    }
                    data[data_ptr] = byte(next_char);
                }
            },
            "[" => {
                if data[data_ptr] == 8b0 {
                    // Advance to the matching "]"
                    let bracket_depth = 1;
                    loop {
                        instruction_ptr += 1;

                        if instruction_ptr >= len(program) {
                            println();
                            println("Something went wrong: unbalanced brackets");
                            return;
                        }

                        match program[instruction_ptr] {
                            "[" => { bracket_depth += 1 },
                            "]" => {
                                bracket_depth -= 1;

                                if bracket_depth == 0 {
                                    break;
                                }
                            },
                        }
                    }
                }
                else {
                    push(bracket_stack, instruction_ptr);
                }
            },
            "]" => {
                match pop(bracket_stack) {
                    null => {
                        println();
                        println("Something went wrong: unbalanced brackets");
                        return;
                    },
                    start_bracket_ptr => {
                        // This was the first ever bug in an ice program :)
                        // I intended to jump back to the start bracket, and
                        // skipped the zero check because I assumed if the
                        // current byte is zero, it's okay to just jump back to
                        // the "[" and we'll just see the zero then and skip
                        // forward again past this "]"... problem is, I forgot
                        // about that assumption when I was considering where to
                        // jump back to. I knew the instruction_ptr would be
                        // incremented at the end of this iteration of the loop,
                        // so I figured we'll just jump back to the matching "["
                        // and the incrementing will bring us to the first
                        // instruction at the start of the "[]" loop... perfect!
                        // ...except now both ends of the "[]" loop are assuming
                        // the other end did the zero check, and it's never
                        // happening. Oops!
                        instruction_ptr = start_bracket_ptr;

                        // Let's add a continue to skip incrementing the
                        // instruction_ptr (that way, we'll jump back exactly to
                        // the "[" and not the next instruction after it)
                        continue;
                    },
                }
            },
        }

        // Advance to the next instruction
        instruction_ptr += 1;
    }

    if len(bracket_stack) > 0 {
        println();
        println("Something went wrong: unbalanced brackets");
        return;
    }
}

fn main() {
    // Ask the user for a brainfuck source file
    print("Enter the path to a brainfuck source file: ");
    let file_path = input();
    if file_path == null {
        println("Failed to read file path from stdin");
        return;
    }

    // Read the brainfuck source file into a string
    let code = read_file(file_path);
    if code == null {
        println(f"Failed to read file: {file_path}");
        return;
    }

    // Interpret the brainfuck code
    execute(code);
}
main();
