# wasm-game-of-life

An example of using [Rust][] and [WebAssembly][] together.

[rust]: https://www.rust-lang.org/
[webassembly]: https://webassembly.org/

[![Test](https://github.com/matanlurey/wasm-game-of-life/actions/workflows/test.yml/badge.svg)](https://github.com/matanlurey/wasm-game-of-life/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/matanlurey/wasm-game-of-life/graph/badge.svg?token=K8rfvGXk1I)](https://codecov.io/gh/matanlurey/wasm-game-of-life)

## Overview

An implementation of [Conway's Game of Life][] written in Rust, and deployed
and built to <https://matanlurey.github.io/wasm-game-of-life> using
WebAssembly.

<img src="https://github.com/user-attachments/assets/3d7e5537-beff-48e9-a95b-f1be542e2ee9" width="300" />

[conway's game of life]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

The resulting code, which is not highly optimized, [is ~23kb][dl], or [Brotli][]
compressed down to ~9kb.

[dl]: https://matanlurey.github.io/wasm-game-of-life/pkg/wasm_game_of_life_bg.wasm
[brotli]: https://en.wikipedia.org/wiki/Brotli

Based on [Rust 🦀 and WebAssembly][book].

[book]: https://rustwasm.github.io/docs/book/introduction.html

## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just init` to install dependencies needed by the rest of the commands.
- `cargo just check` to check formatting and lints.
- `cargo just test` to run tests.
- `cargo just serve` to preview the app, re-building on changes.
- `cargo just coverage` to generate and preview code coverage.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).

> TIP: To speedup tools and use prebuilt binaries, [install `cargo-binstall`][].

[install `cargo-binstall`]: https://github.com/cargo-bins/cargo-binstall
