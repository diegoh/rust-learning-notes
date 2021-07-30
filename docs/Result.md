# Result

Similar to `Option`, however, a `Result` is used to express _**why**_ an operation has failed.

```rust
enum Result<T, E> {
  OK(T),
  Err(E)
}
```

## Variants

### `OK`

Contains the success value.

```rust
let x: Result<i32, &str> = Ok(-3);
assert_eq!(x.is_ok(), true);
```

### `Err`

Contains the error value.

This may be an `Err` variant and should be handled.

```rust
let x: Result<i32, &str> = Err("Some Error Message!");
assert_eq!(x.is_ok(), false);
```

## Handling

Handling the return values is often done with `match`

```rust
fn read_file(path: &str) -> Result<String, io::Error> {
  // open file, read contents and return them
}

match read_file("/tmp/some/folder/") {
  Ok(content) => println!(content),
  Err(error) => println!("Oh no!")
}
```
