# Unsafe

You can say that code written in Rust is safe thanks to the compiler checking and enforcing certain behaviours in terms of memory and access management. Once in a while these rules will have to be broken making the code unsafe.

`unsafe` is a keyword in Rust to declare that a section of code is able to do most things C would allow.

For example:

- Deference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait

Things like these can be used for low level device access, etc...

The language tries to keep things safe and prevent scenarios such as:

- Deferencing null, dangling or unaligned pointers
- Reading uninitalised memory
- Breaking the pointer aliasing rules
- Producting invalid primitive values
  - Dangling/null references
  - Null `fn` pointers
  - A bool that is not 1 or 0
  - An undefined enum discriminant
  - A char outside the ranges `[0x0, 0xD7FF]` and `[0xE000, 0x10FFFF]`
  - A non-UTF8 string
- Unwinding into another language
- Causing a data race
