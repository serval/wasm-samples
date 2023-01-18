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
    if test -f "Cargo.toml"; then
        cargo build --release --target wasm32-wasi --quiet
        find target/wasm32-wasi/release/*.wasm | xargs -n1 -I'{}' wasm-opt '{}' -Oz -o '{}'
        cp target/wasm32-wasi/release/*.wasm ../build/
    else
        # If it is not a cargo project, we assume a just build command is provided.
        just build
        # We also assume a just list-wasm command is provided.
        just list-wasm | while read line ; do cp $line ../build/ ; done
    fi
    cd ..
