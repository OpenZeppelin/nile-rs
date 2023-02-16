# OpenZeppelin | Nile ⛵ - Rust version

[![Coverage Status](https://codecov.io/gh/OpenZeppelin/nile-rs/graph/badge.svg)](https://codecov.io/gh/OpenZeppelin/nile-rs)
[![Tests and linter](https://github.com/OpenZeppelin/nile-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenZeppelin/nile-rs/actions/workflows/ci.yml)

## Overview

Nile is a CLI tool to develop or interact with StarkNet projects written in Cairo. This is an ongoing effort to migrate the existing tool written in Python to Rust, for compatibility with the new Cairo1 compiler.

## Feature parity

For the current status of the features migration from Python check [this page](https://github.com/ericnordelo/nile-rs/blob/main/docs/FEATURE_PARITY.md).

## Testing features

While we have the intention to release to [crates.io](https://crates.io) soon, but in the meantime, you can clone this repository and check the currently available features from the `main` branch running:

```
cargo run -- --help
```

## License

Nile is released under the MIT License.