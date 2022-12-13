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
            if test -f "Cargo.toml"; then
                cargo build --release --target wasm32-wasi --quiet
                cp target/wasm32-wasi/release/*.wasm ../build/
            else
                # If it is not a cargo project, we assume a just build command is provided.
                just build
                # We also assume a just list-wasm command is provided.
                just list-wasm | while read line ; do cp $line ../build/ ; done
                cd ..
            fi
        fi
    done
