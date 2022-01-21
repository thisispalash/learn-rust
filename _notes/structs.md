## Structs

- Structs are custom data types defined with `struct`
- The *fields* within the `struct` are named and annotated, ie, have their types defined
- Instantiating structs involve defining concrete values within `{}` as `key: value` pairs
- A new instance of a struct can also be created as the last expression of a function
- The *field init shorthand* allows for writing less code in the case of *same* name
  - ie, `{ email; ... }` instead of `{ email: email; ... }`
- The *struct update syntax* allows us to copy fields not explicitly set by using `..`
  - This may invalidate the earlier struct instance in case of `move` operation (see [`ownership`](ownership.md))
- *struct tuples* are possible by not naming the fields, eg. `struct Point(i32,i32,i32);`
- Unit like structs are also possible which have no fields. They behave like `()`.
