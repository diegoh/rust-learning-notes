# Impl

_Implement some functionality for a type._

Mainly used to define implementations on types.

Functions and consts can both be defined in an implementation. A function defined in an `impl` block can be standalone, meaning, it can be called like `Foo::bar()`. If the function takes `self`, `&self` or `&mut self` as its **first** argument it can also be called using _method-call_ syntax `foo.bar()`.

```rust
struct Example {
  number: i32,
}

impl Example {
  fn boo() {
    println!("boo! Example::boo() was called");
  }

  fn answer(&mut self) {
    self.number += 42;
  }

  fn get_number(&self) -> i32 {
    self.number
  }
}

trait Thingy {
  fn do_thingy(&self);
}

impl Thingy for Example {
  fn do_thingy(&self) {
    println!("doing a thing! also, number is {}", self.number);
  }
}
```

## Closures

The other use for `impl` is in `impl Trait`, a shorthand for "_a concrete type that implements this trait_". Primarily, this is used when working with closures with type definitions generated at compile time that cannot be simply typed out.

```rust
fn thing_returning_closure() -> impl Fn(i32) -> bool {
  println!("here's a closure!");
  |x: i32| x % 3 == 0;
}
```
