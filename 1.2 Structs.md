# Structs

There are three flavours of strusts in Rust.

## Types of Structs

### Regular Structs

Structs with name fields are the most commonly used structs. Each field has a name and a type, once defined they can be acessed using `.` in other words, `my_struct.field`.

```rust
struct Regular {
  field1: f32,
  field2: String,
  pub field3: bool,
}
```

#### Mutability

Fields share their mutability with the struct they belong to.

For example, `foo.bar = baz;` would only be valid if the struct `foo` was mutable.

#### Visibility

The word `pub` makes a field visible to other modules and allows them to be directly accessed and modified.

### Tuple Structs

Similar to regular structs with the difference that fields don't have names. They are used like tuples and deconstruction is possible via `let TupleStruct(x, y) = foo`.

```rust
struct Tuple(u32, String)
```

The same syntax is used to access individual variables but with an index instead of a name `foo.0`, `foo.1`, etc....

### Unit Structs

Mostly used as a marker. They have a size of _zero bytes_. Unlike empty enums they can be instantiated which makes them isomorphic to the unit type.

They are most useful when a trait needs to be implemented on something but no data needs to be stored inside it.

```rust
struct Unit;
```

## Instantiation

The most common way to istantiate a struct is via a constructor method `new()`.
In the absence of a constructor or when defining one the struct literal is used.

```rust
let example = Foo {
  field1: 'bar',
  field2: 100,
  etc: true,
}
```

This way of instantiating a struct is only possible when all of such struct's fields are visible.

For convenience, Rust offers a handful of shortcuts for writing constructors.
One of them is the `Field Init` shorthand. When there is a variable with the same name as the field it is being assigned to the variable name can be used instead of repeating the word.

```rust
struct User {
  name: String,
  admin: bool,
}

impl User {
  pub fn new(name: String) -> Self {
    Self {
      name, // just the variable name
      admin: false,
    }
  }
}
```

Another convenience shortcut is `Update Syntax`. It is used when making a new struct based on another struct with the same values.

```rust
let updated = Foo {
  field1: "a new value".to_string(),
  ..thing
}
```
