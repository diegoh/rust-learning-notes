# Option

Represents an optional value. There are two variants.

## Variants

### `Some(value)`

Contains a value. A tuple struct wrapping a `value` with type `T`.

### `None`

Indicates the lack of a `value` or failure.

## Uses

`Option` types are very common in Rust and they have a number of uses:

- Initial values
- Return values for partial functions not defined over their entire range
- Return values for otherwise returning simple errors, `None` is returned on error
- Optional `struct` fields
- Struct fields that can be `loaned` or `taken`
- Optional function arguments
- Nullable Pointers
- Swapping things out of difficult situations

```rust
enum Option<T> {
  Some(T),
  None
}
```

## Handling

`Option`s are usually paired with pattern matching to query the presence of a value and do something with it always taking into consideration the `None` case.
Handling the return values is often done with `match` or `if let` clauses.

```rust
fn find(needle: u16, haystack: Vec<u16>) -> Option<usize> {
  // find the needle in the haystack
}

match find(2, vec![1,3,4,5]) {
  Some(_) => println!("Found"),
  None => println!("Not Found!")
}


// using if let ...
if let Some(result) = find(2, vec![1,2,3,4]) {
  println("Found")
}
```
