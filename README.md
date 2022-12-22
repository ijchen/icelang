# icelang

[![Test status](https://github.com/ijchen/icelang/actions/workflows/tests.yml/badge.svg)](https://github.com/ijchen/icelang/actions/workflows/tests.yml)
[![Docs](https://img.shields.io/docsrs/icelang/latest)](https://docs.rs/icelang)
[![Crate](https://img.shields.io/crates/v/icelang)](https://crates.io/crates/icelang)

Icelang (stylized "icelang") is a simple and high-level programming language
created for the purpose of learning about programming language design and
implementation. The goal of this project is **not** to create a general-purpose
programming language for use in the real world. Although I'm not *intentionally*
making icelang a "bad" language for real-world usage, it's simply not designed
for that use case. If you're looking for a language more suitable to general
use, I'd recommend taking a look at any of the languages listed below as sources
of inspiration for the design of icelang.

icelang is dynamically typed, and intended to be an interpreted language. Some
of the design goals for icelang include:
- To be simple to understand and write an interpreter for, but still powerful
  enough to solve non-trivial problems without excessive hullabaloo
- To be expressive where doing so doesn't significantly sacrifice simplicity or
  encourage writing error-prone code
- To be provably turing-complete

The design of icelang takes inspiration from many languages, primarily
[JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript),
[Python](https://www.python.org), and [Rust](https://www.rust-lang.org).

# Getting Started

For a full guide on setting up the icelang interpreter and writing your first
program, check out the [Getting Started](/docs/Getting%20Started.md) guide. For
a general overview of the syntax and features of icelang, check out
[The icelang Guidebook](/docs/Guidebook.md). You can also check out the
[examples](/examples/) folder to see some example icelang programs.

Here's a simple "hello world" program, written in icelang:
```
println("Hello, world");
```

And a program to print the first 10 Fibonacci numbers:
```
// A simple program to print the first 10 Fibonacci numbers

let a = 0;
let b = 1;

loop 10 {
    println(a);

    let c = a;
    a = b;
    b += c;
}
```

# To-do

icelang is still in development, and has a number of incomplete or missing
features:
- [ ] Make a cool logo
- [ ] Guidebook
  - [ ] Language features
  - [ ] Standard library features
- [ ] Formalize grammar
  - [x] Full program
  - [x] Statements
  - [x] Literals
    - [x] int
    - [x] byte
    - [x] float
    - [x] bool
    - [x] string
    - [x] list
    - [x] dict
    - [x] null
  - [x] Identifiers
  - [x] Variables
  - [x] Functions
  - [ ] Expressions
    - [ ] TODO expand
  - [ ] Control flow
    - [x] If/else
    - [x] Loops
      - [x] Simple loops
      - [x] `while` loops
      - [x] `for` loops
    - [ ] Match
- [ ] Implementation
  - [ ] ice (binary)
    - [ ] REPL
    - [ ] File interpreter
  - [ ] lexer
    - [ ] TODO expand
  - [ ] parser
    - [ ] TODO expand
  - [ ] interpreter
    - [ ] TODO expand
- [ ] Testing
  - [ ] Unit tests
    - [ ] General unit tests
    - [ ] Property-based tests (where applicable)
  - [ ] Integration tests
    - [ ] General integration tests
    - [ ] Fuzzing (where applicable)
- [ ] Flesh out examples
- [ ] Future considerations
  - [ ] Format string literal replacement field format specifiers
  - [ ] First-class function support
  - [ ] User-defined types
  - [ ] Compilation, JIT-compilation, transpilation, and static code analysis
  - [ ] Preprocessor and macros
  - [ ] Namespaces and including
  - [ ] Nullish-coalescing operator (JavaScript's `??`), null-propagation
  (Rust's `?`), optional chaining (JavaScript's `.?`)

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
