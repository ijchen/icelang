# The icelang Guidebook
TODO

# Types
All values in icelang are one of the following primitive types: `int`, `byte`, `float`, `bool`, `string`, `list`, `dict`, `null`

TODO reformat with headers

Here is more detailed information on each type:
- `int`
	- A signed, arbitrary precision integer
	- Will always be a finite (but unbounded) positive, negative, or zero integer
- `byte`
	- A single byte (8 bit value) interpreted as an unsigned integer in the range 0 to 255 (both inclusive)
- `float`
	- A floating point number with 64 bits of precision
		- Specifically, the "binary64" type defined in IEEE 754-2008
	- Floats can represent:
		- Positive and negative real numbers (although only a finite subset of them)
		- Positive and negative zero
		- Positive and negative infinity
		- NaN
- `bool`
	- A true or false value
- `string`
	- A resizable UTF-8 encoded string
- `list`
	- A resizable collection of values
	- `list`s may hold values of different types
	- Unlike most primitives, `list`s are passed by reference and not copied
- `dict`
	- A resizable dictionary mapping keys to values
	- The both keys and values in a `dict` may be any combination of types
	- Unlike most primitives, `dict`s are passed by reference and not copied
- `null`
	- A "nothing" value, representing the absence of a value

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
- `dict[type_a, type_b]`
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
### Defining a function
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
