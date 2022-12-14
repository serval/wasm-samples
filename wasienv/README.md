# Compiling a C or C++ program with Wasienv

**PSA: Since the toolchain installs with a curled shell script, this directory should just be skipped if the toolchain is not present. Do not automatically install complex toolchains on people's computers they may not be aware of.**

In their own words, [Wasienv](https://github.com/wasienv/wasienv) "is a tool that aims to bring all programming languages to WebAssembly WASI". Currently, this only includes C/C++ and Swift.

Wasienv comes with a number of compilation tools for all kinds of project flavors (including support for `./configure`, GNU Make or CMake-based projects, etc.)

Currently, this directory only contains a simple "Hello, World" type example, but more complex C++/Swift-based examples would probably be fun to demo. Note: They currently need to be manually set up in the justfile.