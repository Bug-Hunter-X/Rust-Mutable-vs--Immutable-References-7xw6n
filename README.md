# Rust Mutable vs. Immutable References

This example demonstrates the core concept of mutable and immutable references in Rust.  A mutable reference (&mut) allows modification, whereas an immutable reference (&) does not. Attempting to modify a value through an immutable reference results in a compile-time error, preventing data races and ensuring memory safety.