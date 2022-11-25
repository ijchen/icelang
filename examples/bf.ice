// A brainfuck interpreter, written in ice
// https://en.wikipedia.org/wiki/Brainfuck

// Interprets a brainfuck program
fn interpret(program) {
    // Initialize the program state
    let data = [];
    for _ in range(30000) {
        push(data, 8x00);
    }
    let data_ptr = 0;
    let instruction_ptr = 0;
    let bracket_stack = [];

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
            "." => { todo() },
            "," => { todo() },
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

                    // Advance past the matching "]"
                    instruction_ptr += 1;
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

// Read in lines from stdin until no more are available
let program = "";
let line = input();
while line != null {
    program += line;
    program += "\n";
}

// Interpret the brainfuck program
interpret(program);
