# Nullable Pointers

Rust's pointer type must always point to a valid location as there are no `null` references. Instead Rust has optional pointers.

Rust works with `Option<T>` and `Result<T, E>` enums for dealing with success and failure. Both are ennumerations with generic type parameters. `Option` and `Result` can encapsulate a single `Option<T>` or two `Result<T, E>` values.

## Example

The code below uses `Option` to create an aditional `i32` box.
Note that `check_optional` uses _pattern matching_ to determine whether the box has a value.

```rust
let optional = None;
check_optional(optional);

let optional = Some(Box::new(9000));
check_optional(optional)

fn check_optional(optional: Option<Box<i32>>) {
  match optional {
    Some(p) => println!("has value {}", p);
    None => println!("has no value");
  }
}
```
