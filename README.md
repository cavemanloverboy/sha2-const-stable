# sha2-const-stable

`const fn` implementation of the SHA-2 family of hash functions.

This crate allows you to use the SHA-2 hash functions as constant expressions
in Rust. It is adapted from `sha2-const`, which requires nightly. For all other usages,
the [`sha2`] crate includes more optimized implementations of these hash functions.

[`sha2`]: https://crates.io/crates/sha2
