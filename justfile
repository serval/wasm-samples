# List available recipes
help:
    just -l

# Build all examples & put them in ./build
build:
    #!/bin/bash
    set -e

    dirs=$(echo */)
    mkdir -p build
    for example in $dirs; do
        example=${example%/}
        if [ -d "$example" ] && [ "$example" != "build" ]; then
            cd $example
            echo "building ${example}.wasm"
            cargo build --release --target wasm32-wasi --quiet
            cp target/wasm32-wasi/release/*.wasm ../build/
            cd ..
        fi
    done
