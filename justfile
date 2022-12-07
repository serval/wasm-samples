# List available recipes
help:
    just -l

@build:
    #!/bin/bash
    set -e

    mkdir -p build
    for example in *; do
        if [ -d "$example" ] && [ "$example" != "build" ]; then
            cd $example
            cargo build --release --target wasm32-wasi
            cp target/wasm32-wasi/release/*.wasm ../build/
            cd ..
        fi
    done
