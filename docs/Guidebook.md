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

If you do already have experience with another programming language, this
guidebook will act as an overview and reference of the syntax and features of
icelang, and of the icelang standard library.

# Table of Contents
1. [Introduction](#the-icelang-guidebook)
2. [Table of Contents](#table-of-contents)
3. [Anatomy of an icelang program](#anatomy-of-an-icelang-program)
4. [Types](#types)
	1. [int](#int)
	2. [byte](#byte)
	3. [float](#float)
	4. [bool](#bool)
	5. [string](#string)
	6. [list](#list)
	7. [dict](#dict)
	8. [null](#null)
	9. [Special type-like syntax used in The icelang Guidebook](#special-type-like-syntax)
5. [Literals](#literals)
	1. [int](#int-1)
	2. [byte](#byte-1)
	3. [float](#float-1)
	4. [bool](#bool-1)
	5. [string](#string-1)
	6. [list](#list-1)
	7. [dict](#dict-1)
	8. [null](#null-1)
6. [Expressions](#expressions)
	1. [Atomic expressions](#atomic-expressions)
	2. [Operator precedence](#operator-precedence)
	3. [Assignment expressions](#assignment-expressions)
	4. [Inline conditional expressions](#inline-conditional-expressions)
	5. [Mathematical operations](#mathematical-operations)
	6. [Bitwise operations](#bitwise-operations)
	7. [Logical operations](#logical-operations)
	8. [Comparisons](#comparisons)
	9. [Function calls](#function-calls)
	10. [Member access](#member-access)
	11. [Type casting](#type-casting)
7. Control flow (TODO)
8. Declarations (TODO)
9. The Standard Library (TODO)
10. Miscellaneous
	1. Comments (TODO)

# Anatomy of an icelang program
An icelang program consists of zero or more statements seperated by semicolons.
A statement is a single instruction, but may consist of multiple parts. There
are three different types of statements:
1. Expressions
2. Control-flow statements
3. Declarations

[Expressions](https://en.wikipedia.org/wiki/Expression_(computer_science)) are
evaluated to perform general computations, and always evaluate to a value.
[Control-flow](https://en.wikipedia.org/wiki/Control_flow) statements are used
to manipulate the order in which other statements are executed.
[Declarations](https://en.wikipedia.org/wiki/Declaration_(computer_programming))
establish the existence of a new variable or function. You will learn more about
these different kinds of statements as you progress through this guidebook.

As an example, here's a simple icelang program that has multiple different
expressions, control-flow statements, and declarations:
```
// Function declaration
fn get_greeting_phrase(name) {
	// Variable declaration with a string literal expression
	let greeting = "Hi there";

	// Return control-flow statement with expression(s)
	return f"{greeting}, {name}";
};

// Variable declaration with expression(s)
let names = ["Alice", "Bob", "Charlie", "Isaac"];

// For loop control-flow statement with a variable
// access expression (names)
for name in names {
	// Variable declaration
	let message;

	// If-else control-flow statement, comparison expression
	if name == "Isaac" {
		// Assignment expression
		message = "Oh look, it's me!";
	}
	else {
		// Assignment expression containing a function call
		// expression containing a variable access expression
		message = get_greeting_phrase(name);
	};

	// Function call expression containing a variable access
	// expression
	println(message);
};
```

# Types
icelang is a dynamically-typed language, meaning all type-checking is done at
runtime.

All values in icelang are one of the following primitive types: `int`, `byte`,
`float`, `bool`, `string`, `list`, `dict`, `null`

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
A `string` is an immutable collection of
[UTF-8](https://en.wikipedia.org/wiki/UTF-8) encoded characters (In icelang, the
term "character" refers to a
[Unicode scalar value](https://www.unicode.org/glossary/#unicode_scalar_value))

`string`s in icelang are immutable, meaning that the characters in a `string`
cannot be modified after the `string` is created. In order to reproduce the
effect of mutating a `string` in icelang, you will have to create a new `string`
with the new value.

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
};

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
A `null` value represents a "nothing" value, or the absence of a valid value.
All values of type `null` are the same - `null` is just `null`. There is no such
thing as "two different `null`s".

A `null` value represents the absence of a valid value, but doesn't provide any
additional information about *why* a valid value is absent, or what might have
been expected instead. If you are providing an interface which may work with
`null`, it is almost certainly a good idea to include additional information
about what `null` means in your context.

There is only one valid `null` value: `null`

## Special type-like syntax
Although the following types do not actually exist in icelang, The icelang
Guidebook uses the following syntax to refer to special "types":
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

# Literals
The simplest way to obtain a value in icelang is with a literal. A literal is an
expression you can write in your source code that evaluates to a fixed value.
There is literal syntax for every type in icelang.

## Int
icelang supports many types of `int` literals. The most common form you'll see
is a normal decimal `int` literal:
```
42
```

You can also specify a radix with base-prefixed literals:
```
// 0d prefix for decimal
assert(0d42 == 42);

// 0b prefix for binary
assert(0b101010 == 42);

// 0d prefix for hexadecimal
assert(0x2A == 42);

// 0o prefix for octal
assert(0o52 == 42);
```

`int` literals may be expressed in scientific notation, although this is only
allowed for base-10 literals:
```
assert(34e6 == 34000000);
assert(0d34e6 == 34000000);

// Non-base-10 literals cannot be written in scientific notation
// 0b101010e6 <-- Invalid literal

// A signed exponent is not allowed
// 34e+6 <-- Invalid literal
```

Underscores are permitted as a visual seperator in any of the above `int`
literals:
```
assert(1_234_567 == 1234567);
assert(0b1011_0000_0000_1011 == 0b1011000000001011);
assert(0x52_75_73_74_20_3C_33 == 0x52757374203C33);
assert(3__4_e_6_ == 34000000); // Don't do this
```

## byte
`byte` literals in icelang are very similar to `int` literals. In order to
distinguish between them, all `byte` literals in icelang *must* be
base-prefixed, but with an `8` where `0` was in base-prefixed `int` literals:
```
let my_byte_value = 8d42;

assert(typeof(my_byte_value) == "byte");

assert(my_byte_value == byte(42));
```

Like `int` literals, You can also specify a radix with `byte` literals:
```
// 8b prefix for binary
assert(8b101010 == 8d42);

// 8d prefix for hexadecimal
assert(8x2A == 8d42);

// 8o prefix for octal
assert(8o52 == 8d42);
```

`byte` literals may not be expressed in scientific notation
```
// byte literals cannot be written in scientific notation
// 8d1e2 <-- Invalid literal
```

Like `int` literals, underscores are permitted as a visual seperator in `byte`
literals:
```
assert(8d1_2_8 == 8d128);
assert(8b0110_1001 == 8b01101001);
assert(8xF_E == 8xFE);
assert(8o3_7_1 == 8d249);
```

## float
`float` literals in icelang are composed of an integer part, a decimal point,
a fractional part, and an optional exponent:
```
// A few examples of float literals
76.54321;
3.14159265358979323;
0.0;
1.0;
0.25;

// Examples of float literals in exponential notation
6.67430e-11;
0.0000314e+5;
0.0000314e5;

// Scientific notation usually only has one digit in the integer part, but this
// isn't a requirement
123.456e3;

// Neither the integer part nor the fractional part are optional
// .25 <- Invalid literal
// 1. <- Invalid literal
```

`float` literals can also be `Infinity` or `NaN`:
```
// Not a number
NaN;

// Infinity
Infinity;
```

## bool
There are only two `bool` values, and icelang has a literal for each:
```
// True
true;

// False
false;
```

## string
There are three forms of `string` literals in icelang. The simplest is a normal
`string` literal, composed of two double quotes (`"`) surrounding the characters
in the `string`:
```
"Hello world!"
```

Note that single quotes are not permitted for creating `string` literals:
```
// Invalid literal
// 'This is not allowed in icelang'
```

`string` literals may contain any [valid character](#string) except for double
quote (`"`) and backslash (`\`)
```
// String literals can contain non-ASCII characters too
"This string has lots of non-ASCII characters like 你好 and 🦀";

// They can also contain newlines
"This string contains a newline...
see, new line!";

// They cannot, however, have double quotes or backslashes - both of these are
// Invalid:
// "String literals cannot have double quotes -> " <- like that one"
// "String literals cannot have backslashes -> \ <- like that one"
```

What if you want to include a double quote or a backslash in your `string`
literal? Escape sequences to the rescue! Escape sequences begin with a backslash
(this is why they aren't normally allowed in `string` literals) and are followed
by some information about what character the escape sequence represents. Here is
a table of all the valid escape sequences in icelang, the character it
corresponds to, and which types of string literals it can be used in:
| Escape code  | Character           | Normal |  Raw  | Format |
| ------------ | ------------------- | :----: | :---: | :----: |
| `\"`         | double quote        |   ✓    |   ✗   |   ✓    |
| `\\`         | backslash           |   ✓    |   ✗   |   ✓    |
| `\t`         | tab                 |   ✓    |   ✗   |   ✓    |
| `\n`         | newline             |   ✓    |   ✗   |   ✓    |
| `\r`         | carriage return     |   ✓    |   ✗   |   ✓    |
| `\0`         | null character      |   ✓    |   ✗   |   ✓    |
| `{{`         | opening curly brace |   ✗    |   ✗   |   ✓    |
| `}}`         | closing curly brace |   ✗    |   ✗   |   ✓    |
| `\<newline>` | Nothing - ignored   |   ✓    |   ✗   |   ✓    |
| `\x##`       | ASCII character     |   ✓    |   ✗   |   ✓    |
| `\u{######}` | Unicode character   |   ✓    |   ✗   |   ✓    |

There are two special escape sequences: the ASCII character escape and the
Unicode character escape.

The ASCII character escape sequence encodes an ASCII character, and begins with
a backslash (`\`), followed immediately by a lowercase x (`x`), then exactly two
hex digits. The value of the hex digits must be `0x7F` or lower, and corresponds
to the [ASCII value](https://en.wikipedia.org/wiki/ASCII#Character_set) of the
character represented by the escape sequence.

The Unicode character escape sequence encodes a Unicode character, and begins
with a backslash (`\`), followed immediately by a lowercase u (`u`), an opening
curly brace (`{`), one to six hex digits, and finally a closing curly brace
(`}`). The value of the hex digits must be a [valid character](#string) in
Unicode, and corresponds to the character represented by the escape sequence.

Here are some examples of using escape sequences in `string` literals:
```
// This will become: He said "Wow!" then left.
"He said \"Wow!\" then left.";

// This string will contain a newline character
"What's your name?\nMy name is Isaac.";

// This string contains a *single* backslash (escaped as \\)
"A forward slash is /, and a backslash is \\";

// This string uses ASCII and Unicode escape sequences
let my_string = "Boy, I sure do love \x52\x75st! \u{1F980}\u{1F980}\u{1F980}";
assert(my_string == "Boy, I sure do love Rust! 🦀🦀🦀");
```

Sometimes, a `string` may contain many characters that need to be escaped, and
using escape codes would be tedious. In situations like these, you can use a raw
`string` literal, which does not allow escape sequences, interpreting characters
exactly as they appear. Raw `string` literals are just like normal `string`
literals, except they have a lowercase r (`r`) before the first double quote:
```
let my_string = r"I'm a raw string with a backslash \ inside!";
assert(my_string == "I'm a raw string with a backslash \\ inside!");

let my_other_str = r"This isn't an escape sequence: \x69";
assert(my_string == "This isn't an escape sequence: \\x69")
```

If you want to include a double quote in a raw `string` literal, surround the
outer double quotes in [octothorpes](## "Some people would get mad if I called it a 'hashtag', some people would get mad if I call it a 'pound sign'. This will hopefully upset everyone equally :)")
(`#`):
```
let my_string = r#"This string has a double quote "... isn't it lovely?"#;
assert(my_string == "This string has a double quote \"... isn't it lovely?");
```

If you, for some reason, need to have `"#` in your raw `string` literal, or
`"##`, or `"#######`, you can add as many octothorpes as you need - just ensure
there are the same number of octothorpes on each side:
```
let my_string = r###"I don't know why "## anyone would ever need this"###;
assert(my_string == "I don't know why \"## anyone would ever need this");
```

The last type of `string` literal in icelang is a format `string` literal.
Format `string` literals are like normal string literals, but allow embedding
arbitrary expression in replacement fields within the `string`. Format `string`
literals begin with a lowercase f (`f`), then look like a normal `string`
literal. Replacement fields are enclosed in curly braces (`{` and `}`), and the
expression inside is evaluated, converted to a `string` if necessary, and
inserted into the `string`:
```
let name = "Isaac";
assert(f"Hello, {name}!" == "Hello, Isaac!");

// Replacement fields may contain arbitrary expressions:
assert(f"2 + 2 = {2 + 2}" == "2 + 2 = 4");

// Any number of replacement fields are allowed:
let how_many = "any number of";
let my_str = f"Format string literals can have {how_many} \
{"replacement fields"}! This one has {1 + 1 + 1}.";
assert(my_str == "Format string literals can have any number \
of replacement fields! This one has 3.");
```

Because curly braces mark the beginning and end of a replacement field in format
`string` literals, they must be escaped as `{{` and `}}` if you want the actual
curly brace characters in your string:
```
let what_are_they = "pretty cool";
let my_str = f"Curly braces look like this: {{ }} and are {what_are_they}!";

assert(my_str == "Curly braces look like this: { } and are pretty cool!");
```

## list
The syntax for a `list` literal in icelang is as follows:
```
[value1, value2, value3, ...]
```

where each value is an arbitrary expression.

A `list` literal may have any number of elements:
```
// This list is empty:
[];

// This list has lots of elements (well, 11, but you get the idea):
[
	"Hello", "Hola", "Bonjour", "Привет",
	"Namaste", "你好", "안녕하십니까", "Mrhban",
	"Hallå", "Habari", "Olá", // <- trailing comma is ignored
];
```

The elements in a `list` literal can be arbitrary expressions, which will be
evaluated in the order they appear in the literal:
```
let my_var = "Hello";

let count = 0;
fn add_count(n) {
	count += 1;
	return n + count;
}

let my_list = [
	2 + 2,
	add_count(17),
	add_count(1 + 2),
	null,
	add_count(5),
	my_var,
	add_count(3),
];

assert(count == 4);
assert(my_list == [4, 18, 5, null, 8, "Hello", 7]);
```

## dict
The syntax for a `dict` literal in icelang is as follows:
```
{key1: value1, key2: value2, ...}
```

where each key and value is an arbitrary expression.

A `dict` literal may have any number of entries:
```
// This dict has no entries:
{};

// This dict has lots of entries (well, 11, but you get the idea):
{
	"EN": "Hello",
	"ES": "Hola",
	"FR": "Bonjour",
	"RU": "Привет",
	"HI": "Namaste",
	"ZH": "你好",
	"KO": "안녕하십니까",
	"AR": "Mrhban",
	"SV": "Hallå",
	"SW": "Habari",
	"PT": "Olá", // <- trailing comma is ignored
};
```

Both the keys and the values in a `dict` literal can be arbitrary expressions,
which will be evaluated in the order they appear in the literal:
```
let my_var = "Hello";

let count = 0;
fn add_count(n) {
	count += 1;
	return n + count;
}

let my_dict = {
	2 + 2: add_count(17),
	add_count(1 + 2): add_count(4),
	null: add_count(5),
	my_var: "hi",
	add_count(3): false,
};

assert(count == 5);
assert(my_dict == {
	4: 18,
	5: 7,
	null: 9,
	"hello": "hi"
	8: false
});
```

## null
There is only one `null` value:
```
null
```

# Expressions

## Atomic expressions
TODO

## Operator precedence
TODO

## Assignment expressions
TODO

## Inline conditional expressions
TODO

## Mathematical operations
TODO

## Bitwise operations
TODO

## Logical operations
TODO

## Comparisons
TODO

## Function calls
TODO

## Member access
TODO

## Type casting
Type casting expressions in icelang are used to convert a value from one type to
another. The original type being converted from is called the *source* type, and
the new type being converted to is called the *destination* type.

The exact semantics of how a value is converted depends on the source and
destination types, and are detailed in a list below. Additionally, many
combinations of source and destination types are not valid for a type cast, and
attempting to do so anyway will result in a runtime error. As an example, it is
not possible to convert a `bool` into a `float`

The syntax for a type cast is the destination type followed by parenthesis
containing the value to be cast:
```
let my_number = "42"; // Note: my_number is a string, not an int (yet)
let cast_to_int = int(my_number);
assert(cast_to_int == 42);
```

Most combinations of source and destination types in icelang are not valid for
casting. There are no valid type casts with `list`, `dict`, or `null` as either
the source or destination type. Additionally, a type may not be cast to itself -
doing so is useless, and attempting to do so usually indicates an error in the
design or logic of the program.

### A quick note on fallability
Some combinations of source and destination types are infallible conversions,
meaning that the conversion will always result in a valid value of the
destination type, regardless of the value being cast. An example of an
infallible conversion is casting a `byte` to an `int`. Every possible `byte`
value can be converted successfully into a valid `int` value, so the cast cannot
fail.

Other combinations of source and destination types are fallible conversions,
meaning that for some possible values of the source type, it is not possible to
produce a valid value of the destination type. An example of a fallible
conversion is casting an `int` to a `byte`. There are some `int` values, like
`-5` or `300`, which are not possible to represent as a `byte`. When a fallible
cast fails, the resulting value is `null`. When performing a fallible cast, care
must be taken to ensure your program behaves correctly if the cast fails.

### List of valid casts
The following is a complete list of all valid source and destination types for
type casts, as well as the details of how the cast is performed:

#### `int` to `byte` (fallible)
If the `int` value is a valid `byte` (0 <= value <= 255), it will be converted
to the same numerical value as a `byte`. If the `int` value is not a valid
`byte` (value < 0 || value > 255), the cast will result in `null`.

#### `int` to `float` (infallible)
The `int` value will be converted to the nearest representable `float`. Values
greater than the maximum finite `float` value or less than the minimum finite
`float` value are converted to positive and negative `Infinity` respectively.

The behavior In the event of a "tie" (when the )

#### `byte` to `int` (infallible)
The `byte` value will be converted to the same numerical value as an `int`.
Every possible `byte` value is also valid as an `int`, so this conversion cannot
fail.

#### `byte` to `float` (infallible)
The `byte` value will be converted to the same numerical value as a `float`.
Every possible `byte` value is also valid as an `float`, so this conversion
cannot fail.

#### `float` to `int` (fallible)
The `float` will be rounded towards zero, then converted to the same numerical
value as an `int`. Attempting to cast positive `Infinity`, negative `Infinity`,
or `NaN` will result in `null`.

#### `int` to `string` (infallible)
Converts the `int` to a human-readable `string` in decimal (base 10). The exact
output will be identical to calling the standard library function
[`fmt(...)`](TODO) function with default formatting arguments.

#### `byte` to `string` (infallible)
Converts the `byte` to a human-readable `string` in capital hexadecimal, zero
padded to two digits if necessary. The exact output will be identical to calling
the standard library function [`fmt(...)`](TODO) function with default
formatting arguments.

#### `float` to `string` (infallible)
Converts the `float` to a human-readable `string` with similar syntax to a
`float` literal. The exact output will be identical to calling the standard
library function [`fmt(...)`](TODO) function with default formatting arguments.

#### `bool` to `string` (infallible)
Converts the `bool` to a human-readable `string`, either `"true"` or `"false"`.
The exact output will be identical to calling the standard library function
[`fmt(...)`](TODO) function with default formatting arguments.

#### `string` to `int` (fallible)
Attempts to parse the `string` as an `int` following the same syntax as an `int`
literal. Any valid `int` literal will be converted to its corresponding value as
an `int`, and anything else will result in `null`.

#### `string` to `byte` (fallible)
Attempts to parse the `string` as a `byte` following the same syntax as a `byte`
literal. Any valid `byte` literal will be converted to its corresponding value
as a `byte`, and anything else will result in `null`.

#### `string` to `float` (fallible)
Attempts to parse the `string` as a `float` following the same syntax as a
`float` literal. Any valid `float` literal will be converted to its
corresponding value as a `float`, and anything else will result in `null`.

# TODO

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
	println(f"It's nice to meet you, {name}!");
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

`for` loops are often used with the builtin [`range(...)`](TODO) function:
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

### repr
The `repr` function returns a string representing the argument. What exactly
this is varies by type, but it is generally similar to a literal that would be
equal to the passed value.

TODO Specify the output format *exactly*

For some types, the exact output might be slightly different than you'd expect.

`strings` may be escaped as necessary:
```

// If printed, my_string would look like: my "quote"	here
let my_string = "my \"quote\"\there";

// If printed, my_string_repr would look like: "my \"quote\"\there"
let my_string_repr = repr(my_string);
assert(my_string_repr == "\"my \\\"quote\\\"\\there\"");
```

`float`s may be truncated somewhat arbitrarily:
```
let my_float = 0.200000000000000011102230246251565404236316680908203125;
assert(repr(my_float) == "0.2")
```

Recursive `list`s and `dict`s may have all or parts replaced:
```
let my_list = [3, 4, 5];

my_list[1] = my_list;

assert(repr(my_list) == "[3, [<recursive>], 5]");

let my_dict = {"rec_list": my_list, "rec_dict": {}};

my_dict["rec_dict"]["self"] = my_dict;

// TODO this relies on relative entry order in a dict, which is unspecified
assert(
	repr(my_list) ==
	"{\"rec_list\": [3, [<recursive>], 5], \"rec_dict\": {\"self\": {<recursive up 1>}}}"
);
```

Function prototypes:
- `repr(val: any) -> string` (core)

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
