# wasm-samples

This is a collection of sample WASM applications, intended to be used to demonstrate various pieces of functionality as we develop them.

Each top-level directory should be a standalone Rust project (e.g. the result of `cargo new <something>`).

## Initial setup
Your development machine will need to have the wasm32-wasi target installed:

```
rustup target add wasm32-wasi
```