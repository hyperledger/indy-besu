# Indy2-VDR

> Note: The library is under active development!

## Introduction

This is Rust library representing a convenient client for connecting to Indy2 Ledger and executing
transactions/queries/contracts.

The library provides methods to:

- connect to node
- build transactions executing predefined contract methods
- obtain transaction bytes to make EcDSA Recoverable signature
- send transactions to connected node
- parse response returned from the node

## Prerequisites

- Indy2 Ledger running - see [instructions](../README.md) on how to run local network.

## Build

In order to build library, you must have [Rust](https://rustup.rs/) installed. 

Used Rust version: `1.70.0`

```
cargo build
```

## Usage

To use vdr, add this to your `Cargo.toml`:

```
[dependencies]
indy2_vdr = { path = "../path/to/crate" }
```

## Code formatting

Library uses [Rustfmt](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=) to define code formatting rules.

In order to run code formatting, run the following command:
```
cargo +nightly fmt
```

## Features

- `migration` (Optional) - module providing helper methods to convert old indy styled objects (schema id, schema,
  credential definition id, credential definition).
- `ledger_test` (Optional) - ledger integration tests requiring running network.
- `basic_signer` (Optional) - basic helper module for EcDSA signing.
- `wasm` (Optional) - library which can be compiled for [Web-Assembly](https://rustwasm.github.io/book/)
- `uni_ffi` (Optional) - library can be compiled with [UniFFI](https://mozilla.github.io/uniffi-rs/)

## Test

- Basic: run ledger agnostic test:
  ```
  cargo test
  ```

- Integrations: run tests interacting with the ledger
  ```
  RUST_TEST_THREADS=1 cargo test --features "ledger_test"
  ```

# Logging

- To see the logs, please set `RUST_LOG` environment variable to desired log level: `info`, `debug`, `trace` etc.

## FFI

### Kotlin, Python, Swift

`Indy-VDR` library uses [uniffi](https://mozilla.github.io/uniffi-rs/) to generate bindings for Kotlin, Python, Swift languages.

In order to generate language specific bindings, run the following commands with replacing a target
language `<kotlin|python|swift>`:

```
cargo build --features "uni_ffi" --release
cargo run --features "uni_ffi" --bin uniffi-bindgen generate --library target/release/libindy2_vdr.dylib --language <kotlin|python|swift> --out-dir out
```

The `out` directory containing generated bindings will be created as the command execution result.

### JavaScript, NodeJs, WebAssembly

`Indy-VDR` library uses [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) to generate bindings for JavaScript, NodeJs, WebAssembly.

See instructions [here](./wasm/README.md).