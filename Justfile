_default:
    cargo just --list -u

build-wasm:
    cargo tool wasm-pack build --release --target web

build-wasm-dev:
    cargo tool wasm-pack build --dev --target web

build-wasm-pkg:
    cargo tool wasm-pack build --release

serve:
    cargo just serve-watch &
    cargo just serve-preview

serve-watch:
    cargo tool cargo-watch -x 'just build-wasm-dev'

serve-preview:
    cargo tool miniserve . --index "index.html" -p 8080

init:
    cargo binstall taplo-cli
    cargo tool --install

lint: lint-check

lint-check:
    cargo clippy --no-deps --all-targets --all-features -- -D warnings

lint-fix:
    cargo clippy --no-deps --all-targets --all-features --fix

format: format-fix

format-check:
    cargo fmt --all -- --check
    taplo format --check

format-fix:
    cargo fmt --all
    taplo format

fix:
    cargo just format-fix
    cargo just lint-fix

check:
    cargo just format
    cargo just lint

doc:
    cargo doc --all-features --no-deps --open --lib

doc-gen:
    cargo clean --doc
    cargo doc --no-deps
    echo '<meta http-equiv="refresh" content="0;url=test_project/index.html">' > target/doc/index.html
    rm target/doc/.lock

test *ARGS:
    cargo tool cargo-nextest run 

test-doc *ARGS:
    cargo test --doc {{ARGS}}

test-wasm *ARGS:
    cargo tool wasm-pack test {{ARGS}}

test-all:
    cargo just test --all-features
    cargo just test-doc --all-features
    cargo just test-wasm --chrome --headless
    
coverage *ARGS:
    cargo tool cargo-llvm-cov --open

coverage-gen:
    cargo tool cargo-llvm-cov --lcov --output-path lcov.info
