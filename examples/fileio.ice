// Demonstrate reading a file
fn read() {
    // Get the file path from the user
    println();
    println("Enter the path of the file to read:");
    print(">>> ");
    let file_path = input();
    println();

    // Read the file
    let file_contents = read_file(file_path);

    if file_contents == null {
        println(f"Failed to read file \"{file_path}\" - Sorry!");
        println();
    }
    else {
        println("--------------------------------------------------------------------------------");
        println(file_contents);
        println("--------------------------------------------------------------------------------");
        println();
    };
};

// Demonstrate writing to a file
fn write() {
    // Get the file path from the user
    println();
    println("Enter the path of the file to overwrite:");
    print(">>> ");
    let file_path = input();
    println();

    // Get the contents from the user
    println();
    println(f"Enter the text to write to \"{file_path}\":");
    print(">>> ");
    let contents = input();
    println();

    // Write to the file
    let successful = write_file(file_path, contents);

    if successful {
        println(f"Successfully wrote to file \"{file_path}\".");
        println();
    }
    else {
        println(f"Failed to write to file \"{file_path}\" - Sorry!");
        println();
    };
};

loop {
    // Prompt the user for what kind of file i/o they want to do
    println("What would you like to do?");
    println("- \"read\" - read a text file");
    println("- \"write\" - write text to a file");
    println("- \"exit\" - exit the program");

    print(">>> ");
    let chosen_option = input();

    match chosen_option {
        "read" => {
            read();

            continue;
        },
        "write" => {
            write();

            continue;
        },
        "exit" => {
            break;
        },
        null => {
            println();
            break;
        }
    };

    println();
    println(f"Sorry, I don't know what you mean by \"{chosen_option}\"");
    println();
};