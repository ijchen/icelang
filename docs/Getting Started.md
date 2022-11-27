# Setting up the icelang interpreter
Before you are able to run icelang code, you will need to set up the icelang
interpreter. The icelang interpreter is a program that reads and understands
your icelang source code, and then executes the instructions you wrote.

Although this may change in the future, there is currently no option to download
a pre-built executable of the icelang interpreter - You will have to download
and build the source code yourself. Don't worry if you've never done that
before - Cargo (the build system for the programming language I wrote the
interpreter in) makes this process very easy!

To get started, install Cargo on your computer by following the instructions on
the [Rust website](https://www.rust-lang.org/tools/install). You will know you
have completed this step successfully when running the command `cargo --version`
in your terminal gives you no error messages and displays your version of Cargo.

Next, download the icelang interpreter source code from
[the official repository](https://github.com/ijchen/icelang) on GitHub. You can
do so by clicking the green "Code" dropdown button towards the top right of the
screen, then clicking "Download ZIP".

Once you've downloaded the source code as a ZIP, extract the zip contents to
somewhere on your computer. Open a terminal and navigate to the folder you
extracted. Your current working directory should contain a few files and
folders, including a folder called `src` and a file called `Cargo.toml`. Now,
run the command `cargo build --release` to compile the source code into a binary
executable for your computer. Once compilation finishes, there will be a binary
executable called `ice` (or `ice.exe` if you're on Windows) in the
`target/release/` directory. This is the icelang interpreter - you can now move
that file wherever you'd like on your computer, and you no longer need the
source code.

To test that you've done everything correctly, open a terminal and navigate to
the folder containing the icelang interpreter binary you built in the previous
step. Run the command `./ice --version` (or `./ice.exe --version` if you're on
Windows) and you should see a message indicating your version of icelang.

# Running icelang code
## Running icelang in an interactive shell
You can run icelang code in two ways: in a REPL (interactive shell) and from
`.ice` files. This is how to run icelang code in a REPL.

Open a terminal and navigate to the folder containing the icelang interpreter
binary. Run the executable with the command `./ice` (or `./ice.exe` if you're on
Windows). You should see a welcome message and help information about the REPL.
To run icelang code, simply type a line of icelang code into the terminal and
press enter. Your code will be evaluated, and the result will be printed to the
screen. Try the following line of code:

```
println("Hello, world!");
```

When you're done, type `exit` to exit the REPL.

## Creating and running `.ice` files
You can run icelang code in two ways: in a REPL (interactive shell) and from
`.ice` files. This is how to run icelang code from `.ice` files.

First, create a text file on your computer with the extension `.ice` (ex:
`hello.ice`). Write the source code of your program in that file. Try the
following example program:

```
println("Hello, world!");
```

Once you're done writing your program, save and close the file. Open a terminal
and navigate to the folder containing the icelang interpreter binary. Run the
executable with the path to your source code file as an argument (ex:
`./ice ~/Desktop/hello.ice`). The icelang interpreter will now run your program,
and you will see the output of your program in the terminal.

# Learning icelang
If you do not already have experience with another programming language, icelang
probably isn't a great choice to learn programming with. If you're interested in
learning to code, I'd recommend you start with a language like
[Python](https://www.python.org/) or
[JavaScript](https://developer.mozilla.org/en-US/docs/Learn/JavaScript).

If you do already have experience with another programming language, check out
the icelang [Guidebook](/docs/Guidebook.md) for a general overview of the syntax
and features of icelang. You can also check out the [examples](/examples/)
folder to see some example icelang programs.
