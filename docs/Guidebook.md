# The icelang Guidebook
Welcome to The icelang Guidebook! The icelang Guidebook is intended as a
reference and general overview of the syntax and features of icelang. It assumes
some familiarity with basic programming concepts - you may find it difficult to
read if you do not already have experience with another programming language.

If you do not already have experience with another programming language, icelang
probably isn't a great choice to learn programming with. If you're interested in
learning to code, I'd recommend you start with a language like
[Python](https://www.python.org/) or
[JavaScript](https://developer.mozilla.org/en-US/docs/Learn/JavaScript).

If you do already have experience with another programming language, check out
the icelang [Guidebook](/docs/Guidebook.md) for a general overview of the syntax
and features of icelang. You can also check out the [examples](/examples/)
folder to see some example icelang programs.

# Table of Contents
1. [Introduction](#the-icelang-guidebook)
2. [Types](#types)
   1. [int](#int)
   2. [byte](#byte)
   3. [float](#float)
   4. [bool](#bool)
   5. [string](#string)
   6. [list](#list)
   7. [dict](#dict)
   8. [null](#null)
3. TODO

# Types
icelang is a dynamically-typed language, meaning all type-checking is done at
runtime.

All values in icelang are one of the following primitive types: `int`, `byte`,
`float`, `bool`, `string`, `list`, `dict`, `null`

Here is more detailed information on each type:

## `int`
An `int` is a [signed](https://en.wikipedia.org/wiki/Signedness),
[arbitrary precision](https://en.wikipedia.org/wiki/Arbitrary-precision_arithmetic)
[integer](https://en.wikipedia.org/wiki/Integer). An `int` value will always be
a finite (but unbounded) positive, negative, or zero integer.

Operations on `int`s in icelang cannot overflow, since `int`s in icelang are
[arbitrary precision](https://en.wikipedia.org/wiki/Arbitrary-precision_arithmetic).

Some examples of valid `int`s include: `42`, `69`, `-13`, `0`,
`99999999999999999999`

Some examples of things which are *not* valid `int`s include: `3.14`, `Infinity`,
`NaN`

## `byte`
A `byte` is a single byte (8-bit value) interpreted as an unsigned integer in
the range 0 to 255 (both ends inclusive)

Operations on `byte`s in icelang are
[wrapping modulo 256](https://en.wikipedia.org/wiki/Modular_arithmetic).

Some examples of valid `byte`s include: `0`, `3`, `64`, `100`, `198`, `255`

Some examples of things which are *not* valid `byte`s include: `-1`, `256`,
`2.71828`, `5000`

## `float`
A `float` is a
[floating-point](https://en.wikipedia.org/wiki/Floating-point_arithmetic) number
with 64 bits of precision. Specifically, `float` is the "binary64" type defined
in [IEEE 754-2008](https://standards.ieee.org/ieee/754/4211/). See Wikipedia for
more information on
[floating-point arithmetic](https://en.wikipedia.org/wiki/Floating-point_arithmetic)
in general and
[64-bit floats](https://en.wikipedia.org/wiki/Double-precision_floating-point_format)
specifically.

**Warning: becaues `float`s cannot exactly represent most real numbers, care
must be taken not to rely on the *exact* values of `float`s**. In most cases, 64
bits of precision is more than enough, but there are
[some situations](https://en.wikipedia.org/wiki/Round-off_error) where the
inaccuracy of floating-point arithmetic can cause problems, including:
- When comparing `float` equality (ex: `0.1 + 0.2 != 0.3`)
- When small differences can compound and "snowball" into a big difference (ex:
simulations of [chaotic systems](https://en.wikipedia.org/wiki/Chaos_theory))
- When a high level of precision is required (ex: serious financial applications
(which, for the record, is a use case that icelang is ***absolutely not***
suited to))

`float`s can represent:
- Positive and negative real numbers (although only a finite subset of them)
- Positive and negative zero
- Positive and negative infinity
- NaN (not a number)

Some examples of \*valid `float`s include: `1.0`, `0.0`, `3.14159`, `-0.618`,
`-0.0`, `1234.56789`, `Infinity`, `-Infinity`, `NaN`

\**Technically, some of these `float` values might not be exactly representable
in the binary64 format, and so the nearest representable number will be stored
instead*

Some examples of things which are *not* valid `float`s include:
[complex numbers](https://en.wikipedia.org/wiki/Complex_number),
[hypercomplex numbers](https://en.wikipedia.org/wiki/Hypercomplex_number)

## `bool`
A `bool` is a [boolean](https://en.wikipedia.org/wiki/Boolean_data_type) `true`
or `false` value.

There are two valid `bool` values: `true` and `false`

## `string`
A `string` is a resizable collection of
[UTF-8](https://en.wikipedia.org/wiki/UTF-8) encoded characters (In icelang, the
term "character" refers to a
[Unicode scalar value](https://www.unicode.org/glossary/#unicode_scalar_value))

Some examples of valid `string`s include: `"Hello, world!"`, `""` (an empty
string), `"foaiu39pauhp"`, `"🦀 <3"` (the "Crab" emoji followed by an ASCII
heart, representing
[my love for the Rust programming language](https://rustacean.net/))

## `list`
A `list` is a resizable collection of values. A `list` may hold any number of
values (called "elements"), and will adjust its size automatically when elements
are added or removed. Additionally, elements in a `list` do not need to be the
same type.

There are many parts of the [icelang standard library](TODO) designed to help
work with `list`s.

Unlike most types in icelang, `list`s are passed and assigned as
[shared references](https://en.wikipedia.org/wiki/Evaluation_strategy#Call_by_sharing).
This means that when passing a `list` to a function, the `list` in the function
and the `list` that was passed to the function are *the same object in memory* -
modifications made to one will effect the other. Similarly, multiple variables
may refer to the same `list`, which means modifications to any one of those
variables will effect all of those variables:
```
// Create a new list, called my_list
let my_list = ["sharing"];

// This function will add elements to a list, *modifying the original list*
fn add_to_list(the_list) {
	push(the_list, "the");
	push(the_list, "references");
}

// Here, `my_list` is passed *as a shared reference* to `add_to_list(...)`
add_to_list(my_list);
assert(my_list == ["sharing", "the", "references"]);

// Let's make another variable called `same_list`, and assign it to `my_list`
// Note: this new variable refers to the same list that is stored in `my_list`
let same_list = my_list;
assert(same_list == ["sharing", "the", "references"]);

// Modifications made to `my_list` effect `same_list`
my_list[2] = "caring";
assert(same_list == ["sharing", "the", "caring"]);

// And vice-versa
same_list[1] = "is";
assert(my_list == ["sharing", "is", "caring"]);
```
To make a separate, independent copy of a `list`, use the built-in
[copy(...)](TODO) function.

Some examples of valid `list`s include: `[1, 2, 3]`, `["hi"]`, `[]` (an empty
list), `[5, null, "A string!", false, 8, 2.71828, false, true, -49]`,
`["Howdy", ["A string, in a list, in a list!", 3], [[["Deeply nested"]]], 95]`

## `dict`
A `dict` is a resizable dictionary mapping keys to values. A `dict` may hold any
number of key-value pairs (called "entries"), and will adjust its size
automatically when entries are added or removed. Additionally, there are no
special restrictions on the types of keys or values in a `dict` - it is
perfectly valid to have a `dict` containing multiple keys of different types,
each corresponding to values also of various different types.

There are many parts of the [icelang standard library](TODO) designed to help
work with `dict`s.

It's worth noting that the relative order of entries in a `dict` **does not
matter**, and is **not specified** - it may not be the same order as entries
were added, may not be the same as it was last time you ran the same code, and
*even may not be the same as it was last time you checked that same exact dict*.
Do not rely on the relative order of entries in a `dict` in your program.

Unlike most types in icelang, `dict`s are passed and assigned as
[shared references](https://en.wikipedia.org/wiki/Evaluation_strategy#Call_by_sharing).
This means that when passing a `dict` to a function, the `dict` in the function
and the `dict` that was passed to the function are *the same object in memory* -
modifications made to one will effect the other. Similarly, multiple variables
may refer to the same `dict`, which means modifications to any one of those
variables will effect all of those variables:
```
// Create a new dict, called my_dict
let my_dict = {"age": 21};

// This function will add an entry to a dict, *modifying the original dict*
fn add_to_dict(the_dict) {
	the_dict["is_hungry"] = false;
}

// Here, `my_dict` is passed *as a shared reference* to `add_to_dict(...)`
add_to_dict(my_dict);
assert(my_dict == {"age": 21, "is_hungry": false});

// Let's make another variable called `same_dict`, and assign it to `my_dict`
// Note: this new variable refers to the same dict that is stored in `my_dict`
let same_dict = my_dict;
assert(same_dict == {"age": 21, "is_hungry": false});

// Modifications made to `my_dict` effect `same_dict`
my_dict["is_hungry"] = true;
assert(same_dict == {"age": 21, "is_hungry": true});

// And vice-versa
same_dict["age"] = 25;
assert(my_dict == {"age": 25, "is_hungry": true});
```
To make a separate, independent copy of a `dict`, use the built-in
[copy(...)](TODO) function.

Some examples of valid `dict`s include:
`{"Alice": 97, "Bob": 79, "Charlie": 84}`, `{false: "N"}`, `{}` (an empty dict),
`{"weird dict": true, 4: -2, 9: null, -10: "negative ten", null: {"what is this?": ["confusion", "chaos"]}, 8b4: "not -2"}`

## `null`
A `null` value represents a "nothing" value, or the absence of a value. All
values of type `null` are the same - `null` is just `null`. There is no such
thing as "two different `null`s".

A `null` value represents the absence of a value, but doesn't provide any
additional information about *why* a value is absent, or what might have been
expected instead. If you are providing an interface which may work with `null`,
it is almost certainly a good idea to include additional information about
`null` means in your context.

There is only one valid `null` value: `null`

## TODO
Additionally, although the following types do not actually exist in icelang, The icelang Guidebook uses the following syntax for special types:
- `any`
	- A value that may be of any type
- `never`
	- A value that will never actually exist at runtime (ex: the type returned by `error(...)` or `assert(false)`)
- `?type`
	- A value which may be of type `type`, or instead may be `null`
- `type_a | type_b`
	- A value which may be either of type `type_a` or `type_b`
- `list[type]`
	- A `list` containing elements which are all of type `type`
- `dict[type_a: type_b]`
	- A `dict` containing only keys of type `type_a` and only values of type `type_b`

# Basic syntax

## Literals
- int
	- TODO (`69`, `0b01000101`, `0x45`, `0o105`)
- byte
	- TODO (`8b01000101`, `8x45`, `8d69`, `8o105`)
- float
	- TODO (`1.0`, `0.0`, `4.2e2`, `6.674e-11`, `3.14`, `Infinity`, `NaN`)
- bool
	- `true` or `false`
- string
	- TODO (`"Hello, world!"`, `f"2 + 2 = {2 + 2}"`, `"He said \"hi\" then left."`, `r"raw \ backslash and newline [NEWLINE] in this string"`, escape codes, maybe more)
- list
	- TODO `[val1, val2, val3, ...]`
- dict
	- TODO `{key1: val1, key2: val2, ...}`
- null
	- `null`

## Identifiers
TODO

## Comments
Comments come in two forms: line comments and block comments.

### Line comments
Line comments begin with the characters `//` and end with the last character before the next newline. This means a line comment cannot span multiple lines, nor can it end in the middle of a line.

```
// This is a line comment. It will be ignored, and has no effect

let foo; // This is another line comment, this time after a line of code!
```

### Block comments
Block comments begin with the characters `/*` and end with the characters `*/`. Unlike line comments, block comments can span multiple lines, and can exist within a line of code.
```
/* This is an example of a block comment */

/* Here
is
    a weird
      block
    comment
 spanning
   multiple
      lines!
    */

let foo /* inline block comment */ = 3.14;
```

Note that while a comment's contents are ignored, the comment's precense is syntactically equivalent to whitespace:
```
// This is invalid syntax - the block comment acts like whitespace
let fo/**/o;

// This is semantically the same as:
let fo o; // <- Invalid syntax!
```

## Variables
Variables are declared with the `let` keyword:
```
let foo;
```

and can be assigned a value with the `=` operator:
```
foo = 19;
```

These may be combined as such:
```
let foo = 19;
```

Uninitialized variables always contain the value `null`:
```
let foo;

assert(foo == null);
```

Multiple variables may be declared or assigned with a single use of the `let` keyword:
```
let foo, bar = 21, bat = "spooky", baz;

assert(foo == null);
assert(bar == 21);
assert(bat == "spooky");
assert(baz == null);
```

## Functions
### Defining and calling a function
Functions are defined with the `fn` keyword:
```
fn greet(name) {
	println("It's nice to meet you, " + name + "!");
}
```

and called as such:
```
greet("Ferris");
```

TODO Add information about pass-by-value and pass-by-sharing

### Returning a value from a function
Functions may return values back to the caller with the `return` keyword:
```
fn add(x, y) {
	return x + y;
}

assert(add(3, 2) == 5);
```

If a function doesn't return a value, the function call evaluates to `null`
```
fn no_return() {
	// Implicitly returns (without a value)
}

fn return_no_value() {
	// Explicit return without a value
	return;
}

assert(no_return() == null);
assert(return_no_value() == null);
```

### Function overloading
Functions may be overloaded based on the number of arguments:
```
fn add(x, y) {
	return x + y;
}

fn add(x, y, z) {
	return x + y + z;
}

assert(add(2, 3) == 5); // Calls the two-parameter overload
assert(add(2, 3, 4) == 9); // Calls the three-parameter overload
```

Functions can also be overloaded to accept any number of arguments:
```
// my_fn_args is bound to a list containing all
// the arguments passed to the function
fn add([my_fn_args]) {
	let sum = 0;

	for x in my_fn_args {
		sum += x;
	}

	return sum;
}
```

This allows the function `add` to be called with any number of arguments, but fixed-length overloads will always be preferred over variable-length overloads:
```
fn my_func(a, b) {
	// This overload is preferred to the variable-length one below
}

fn my_func([args]) {
	// This overload is not called, since the fixed-length one above
	// is preferred
}

my_func(1, 2); // Calls the first overload
```

## Expressions
TODO
(don't forget type casting)

## Control flow
### If/else statements
#### If statements
`if` statements can be used to conditionally execute a block of code:
```
if condition {
	// Your code here
}
```

where `condition` is a boolean value. If `condition` is `true`, the code in the block will be executed. If `condition` is false, the code in the block will be skipped over and not executed.

#### Else statements
the `else if` and `else` statements can be used to execute a block of code if all prior `if`/`else if` statements were skipped:
```
let age = 21;

if age < 16 {
	// Too young to drive
}
else if age < 18 {
	// Too young to vote
}
else if age < 21 {
	// Too young to drink
}
else {
	// Wishes they were younger
}
```

### Loops
A single block of code can be executed multiple times with loops.

#### Break and Continue
Loops are often combined with the `break` and `continue` keywords to exit a loop early. The `break` keyword immediately exits the loop completely, whereas the `continue` keyword immediately exits that *iteration* of the loop, continuing with the loop as normal.
```
let i = 0;
let sum = 0;

loop {
	// Stop the loop if the value of i is 3
	if i == 3 {
		break;
	}

	// Add i to sum
	sum += i;
	
	// Increment i
	i += 1;
}

assert(i == 3);
assert(sum == 0 + 1 + 2);
```

```
let i = 0;
let sum = 0;

loop {
	// Increment i
	i += 1;

	// Skip adding 3 to sum
	if i == 3 {
		continue;
	}

	// Add i to sum
	sum += i;
	
	// Stop the loop if the value of i is 5
	if i == 5 {
		break;
	}
}

assert(i == 5);
assert(sum == 1 + 2 + 4 + 5); // Note: 3 was skipped and not added to sum
```

#### Simple loops
The simplest form of loops are simple loops:
```
loop {
	// This loop runs forever!
}
```

Simple loops can optionally specify an `int` value for the number of times to repeat the loop:
```
loop 3 {
	// This code will execute 3 times
}
```

```
let count = 3;

loop count {
	// This code will execute 3 times
}
```

#### While loops
The second type of loop is the `while` loop. A `while` loop runs while a `bool` value is true:
```
let i = 0;

while i < 5 {
	i += 1;
}

assert(i == 5);
```

#### For loops
The third type of loop is the `for` loop. A `for` loop iterates through the values in a `list` or the characters in a `string`:
```
let my_list = [6, 3, 5, 9];
let sum = 0;

for val in my_list {
	// This loop runs four times, once for each value in my_list
	sum += val;
}

assert(sum == 6 + 3 + 5 + 9);
```

```
let my_list = [];
for character in "Howdy!" {
	// This loop runs six times, once for each character in "Howdy!"
	push(my_list, character);
}

assert(my_list == ["H", "o", "w", "d", "y", "!"]);
```

`for` loops are often used with the builtin `range(...)` function:
```
let sum = 0;

for i in range(5) {
	sum += i;
}

assert(sum == 0 + 1 + 2 + 3 + 4);
```

### Match
TODO

# Built-in functions
## Input/Output
### args
The `args` function returns a list containing all the command line arguments to the program (each as a string)

Function prototypes:
- `args() -> list[string]` (core)

### print
The `print` function outputs a value to stdout (without adding a trailing newline).

Function prototypes:
- `print(val: any) -> null` (core)
	- If `val` isn't a string, it will be converted to a string first

### println
The `println` function outputs a value to stdout, followed by a newline character.

Function prototypes:
- `println(val: any) -> null`
	- If `val` isn't a string, it will be converted to a string first

### eprint
The `eprint` function outputs a value to stderr (without adding a trailing newline).

Function prototypes:
- `eprint(val: any) -> null` (core)
	- If `val` isn't a string, it will be converted to a string first

### eprintln
The `eprintln` function outputs a value to stderr, followed by a newline character.

Function prototypes:
- `eprintln(val: any) -> null`
	- If `val` isn't a string, it will be converted to a string first

### input
The `input` function reads a string from stdin and returns it, stripping any trailing newline. If no more input is available (if EOF is reached, for example), `null` is returned. This call is blocking, meaning TODO (also specificy all blocking functions).

Function prototypes:
- `input() -> ?string` (core)

### read_file
The `read_file` function reads a file from disk, returning a `string` containing the contents of the file or `null` if the file couldn't be read into a string

Function prototypes:
- `read_file(path: string) -> ?string` (core (might not need to be?))
	- Returns null if the file can't be read from disk, or if the file contents can't be interpreted as a string

### read_file_bin
The `read_file_bin` function reads a binary file from disk, returning a `list` of `byte`s containing the contents of the file or `null` if the file couldn't be read

Function prototypes:
- `read_file_bin(path: string) -> ?list[byte]` (core)
	- Returns null if the file can't be read from disk

### write_file
The `write_file` function writes a string to a file, returning whether or not the file was written successfully

Function prototypes:
- `write_file(path: string, contents: string) -> bool` (core (may not need to be?))
	- Returns true if the file was written to disk successfully, false otherwise

### write_file_bin
The `write_file_bin` function writes a list of bytes to a binary file, returning whether or not the file was written successfully

Function prototypes:
- `write_file_bin(path: string, contents: list[byte]) -> bool` (core)
	- Returns true if the file was written to disk successfully, false otherwise

## Collections
### len
The `len` function returns the length of a collection. For `list`s this is the number of elements, and for `string`s it's the number of characters.

Function prototypes:
- `len(val: list | string) -> int` (core)

### push
The `push` function appends an element to the end of a list

Function prototypes:
- `push(l: list, val: any) -> null` (core)

### pop
The `pop` function removes an element from the end of a list and returns it, or null if the list was empty

Function prototypes:
- `pop(l: list) -> ?any` (core)

### push_start
The `push_start` function appends an element to the start of a list

Function prototypes:
- `push_start(l: list, val: any) -> null` (core)

### pop_start
The `pop_start` function removes an element from the start of a list and returns it, or null if the list was empty

Function prototypes:
- `pop_start(l: list) -> ?any` (core)

### contains_key
The `contains_key` function returns whether or not the given key corresponds to an entry in a `dict`. This will return true even if the key or value at the corresponding entry is `null`.

Function prototypes:
- `contains_key(d: dict, key: any) -> bool`

### remove_entry
The `remove_entry` function removes an entry from a `dict` by its key, returning the value at the removed key (or null if the key did not correspond to an entry in the `dict`)

Function prototypes:
- `remove_entry(d: dict, key: any) -> ?any` (core)

### keys
The `keys` function returns a list containing all the keys in a dictionary

Function prototypes:
- `keys(val: dict) -> list` (core)

## Time
### now
The `now` function returns the number of milliseconds since January 1st, 1970 at UTC

Function prototypes:
- `now() -> int` (core)

### sleep
The `sleep` function puts the current thread to sleep for at least `millis` milliseconds

Function prototypes:
- `sleep(millis: int) -> null` (core)

## Error
### error
The `error` function throws a RuntimeError (optionally with a provided message), immediately halting program execution and displaying an error message to stderr

Function prototypes:
- `error() -> never` (core)
- `error(msg: string) -> never` (core)

### assert
The `assert` function evaluates a condition, throwing a RuntimeError if the condition evaluates to `false`.

Function prototypes:
- `assert(condition: bool) -> null | never`

### todo
The `todo` function immediately throws a RuntimeError with a message indicating that the code that was executed is not yet implemented, but is intended to be later during development.

Function prototypes:
- `todo() -> never`

## String
### from_codepoint
The `from_codepoint` function returns a `string` containing a single character corresponding to the passed Unicode code point, or `null` if the passed `int` is not a valid Unicode scalar value

Function prototypes:
- `from_codepoint(codepoint: int | byte) -> ?string` (core (doesn't *technically* have to be, but it absolutely is going to be))

### to_codepoint
The `to_codepoint` function returns an `int` corresponding to the Unicode code point of the character in the given string, or null if the passed `string` isn't exactly one character

Function prototypes:
- `to_codepoint(character: string) -> ?int` (core (doesn't *technically* have to be, but it absolutely is going to be))

## Miscellaneous
### typeof
The `typeof` function returns a string representing the type of the argument. Here is the exact string corresponding to each type:
- `int` -> `"int"`
- `byte` -> `"byte"`
- `float` -> `"float"`
- `bool` -> `"bool"`
- `string` -> `"string"`
- `list` -> `"list"`
- `dict` -> `"dict"`
- `null` -> `"null"`

Function prototypes:
- `typeof(val: any) -> string` (core)

### copy
The `copy` function creates a deep copy of the passed value. This is only useful for `list`s and `dict`s, as all other types are always automatically copied

Function prototypes:
- `copy(val: list) -> list`
- `copy(val: dict) -> dict`

### range
The `range` function returns a list of `int`s, starting from some start value (`0` by default), stepping by some step value (`1` by default) and ending immediately before some end value.

The `range` function is commonly used with `for` loops.

Function prototypes:
- `range(end: int) -> list[int]`
- `range(start: int, end: int) -> list[int]`
- `range(start: int, end: int, step: int) -> list[int]`

### rand
The `rand` function returns a `float` pseudo-randomly chosen from a uniform distribution of numbers greater than or equal to `0.0` and less than `1.0`. 

Function prototypes:
- `rand() -> float`
