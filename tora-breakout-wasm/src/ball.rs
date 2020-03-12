use crate::consts::{BALL_RADIUS, INIT_SPEED};
use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Ball {
    dy: f64,
    dx: f64,
    x: f64,
    y: f64,
    speed: usize,
}

#[wasm_bindgen]
impl Ball {
    pub fn new(dx: f64, dy: f64, x: f64, y: f64) -> Ball {
        Ball {
            dx: dx,
            dy: dy,
            x: x,
            y: y,
            speed: INIT_SPEED,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn get_dx(&self) -> f64 {
        self.dx
    }

    pub fn set_dx(&mut self, dx: f64) {
        self.dx = dx;
    }

    pub fn get_dy(&self) -> f64 {
        self.dy
    }

    pub fn set_dy(&mut self, dy: f64) {
        self.dy = dy;
    }

    pub fn get_speed(&self) -> usize {
        self.speed
    }

    pub fn init_speed(&mut self) {
        self.speed = INIT_SPEED;
    }

    pub fn add_speed(&mut self) {
        if self.speed >= 80 {
            return;
        }
        self.speed += 1;
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.set_stroke_style(&"black".into());
        ctx.set_line_width(0.3);
        ctx.arc(self.x, self.y, BALL_RADIUS, 0.0, f64::consts::PI * 2.0)
            .unwrap();
        ctx.set_fill_style(&"rgb(255, 255, 255)".into());
        ctx.fill();
        ctx.close_path();
    }
}
