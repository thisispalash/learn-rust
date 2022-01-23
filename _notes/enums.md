## Ennumerations and Variants

- enums can be only of one variant type
- Variants of the enum are namespaced by using `::`
- enum variants can store data of any type, including structs
- Methods on enums are also possible using `impl`
- Rust provides the `Option<T>` enum to provide `null` functionality with variants `None` and `Some(T)`
- `Option<T>` is included in the prelude so `let x = Some(5)` is possible