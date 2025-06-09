# wasm-game-of-life

An example of using [Rust][] and [WebAssembly][] together.

[rust]: https://www.rust-lang.org/
[webassembly]: https://webassembly.org/

[![Test](https://github.com/matanlurey/wasm-game-of-life/actions/workflows/test.yml/badge.svg)](https://github.com/matanlurey/wasm-game-of-life/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/matanlurey/wasm-game-of-life/graph/badge.svg?token=K8rfvGXk1I)](https://codecov.io/gh/matanlurey/wasm-game-of-life)

Based on [Rust 🦀 and WebAssembly][book].

[book]: https://rustwasm.github.io/docs/book/introduction.html

## Overview

...

## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just init` to install dependencies needed by the rest of the commands.
- `cargo just check` to check formatting and lints.
- `cargo just test` to run tests.
- `cargo just doc` to generate and preview docs.
- `cargo just coverage` to generate and preview code coverage.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).

> TIP: To speedup tools and use prebuilt binaries, [install `cargo-binstall`][].

[install `cargo-binstall`]: https://github.com/cargo-bins/cargo-binstall
