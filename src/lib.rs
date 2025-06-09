//! TBD.

#![cfg_attr(target_arch = "wasm32", no_main)]

use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[cfg(test)]
pub(crate) mod tests;

#[wasm_bindgen(start)]
fn main() {
    // When the `console_error_panic_hook` feature is enabled, we can call the `set_panic_hook`
    // function at least once during initialization, and then we will get better error messages if
    // our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(all(feature = "console_error_panic_hook", debug_assertions))]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Bilbo!");
}
