## Packages

- A crate is a binary or a library
- A package is one or more crates, containing a `Cargo.toml` file
- A package can have at most one library crate, but can have any number of binary crates
- The multiple binary crates are stored in `src/bin` where each file is a separate crate
- `src/main.rs` or `src/lib.rs` are the root of the crates

## Modules

- Modules help organise code *within* crates and control privacy
- Contents of `src/main.rs` and `src/lib.rs` form a module named `crate` at the root of the crate's *module tree*
- The module tree is comparable to a file directory tree and items are located via *paths*

## Paths

- Two forms, *absolute* and *relative*, both followed by identifiers separated by `::`
- Beware of privacy boundaries, ie, all items are private by default
  - Siblings may invoke private items
  - Children may invoke parent's private items due to predefined context
  - Parents may not invoke a child's private item
- When using `pub` to expose an item, all items on the path must be exposed for access
- `super` can be used to construct paths from the parent's module (for sibling access)
- For `struct`s and `enum`s, fields of a `struct` are explicitly marked `pub` while all variants of an `enum` are exposed
- Modules may be separated into different files by adding `;` instead of `{...}` in the root crate

## Bringing into scope

- Items are brought into scope using `use` and a relative path
  - Convention is to bring parent in scope for functions and similar types
- Items brought into scope can be renamed using `as` keyword
- Items brought into scope can be *re-exposed* for future convenience using `pub use`
- External packages need to be downloaded by specifiying them in the `Cargo.toml` before using them in scope
- Nested paths are allowed for `use` using `{}`, eg, `use std::io::{self, Write}`
- The glob operator, `*`, brings all exposed items into scope