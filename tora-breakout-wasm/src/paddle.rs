use crate::consts::{PADDLE_HEIGHT, PADDLE_WIDTH};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Paddle {
    x: f64,
    right_pressed: bool,
    left_pressed: bool,
}

#[wasm_bindgen]
impl Paddle {
    pub fn new(x: f64) -> Paddle {
        Paddle {
            x: x,
            right_pressed: false,
            left_pressed: false,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn get_right_pressed(&self) -> bool {
        self.right_pressed
    }

    pub fn set_right_pressed(&mut self, right_pressed: bool) {
        self.right_pressed = right_pressed;
    }

    pub fn get_left_pressed(&self) -> bool {
        self.left_pressed
    }

    pub fn set_left_pressed(&mut self, left_pressed: bool) {
        self.left_pressed = left_pressed;
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d, height: f64) {
        ctx.begin_path();
        ctx.rect(self.x, height - PADDLE_HEIGHT, PADDLE_WIDTH, PADDLE_HEIGHT);
        ctx.set_fill_style(&"rgb(255, 136, 0)".into());
        ctx.fill();
        ctx.close_path();
    }
}
