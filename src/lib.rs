use evosim_core::{simulation::Simulation};
use wasm_bindgen::{JsCast, prelude::*};

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Evosim {
    simulation: Simulation,
}

#[wasm_bindgen]
impl Evosim {
    pub fn new() -> Evosim {
        Evosim {
            simulation: Simulation::new(50, 50),
        }
    }

    pub fn start(&mut self) {
        self.simulation.start();
    }

    pub fn render(&mut self) {
        self.simulation.step();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("evosim-canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_width((TILE_WIDTH + 1)*self.simulation.board.width() as u32 + 1);
        canvas.set_height((TILE_HEIGHT + 1)*self.simulation.board.height() as u32 + 1);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        for x in 0..self.simulation.board.width() {
            for y in 0..self.simulation.board.height() {
                let v = self.simulation.board.color_at(x, y);
                context.set_fill_style(&JsValue::from_str(&format!(
                    "rgb({}, {}, {})",
                    v.r, v.g, v.b
                )));
                context.fill_rect(x as f64 * TILE_WIDTH as f64, y as f64 * TILE_WIDTH as f64, TILE_WIDTH as f64, TILE_WIDTH as f64);
            }
        }
    }
    
}