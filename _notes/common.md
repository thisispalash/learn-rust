Common programming concepts in the context of Rust, with conventions.

## Variables (and Constants)

- Variables are defined with the `let` keyword
- By default in Rust all variables are *immutable*. For mutability, add the `mut` keyword while defining the variable.
- Shadowing allows us to redefine a variable (and its type) by using the `let` keyword
- Shadowing preserves immutability of the variable
- Constants are defined by the `const` keyword.
- `mut` cannot be used with `const`.
- `const` must always be annotated, ie, its data type must be defined.
- Finally, `const` cannot be set to a value of a runtime result.

## Data Types

- While being a strong statically typed language, Rust also features type inference
- Scalar types :: integers, floating points, Boolean, and characters
  - Signed integers start with `i`, unsigned with `u`
  - 8, 16, 32, 64, 128 bits + `arch` which depends on OS architecture
  - Rust default is `i32`
  - Can include `_` for readability, ie, `1_000 == 1000`
  - Can also store hex (`0x` prefix), octal (`0o` prefix), and binary (`0b` prefix)
  - In case of integer overflow the compiler panics in debug mode and wraps around in release mode
  - Two floating types, `f32` and `f64`; `f64` is default
  - Integer operations are `+`, `-`, `*`, `/`, `%`
  - Booleans values are `true` and `false`
  - Booleans are specified with `bool`
  - Characters are 4 bytes specified with `char` and single quotes
  - Characters represent Unicode Scalar Values, ie, `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive
- Compound types :: tuples and arrays
  - Tuples are used to store a variety of types in one type
  - Tuples have fixed length
  - Tuples can be destructured using `let`, ie, `let (...) = tup`
  - Tuple elements can also be accessed with the dot operator
  - The *unit type* tuple `()` with *unit value* `()` is returned implicitly if no other value is returned
  - Arrays have multiple values of the same type
  - Arrays are fixed in length; use Vectors for variability
  - Array types are specified with a semi colon as such - `[i32; 5]`
  - Arrays can be accessed by indexing and using square brackets

## Functions

- Convention is to use *snake_case* when defining functions
- Functions are defined using the `fn` keyword and invoked directly
- For parameters, the type *must* be declared in the function signature
- Statements are instructions that return no value, expressions return a value
  - `let x = 3 + 2` is a statement, `3 + 2` is an expression
  - expressions do not end with semicolons
- Functions that return a value must have return type in signature after `->`
  - By default last expression is returned; be careful of `;`
  - Can return early using `return` keyword

## Comments

- All comments begin with `//` including multiline comments
- Multiline comments must begin with `//` on each line
- Documentation comments are something else*

## Control Flow

- Conditional control flow with `if`
  - `if` conditions must always evaluate to a `bool`
  - `else` is optional
  - multiple branches possible with `else if`
  - `if` can be used on the RHS of a `let` statement, but the type must be same
- Looping with `loop`, `while`, and `for`
  - `loop` enters the program into an infinite loop until an explicit `break`
  - `continue` is also a keyword
  - `break` and `continue` apply to the innermost loop at the point
  - loop labels are possible which can be invoked later
  - adding a value after `break` can return a value from the loop, making it an expression
  - `while` is for conditional loops
  - `for` loops through a collection
  - `for` is the *safest* loop to use