# wasm-samples

This is a collection of sample WASM applications, intended to be used to demonstrate various pieces of functionality as we develop them.

Each top-level directory should be a standalone Rust project (e.g. the result of `cargo new <something>`).

## Initial setup

This project uses [just](https://github.com/casey/just) (`brew install just`) for development
workflows and automation. Run `just` with no arguments to see a list of available commands.

Your development machine will need to have several tools installed; `just install-deps` will install
them for you.
