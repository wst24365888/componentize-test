# Componentize Test

This is a test of componentizing Rust, JS and Python code to WASM component.

## Setup

Clone the repository and cd into it.

```bash
git clone
cd componentize-test
```

### Rust

```bash
cargo install cargo-component
cargo component build --release -j 16
# find the built component at target\wasm32-wasi\release\componentize_test.wasm
```

### JS

```bash
npm install -g @bytecodealliance/jco @bytecodealliance/componentize-js
jco componentize source.js --wit wit -o component-js.wasm
# find the built component at component-js.wasm, NOT WORKING
```
