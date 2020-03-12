use crate::consts::{
    BRICK_HEIGHT, BRICK_OFFSET_LEFT, BRICK_OFFSET_TOP, BRICK_PADDING, BRICK_WIDTH,
};
// use wasm_bindgen::prelude::*;
use web_sys::ImageBitmap;

// #[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[repr(u8)]
pub enum BrickStatus {
    Live,
    Dead,
}

// #[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Brick {
    x: f64,
    y: f64,
    status: BrickStatus,
    life: u32,
}

// TODO: wasm_bindgenなimplとasyncが絡むとエラーになるのでとりあえずコメント
// #[wasm_bindgen]
impl Brick {
    pub fn new(x: f64, y: f64, status: BrickStatus) -> Brick {
        Brick {
            x: x,
            y: y,
            status: status,
            life: 1,
        }
    }

    // TODO: 本当はコンストラクタでやりたいかも vecの初期化をresizeでやってるのでそこらへんを変えれば？
    pub fn set_x_and_y_from_col_row(&mut self, col: f64, row: f64) {
        self.x = col * (BRICK_WIDTH + BRICK_PADDING) + BRICK_OFFSET_LEFT;
        self.y = row * (BRICK_HEIGHT + BRICK_PADDING) + BRICK_OFFSET_TOP;
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

    pub fn set_status(&mut self, status: BrickStatus) {
        self.status = status;
    }

    pub fn get_status(&self) -> BrickStatus {
        self.status
    }

    pub fn update_status(&mut self) -> BrickStatus {
        self.life -= self.life;
        if self.life == 0 {
            self.status = BrickStatus::Dead;
        }
        self.status
    }

    pub fn draw(&mut self, bitmap: &ImageBitmap, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();
        let _ = ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            bitmap,
            self.x,
            self.y,
            BRICK_WIDTH,
            BRICK_HEIGHT,
        );
        if self.status == BrickStatus::Live {
            ctx.set_stroke_style(&"rgb(95,95,95)".into());
            ctx.set_line_width(0.5);
            ctx.stroke_rect(self.x, self.y, BRICK_WIDTH, BRICK_HEIGHT);
        }
        ctx.close_path();
    }

    /// ブロックが破壊されたとき用のブロック描画メソッド
    pub fn draw_with_break(
        &mut self,
        bitmap: &ImageBitmap,
        ctx: &web_sys::CanvasRenderingContext2d,
    ) {
        let _ = ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            bitmap,
            self.x,
            self.y,
            BRICK_WIDTH,
            BRICK_HEIGHT,
        );
    }
}
