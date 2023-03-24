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
        just --quiet list-wasm | xargs -I'{}' wasm-opt '{}' -Oz -o '{}'
        just --quiet list-wasm | xargs -I'{}' cp '{}' ../build/
    else
        # If there is no justfile, we simply `cargo build` and grab the output.
        cargo build --release --target wasm32-wasi --quiet
        chmod -x target/wasm32-wasi/release/*.wasm # https://serval.slack.com/archives/C04BKH1J31S/p1674672880593649
        find target/wasm32-wasi/release/*.wasm | xargs -I'{}' wasm-opt '{}' -Oz -o '{}'
        cp target/wasm32-wasi/release/*.wasm ../build/
    fi
    cd ..

build-and-run PROJECT BINARY='':
    #!/bin/bash
    just build {{PROJECT}}
    if [ "{{BINARY}}" == "" ]; then
        BINARY=build/{{PROJECT}}.wasm
    else
        BINARY={{BINARY}}
    fi
    pushd ../serval-mesh
    cargo run --bin serval -- run ../wasm-samples/${BINARY}

install-deps:
    #!/bin/bash
    set -e
    brew install binaryen
    rustup --quiet target add wasm32-wasi

clean:
    rm build/*.wasm
