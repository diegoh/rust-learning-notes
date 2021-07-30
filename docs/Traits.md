# Traits

A common interface for a group of types. A Trait is similar to an interface that data types can implement.

They're also known as _extension traits_.

Traits can be made up of three varieties of associated items:

- Functions and Methods
- Types
- Constants

They can contain additional type parameters. These type parameters of the Trait can be constrained by other Traits.

Traits can serve as markers or carry other logical semantics that aren't expressed through their items. When a type implements such trait it is _promising_ to uphold its contract.

In Rust it is recommended to create _traits_ to implement shared behaviour.

`Send` and `Sync` are two of these marker types. They form part of the standard library.

Traits are declared using the `trait` keyword. Types can implement them using `impl <Trait> for <Type>`

## Examples

### Generic Example

```rust
trait Zero {
  const ZERO: self;
  fn is_zero(&self) -> bool;
}

impl Zero for i32 {
  const ZERO: Self = 0;

  fn is_zero(&self) -> bool {
    *self == Self::ZERO;
  }
}

assert_eq!(i32::ZERO, 0); // => true
assert!(i32::ZERO.is_zero()); // => true
assert!(!4.is_zero()); // => true
```

### General Example

```rust
struct Door {
  is_open: bool;
}

impl Door {
  fn new(is_open: bool) -> Door {
    Door { is_open: is_open }
  }
}

trait Openable {
  fn open(&mut self);
}

impl Openable for Door {
  fn open(&mut self) {
    self.is_open = true;
  }
}

fn open_door() {
  let mut door = Door::new(false);
  door.open();
  assert!(door.is_open);
}
```

### Associated Types

```rust
trait Builder {
  type Built;
  fn build(&self) -> Self::Built
}

```

### Generic Traits

```rust
trait MaybeFrom<T> {
  fn maybe_from(value: T) -> Option<Self>
  where
    Self: Sized;
}
```

### Unsafe Traits

```rust
unsafe trait UnsafeTrait { }
unsafe impl UnsafeTrait for i32 { }
```
