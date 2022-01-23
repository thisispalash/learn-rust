## Ennumerations and Variants

- enums can be only of one variant type
- Variants of the enum are namespaced by using `::`
- enum variants can store data of any type, including structs
- Methods on enums are also possible using `impl`
- Rust provides the `Option<T>` enum to provide `null` functionality with variants `None` and `Some(T)`
- `Option<T>` is included in the prelude so `let x = Some(5)` is possible

## `match` control flow

- The `match` operator allows you to compare a value against patterns
- The power comes from the compiler *ensuring* all possible patterns are accounted for
- The match arms can bind the values that match the pattern
  - So, for the `Option<T>` enum, we can extract the value in `x` using `Some(x) => {}`
- Matches are *exhaustive* but there exist catch-all patterns for default operations
- The `_` placeholder is a catch-all pattern that does not bind any value

## `if let` sugar

- In case of matching only one pattern, use `if let ... = ... {}`
- The trade off is losing exhaustiveness of `match` control flow
- `else` can be used to match all other patterns
- The main objective is to reduce boilerplate code