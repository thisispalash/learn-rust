Collections are stored on the heap rather than the stack.

## Vectors

- `Vec<T>` puts all values next to each other in memory, ie, a variable array
- Create new vectors with `Vec::new()`
- Add elements to vectors using `.push()`; remember to define it as `mut`
- Vector elements can be read using `&` and `[]` or matching the result of `.get()` which is `Option<&T>`
- For *index out of bounds*, `.get()` is better since it returns `None`
- Vectors can be iterated over by using `for` loops
- `enum`s help store different data types in the same vector
- `pop` removes the last element in the vector

## Strings

- Strings are implemented as a collection of bytes on the heap
- New strings are created from `::new()` or string slices (`::from(...)` or `.to_string()`)
- `.push()` takes in a character and `.push_str()` takes in a string slice and appends it to the original String
- Using the `format!` macro is better than using `+` due to ownership
- Rust provides three ways to look at strings — bytes, scalar values, and grapheme clusters
- Bytes are actually stored in memory, but indexing is not allowed due to UTF-8
- Slicing can be tricky with ranges within `[]` as storage can be 2 bytes or 3 bytes per character as well
- `.chars()` and `.bytes()` returns the scalar values and bytes respectively

## Hashmaps

- `HashMap<K,V>` stores a mapping of keys `K` to values `V` via a hashing function
- Hash maps need to be brought into scope using `use`
- Create new hashmaps with `::new()` and add items with `::insert(k,v)`
- Ownership is transferred into the hashmap, so be careful
- `.get()` is used to read values of a hashmap, but it returns `Option<&V>`
- `.insert()` overwrites a value that already exists
- `.entry()` checks for existence of key and returns an `Entry` enum, on which `.or_insert()` can be used
- The mutable reference from `.entry()` can be dereferenced using `*` for updating the value
- The hashing algorithm can also be changed from the default *SipHash*
