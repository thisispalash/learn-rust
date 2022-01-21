## Ownership

- In Rust, everything is based in ownership
- Immutable types are stored on the stack, including string literals, which are *popped* once out of scope
- For data allocated on the heap, pointers are used. Deallocation occurs implicitly with `drop` once out of scope
- Variable data (eg, `String` type) stores some info on the stack and a pointer to the data in the heap
- Copying can be automatic, in which case the first allocation is invalidated; effectively a move
- Copying can also be done using `clone` which copies the heap data as well
- Function calls take ownership and return values return ownership for non static types

## References

- We can pass references to the function using `&` which only *borrows* the value
- Effectively, a pointer is created to the original variable which is `drop`ed at the end of function scope
- References are immutable by default, but can be made mutable using `mut`
- To prevent data races, only one mutable reference to a piece of data is allowed at a time
- Mutable references are not allowed after immutable references as well
- As always, scopes can be modified to allow multiple mutable references, just not *simultaneous* ones
- There can be any number of immutable references, since they cannot change the data

## Slices

- We can slice collections using `..`
- Slices create an immutable reference, so errors are caught at compile time
- String literals are slices, ie, they have type `&str`
- Slices ensure memory safe programs by introducing references