# The Bytecode Alliance

## Introduction

- [Runtime Instalation](https://wasmtime.dev/)

```sh
curl https://wasmtime.dev/install.sh -sSf | bash
```

## Installing Runtime

## Suports Rust
```sh
rustup target add wasm32-wasi
cargo new demo
cd demo/
cargo build --release --target wasm32-wasi
wasmtime ./target/wasm32-wasi/release/demo.wasm
```

## main.wat source code

(https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-tutorial.md#web-assembly-text-example)


