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

## Comments

## Control Flow