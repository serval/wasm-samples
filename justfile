# List available recipes
help:
    just -l

# Build all examples & put them in ./build
build PROJECT='':
    #!/bin/bash
    set -e

    if [ "{{PROJECT}}" == "" ]; then
        # If not given a particular project to build, build everything.
        dirs=$(echo */)
        for example in $dirs; do
            example=${example%/}
            if [ -d "$example" ] && [ "$example" != "build" ]; then
                just build $example
            fi
        done
        exit
    fi

    mkdir -p build
    example={{PROJECT}}
    cd $example
    echo "Building ${example}.wasm ..."
    if test -f "justfile"; then
        # > (you should assume just is there)
        #                     --ceejbot, 2023
        just build
        # We also assume a just list-wasm command is provided.
        just list-wasm | while read line ; do cp $line ../build/ ; done
    else
        # If there is no justfile, we simply `cargo build` and grab the output.
        cargo build --release --target wasm32-wasi --quiet
        cp target/wasm32-wasi/release/*.wasm ../build/
    fi
    cd ..
