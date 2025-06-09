# wasm-game-of-life

An example of using [Rust][] and [WebAssembly][] together.

[rust]: https://www.rust-lang.org/
[webassembly]: https://webassembly.org/

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
