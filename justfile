# List available recipes
help:
    just -l

# Build all examples & put them in ./build
build PROJECT='': deps
    #!/bin/bash
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
    printf "Building {{GREEN}}${example}.wasm{{CLEAR}} ...\n"
    if test -f "justfile"; then
        # > (you should assume just is there)
        #                     --ceejbot, 2023
        just build
        find build/release/*.wasm 2>/dev/null | xargs -I'{}' wasm-opt '{}' -Oz -o '{}'
        find build/release/*.wasm 2>/dev/null | xargs -I'{}' cp '{}' ../build/
    else
        # If there is no justfile, we simply `cargo build` and grab the output.
        cargo build --release --target wasm32-wasi --quiet
        chmod -x target/wasm32-wasi/release/*.wasm # https://serval.slack.com/archives/C04BKH1J31S/p1674672880593649
        find target/wasm32-wasi/release/*.wasm | xargs -I'{}' wasm-opt '{}' -Oz -o '{}'
        cp target/wasm32-wasi/release/*.wasm ../build/
    fi
    cd ..

list-wasm:
    find build/release/*.wasm 2>/dev/null

deps:
    #!/bin/bash
    set -e
    if ! hash wasm-opt 2>/dev/null; then
        cargo install wasm-opt
    fi

build-and-run PROJECT BINARY='':
    #!/bin/bash
    set -e
    just build {{PROJECT}}
    if [ "{{BINARY}}" == "" ]; then
        BINARY=build/{{PROJECT}}.wasm
    else
        BINARY={{BINARY}}
    fi
    pushd ../serval-mesh
    cargo run --bin serval -- run ../wasm-samples/${BINARY}

clean:
    rm build/*.wasm

GREEN := "\\x1b[32m"
CLEAR := "\\x1b[0m"
