//! TBD.

#![cfg_attr(target_arch = "wasm32", no_main)]

pub mod renderer;
pub mod universe;

use crate::universe::Cell;
use crate::{
    renderer::{Options, Renderer},
    universe::Universe,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

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
pub struct GameBuilder {
    universe: Option<Universe>,
    renderer: Option<Renderer>,
    options: Options,
}

#[wasm_bindgen]
impl GameBuilder {
    /// Creates a new `GameBuilder` instance with default options.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            universe: None,
            renderer: None,
            options: Options::new(),
        }
    }

    /// Sets a universe with the provided size and fill function.
    ///
    /// The `fill` parameter should be a JavaScript function with the signature:
    ///
    /// `(x: number, y: number) => bool`, where the return value corresponds to [`Cell::Alive`].
    #[wasm_bindgen(js_name = setUniverse)]
    pub fn set_universe(&mut self, width: u32, height: u32, fill: &js_sys::Function) {
        let universe = Universe::with_size_filled(width, height, &mut |x, y| {
            let alive: bool = fill
                .call2(&JsValue::UNDEFINED, &JsValue::from(x), &JsValue::from(y))
                .unwrap_or(JsValue::UNDEFINED)
                .as_bool()
                .unwrap_or(false);
            if alive { Cell::Alive } else { Cell::Dead }
        });
        self.universe = Some(universe);
    }

    /// Sets a renderer with the provided canvas element.
    ///
    /// # Errors
    ///
    /// Returns an error if the canvas context cannot be obtained.
    #[wasm_bindgen(js_name = setRenderer)]
    pub fn set_renderer(&mut self, canvas: &HtmlCanvasElement) -> Result<(), String> {
        let renderer = Renderer::try_with_canvas(canvas)?;
        self.renderer = Some(renderer);
        Ok(())
    }

    /// Builds the `GameOfLife` instance with the configured universe and renderer.
    ///
    /// # Errors
    ///
    /// Returns an error if the universe or renderer is not set.
    #[wasm_bindgen(js_name = build)]
    pub fn build(self) -> Result<GameOfLife, String> {
        if let (Some(universe), Some(renderer)) = (self.universe, self.renderer) {
            Ok(GameOfLife {
                universe,
                renderer,
                options: self.options,
            })
        } else {
            Err("Universe and Renderer must be set before building".into())
        }
    }
}

#[wasm_bindgen]
pub struct GameOfLife {
    universe: Universe,
    renderer: Renderer,
    options: Options,
}

#[wasm_bindgen]
impl GameOfLife {
    /// Ticks the game forward by one step and renders the current state of the universe.
    pub fn tick(&mut self) {
        self.universe.tick();
        self.renderer.render(&self.options, &self.universe);
    }
}
