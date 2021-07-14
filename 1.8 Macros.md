# Macros

In Rust, the term _macro_ refers to a family of features.

Macros are a way of writing code that writes other code, this is called _metaprogramming_.

Metaprogramming is useful for reducing the amount of code you need to write and maintain, although this is also the role of functions, macros provide additional powers that functions lack.

Functions must declare each of the parameters they can take and their types, macros can take a variable amount of parameters.

Macros are expanded before the compiler interprets the meaning of the code so a macro can implement a trait on a given type. A function is unable to do so because it gets called at runtime and a trait needs to be implemented at compile time.

The tradeoff is in the complexity of writing a macro. Macro definitions are harder to write, read, understand and maintain compared to functions.

Macros must be defined or brought into scope before they're called, functions can be defined and called anywhere.

## Types of Macros

### Declarative macros with `macro_rules!`

They go by many names such as _Macros by example._, _`macro_rules!` macros_ or simply _macros_.

At their core they allow you to write something similar to Rust's `match` expression, comparing the resultiung value to patterns and then run code associated with the matching pattern.

In this case the value is literal Rust code; the patterns are compared with the structure of such code and associated code. Then the code is associated with each pattern, when matched, it replaces the code passed to the pacro.

All this happens during compilation.

The following macro creates a new vector containing three integers.

```rust
let v: Vec<i32> = vec![1, 2, 3];
```

the `vec!` macro can also be used to create a vector of 2 integers or five strings.
A function would simply not be able to do this because it would need to declare each argument and its type.

```rust
#[macro_export]
macro_rules! vec { // vec is the name
  ( $( $x:expr ), * ) => { // pattern
    { // body associated with the pattern
      let mut temp_vec = Vec::new();
      $( // replace each matching $x with the code below
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}
```

The `#[macro_export]` annotation indicates that this macro should be available whenever the _create_ in which the macro is defined is brought into scope.

The macro above only has one pattern, if the pattern matches, the associated block of code will be emited, otherwise the result will be an error.

What's going on in: `( $( $x:expr ), * )`???

- `()`: parentheses surround the whole pattern
  - `$()`: captures values that match the pattern within the parentheses for use in the replacement code.
    - `$x:expr`: Matches any Rust expression and gives it the name `$x`
  - `,`: Literal comma separator
  - `*`: Specifies that the pattern matches zero or more elements that precede `*`

For vector `vec![1, 2, 3]` the pattern `( $( $x:expr ), * )` matches three times, `1`, `2` and `3`.

In ters of the body:

- `$()`: `temp_vec.push()` is generated for each part that matches `$()`.

When the pattern matches, `$x` is replaced with each expression matched and the macro call is replaced with the following code:

```rust
{
  let mut temp_vec = Vec::new();
  temp_vec.push(1);
  temp_vec.push(2);
  temp_vec.push(3);
  temp_vec
}
```

### Procedural Macros

Used to generate code from attributes.
They act more like functions, they accept code as input, operate on that code and product some code as an output rather than matching against partterns and replacing code with other code.

When creating procedural macros the definitions must reside in their own crate with a special crate type.

Currently the creation of procedural macros is complex due to technical reasons but the team aim to change this in the future.

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
  //...
}
```

The macro in the code above defines a procedural macro that takes a `TokenStream` as an input and returns a `TokenStream`.

`TokenStream` is defined by the `proc_macro` (included within Rust).

### Custom `derive` Macro

They specify coded added with the `derive` attribute used on structs and enums.

They can be used to allow macro users annotate their type with a `#[derive(HelloMacro)]` to get an implementation of the `hello_macro` function.

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes

fn main() {
  Pancakes::hello_macro();
}
```

## TODO: Continue documenting macros

- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens specified as their argument
