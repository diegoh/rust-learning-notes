# Exceptions & Panic

Exceptions are also missing and failed function executions should be indicated in the return type. There are exceptions to this and Rust provides the macro `panic!` for cases when immediate termination is required.

## Panic

A macro that stops the program immediately and provides feedback to the caller. `panic!` should be used when the program reaches an unrecoverable state.

It is a perfect way to assert conditions in example code and tests. It is closely tied with the `unwrap` method of `Option` and `Result`, both implementations call `panic!` when they're set to their respective`None` or `Err` variants.

If the main thread panics it will terminate all your threads and end your program with code `101`.
