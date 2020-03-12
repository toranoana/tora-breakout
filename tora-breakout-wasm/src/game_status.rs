use crate::consts::DEFAULT_LIVES;
use crate::consts::DEFAULT_SCORE;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Prepare,
    Stop,
    Start,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct GameStatus {
    score: u32,
    lives: u32,
    status: Status,
}

#[wasm_bindgen]
impl GameStatus {
    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn set_lives(&mut self, lives: u32) {
        self.lives = lives;
    }

    pub fn get_lives(&self) -> u32 {
        self.lives
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn new() -> GameStatus {
        GameStatus {
            score: DEFAULT_SCORE,
            lives: DEFAULT_LIVES,
            status: Status::Prepare,
        }
    }

    pub fn draw_score(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_font("16px Arial");
        ctx.set_fill_style(&"rgb(0, 149, 208)".into());
        let _ = ctx.fill_text(&format!("Score: {}", self.score), 8.0, 20.0);
    }

    pub fn draw_lives(&self, ctx: &web_sys::CanvasRenderingContext2d, width: f64) {
        ctx.set_font("16px Arial");
        ctx.set_fill_style(&"rgb(0, 149, 208)".into());
        let _ = ctx.fill_text(&format!("Lives: {}", self.lives), width - 65.0, 20.0);
    }
}
