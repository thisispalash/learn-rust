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

## Functions

## Comments

## Control Flow