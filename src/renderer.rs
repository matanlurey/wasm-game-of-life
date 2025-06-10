use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::universe::{Cell, Universe};

#[derive(Debug, Clone)]
pub struct Options {
    cell_size_pixels: u32,
    grid_color: String,
    dead_color: String,
    alive_color: String,
}

impl Options {
    /// Creates a new `Options` instance with default values.
    #[must_use]
    pub fn new() -> Self {
        Options {
            cell_size_pixels: 5,
            grid_color: "#CCCCCC".into(),
            dead_color: "#FFFFFF".into(),
            alive_color: "#000000".into(),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Options::new()
    }
}

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

impl Renderer {
    /// Creates a new `Renderer` with the provided `CanvasRenderingContext2d`.
    ///
    /// # Errors
    ///
    /// Returns an error if a 2D context cannot be obtained from the canvas.
    pub fn try_with_canvas(canvas: &HtmlCanvasElement) -> Result<Self, &'static str> {
        let ctx = canvas
            .get_context("2d")
            .map_err(|_| "Failed to get 2D context")?
            .ok_or("No 2D context found")?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| "Failed to cast to CanvasRenderingContext2d")?;

        let width = canvas.width();
        let height = canvas.height();

        Ok(Renderer { ctx, width, height })
    }

    /// Renders the given universe to the renderer.
    pub fn render(&mut self, options: &Options, universe: &Universe) {
        self.clear();
        self.draw_grid(options, universe);
        self.draw_cells(options, universe);
    }

    fn clear(&mut self) {
        let ctx = &mut self.ctx;
        ctx.clear_rect(0.0, 0.0, self.width.into(), self.height.into());
    }

    fn draw_grid(&mut self, options: &Options, universe: &Universe) {
        let ctx = &mut self.ctx;
        ctx.begin_path();
        ctx.set_stroke_style_str(&options.grid_color);

        // Vertical lines.
        for i in 0..=universe.width() {
            let x = i * (options.cell_size_pixels + 1);
            let x: f64 = x.into();
            ctx.move_to(x, 0.0);
            ctx.line_to(
                x,
                ((options.cell_size_pixels + 1) * universe.height() + 1).into(),
            );
        }

        // Horizontal lines.
        for j in 0..=universe.height() {
            let y = j * (options.cell_size_pixels + 1) + 1;
            let y: f64 = y.into();
            ctx.move_to(0.0, y);
            ctx.line_to(
                ((options.cell_size_pixels + 1) * universe.width() + 1).into(),
                y,
            );
        }

        ctx.stroke();
    }

    fn draw_cells(&mut self, options: &Options, universe: &Universe) {
        let ctx = &mut self.ctx;

        for (state, row, col) in universe.iter_cells() {
            let x = col * (options.cell_size_pixels + 1) + 1;
            let y = row * (options.cell_size_pixels + 1) + 1;

            ctx.set_fill_style_str(match state {
                Cell::Alive => &options.alive_color,
                Cell::Dead => &options.dead_color,
            });

            let size: f64 = options.cell_size_pixels.into();
            ctx.fill_rect(x.into(), y.into(), size, size);
        }

        ctx.stroke();
    }
}
