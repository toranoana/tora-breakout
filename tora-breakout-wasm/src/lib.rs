mod ball;
mod bricks;

mod bitmap_container;
mod consts;
mod game_status;
mod paddle;
mod utils;

use crate::ball::Ball;
use crate::bitmap_container::get_image;
use crate::bricks::{Brick, BrickStatus};
use crate::consts::*;
use crate::game_status::{GameStatus, Status};
use crate::paddle::Paddle;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, ImageBitmap};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate serde_derive;
extern crate serde;

enum BrickEdge {
    Top,
    Bottom,
    Left,
    Right,
}

fn is_nealy_edge(top_dist: f64, bottom_dist: f64, left_dist: f64, right_dist: f64) -> BrickEdge {
    if top_dist < bottom_dist && top_dist < left_dist && top_dist < right_dist {
        BrickEdge::Top
    } else if bottom_dist < top_dist && bottom_dist < left_dist && bottom_dist < right_dist {
        BrickEdge::Bottom
    } else if left_dist < top_dist && left_dist < bottom_dist && left_dist < right_dist {
        BrickEdge::Left
    } else {
        BrickEdge::Right
    }
}

pub type Bricks = Vec<Vec<Brick>>;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Container {
    bricks: Vec<Vec<Brick>>,
    asset_url: String,
}

impl Container {
    pub fn new(asset_url: String) -> Container {
        let mut bricks: Bricks = Vec::new();
        bricks.resize(BRICK_COLUMN_COUNT, Vec::new());
        for c in 0..bricks.len() {
            bricks[c].resize(BRICK_ROW_COUNT, Brick::new(0.0, 0.0, BrickStatus::Live));
        }
        Container {
            bricks: bricks,
            asset_url: asset_url,
        }
    }

    pub fn get_col_len(&self) -> usize {
        self.bricks.len()
    }

    pub fn get_row_len(&self, col: usize) -> usize {
        self.bricks[col].len()
    }

    pub fn get_status(&self, col: usize, row: usize) -> BrickStatus {
        self.bricks[col][row].get_status()
    }

    pub fn set_x(&mut self, col: usize, row: usize, val: f64) {
        self.bricks[col][row].set_x(val);
    }

    pub fn set_y(&mut self, col: usize, row: usize, val: f64) {
        self.bricks[col][row].set_y(val);
    }

    pub fn get_brick(&self, col: usize, row: usize) -> Brick {
        self.bricks[col][row]
    }

    pub fn set_status(&mut self, col: usize, row: usize, status: BrickStatus) {
        self.bricks[col][row].set_status(status);
    }

    pub fn draw(
        &mut self,
        image_container: &HashMap<String, ImageBitmap>,
        ctx: &web_sys::CanvasRenderingContext2d,
    ) {
        for c in 0..self.bricks.len() {
            for r in 0..self.bricks[c].len() {
                // if self.bricks[c][r].get_status() == BrickStatus::Live {
                self.bricks[c][r].set_x_and_y_from_col_row(c as f64, r as f64);
                let bitmap = image_container.get(&format!("{}-{}", c, r));
                match bitmap {
                    Some(b) => self.bricks[c][r].draw(&b, &ctx),
                    None => continue,
                };
                // }
            }
        }
    }

    pub fn collision_detection(
        &mut self,
        status: &mut GameStatus,
        ball: &mut Ball,
        bitmaps_container: &Rc<RefCell<HashMap<String, ImageBitmap>>>,
    ) {
        for c in 0..self.bricks.len() {
            for r in 0..self.bricks[c].len() {
                let b = self.bricks[c][r];
                if b.get_status() == BrickStatus::Live {
                    let brick_x = b.get_x();
                    let brick_y = b.get_y();
                    let x = ball.get_x();
                    let y = ball.get_y();
                    if x > brick_x
                        && x < brick_x + BRICK_WIDTH
                        && y > brick_y
                        && y < brick_y + BRICK_HEIGHT
                    {
                        // ここに来る場合中心点がいずれかのブロックの中にめり込んでいる場合なので、あとはどの辺が一番近いかチェックする
                        let url = self.asset_url.clone();
                        let bitmap_container = bitmaps_container.clone();
                        spawn_local(async move {
                            let image = get_image(c as u32, r as u32, &url, "26_angel")
                                .await
                                .unwrap();
                            bitmap_container
                                .borrow_mut()
                                .insert(format!("{}-{}", c, r), image);
                        });

                        let left_dist = (x - brick_x).abs();
                        let right_dist = (x - (brick_x + BRICK_WIDTH)).abs();
                        let bottom_dist = (y - brick_y).abs();
                        let top_dist = (y - (brick_y + BRICK_HEIGHT)).abs();
                        match is_nealy_edge(top_dist, bottom_dist, left_dist, right_dist) {
                            BrickEdge::Top => ball.set_dy(-ball.get_dy()),
                            BrickEdge::Bottom => ball.set_dy(-ball.get_dy()),
                            BrickEdge::Left => ball.set_dx(-ball.get_dx()),
                            BrickEdge::Right => ball.set_dx(-ball.get_dx()),
                        }
                        ball.add_speed();
                        let is_break = self.bricks[c][r].update_status();
                        if is_break == BrickStatus::Dead {
                            status.set_score(status.get_score() + 1);
                        }
                        if status.get_score() == BRICK_SUM {
                            let _ = web_sys::window()
                                .unwrap()
                                .alert_with_message("YOU WIN, CONGRATULATIONS!");
                            let _ = web_sys::window().unwrap().location().reload();
                        }
                    }
                }
            }
        }
    }
}

// startをつけると読み込み時に自動で実行される
#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn start(asset_url: String) {
    // 各種エレメントがない/APIが実行できない場合は進めないのでその場で終了
    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("myCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| console::log_1(&JsValue::from_str("CanvasElement is invalid")))
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let width = canvas.width();
    let height = canvas.height();
    let offset_left = canvas.offset_left();
    let bitmap_container: HashMap<String, ImageBitmap> = HashMap::new();
    let bitmap_container = Rc::new(RefCell::new(bitmap_container));

    for i in 0..BRICK_COLUMN_COUNT {
        for j in 0..BRICK_ROW_COUNT {
            let asset_url = asset_url.clone();
            let local_container = bitmap_container.clone();
            spawn_local(async move {
                // moveされたやつの参照を取るのでライフタイムは問題ない
                // asyncの中でborrow_mutを取るとマルチスレッドについて考慮が必要(lockとか)
                // let mut mut_bitmap = local_container.borrow_mut();
                let image = get_image(i as u32, j as u32, &asset_url, "27_devil")
                    .await
                    .unwrap();
                local_container
                    .borrow_mut()
                    .insert(format!("{}-{}", i, j), image);
            });
        }
    }

    let mut bricks = Container::new(asset_url);
    let ball = Ball::new(
        2.0 * SPEED,
        -2.0 * SPEED,
        canvas.width() as f64 / 2.0,
        canvas.height() as f64 - BALL_RADIUS * 2.0,
    );
    let status = GameStatus::new();
    let paddle = Paddle::new((canvas.width() as f64 - PADDLE_WIDTH) / 2.0);

    let f = Rc::new(RefCell::new(None));
    // イベントハンドラ内で変更して、描画処理で使うものについては参照を共有したいのでRcで作る
    let context = Rc::new(context);
    // 変更したい変数は更に追加でCellで作る
    let paddle = Rc::new(RefCell::new(paddle));
    let status = Rc::new(RefCell::new(status));
    let ball = Rc::new(RefCell::new(ball));

    {
        let g = f.clone();
        let context = context.clone();
        let paddle = paddle.clone();
        let status = status.clone();
        let ball = ball.clone();
        let bitmap_container = bitmap_container.clone();

        // クロージャのキャプチャで一回Moveされる
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // 前のフレームの描画を消すために一旦clearする
            context.clear_rect(0.0, 0.0, width as f64, height as f64);
            // ボールが下側に表示されないようにブロックから表示する
            bricks.draw(&mut bitmap_container.borrow_mut(), &context);
            ball.borrow_mut().draw(&context);
            paddle.borrow_mut().draw(&context, height as f64);

            status.borrow().draw_score(&context);
            status.borrow().draw_lives(&context, width as f64);
            // forの中でborrowとかするとスコープ内のborrow_mutとかち合うので外側で取っておく
            let ball_speed = ball.borrow().get_speed();
            for _ in 0..ball_speed {
                bricks.collision_detection(
                    &mut status.borrow_mut(),
                    &mut ball.borrow_mut(),
                    &bitmap_container,
                );
                update(
                    &mut ball.borrow_mut(),
                    &mut paddle.borrow_mut(),
                    &mut status.borrow_mut(),
                    canvas.width() as f64,
                    canvas.height() as f64,
                );
            }

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    // キーボードのキー押した時のイベント
    {
        let paddle = paddle.clone();

        let keydown_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Right" || e.key() == "ArrowRight" {
                paddle.borrow_mut().set_right_pressed(true);
            } else if e.key() == "Left" || e.key() == "ArrowLeft" {
                paddle.borrow_mut().set_left_pressed(true);
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

        document.set_onkeydown(Some(keydown_handler.as_ref().unchecked_ref()));
        keydown_handler.forget();
    }

    // キーボードのキー離したときのイベント
    {
        let paddle = paddle.clone();

        let keyup_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Right" || e.key() == "ArrowRight" {
                paddle.borrow_mut().set_right_pressed(false);
            } else if e.key() == "Left" || e.key() == "ArrowLeft" {
                paddle.borrow_mut().set_left_pressed(false);
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        document.set_onkeyup(Some(keyup_handler.as_ref().unchecked_ref()));
        keyup_handler.forget();
    }

    // マウスイベント
    {
        let paddle = paddle.clone();
        let status = status.clone();
        let ball = ball.clone();

        let mousemove_handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let relative_x = e.client_x() - offset_left;
            if relative_x > 0 && relative_x < (width as i32) {
                paddle
                    .borrow_mut()
                    .set_x(relative_x as f64 - PADDLE_WIDTH / 2.0);
                if status.borrow().get_status() != Status::Start {
                    ball.borrow_mut().set_x(relative_x as f64);
                }
            }
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        document.set_onmousemove(Some(mousemove_handler.as_ref().unchecked_ref()));
        mousemove_handler.forget();
    }

    // クリックのスタートイベント
    {
        let status = status.clone();
        let click_handler = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            status.borrow_mut().set_status(Status::Start);
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        document.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
        click_handler.forget();
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn update(
    ball: &mut Ball,
    paddle: &mut Paddle,
    status: &mut GameStatus,
    width: f64,
    height: f64,
) {
    if status.get_status() != Status::Start {
        return;
    }
    if ball.get_x() + ball.get_dx() > width as f64 - BALL_RADIUS
        || ball.get_x() + ball.get_dx() < BALL_RADIUS
    {
        // 壁にあたった場合その１
        ball.set_dx(-ball.get_dx());
    }

    if ball.get_y() + ball.get_dy() < BALL_RADIUS {
        // 壁にあたった場合その2
        ball.set_dy(-ball.get_dy());
    } else if ball.get_y() + ball.get_dy() > height as f64 - BALL_RADIUS {
        if ball.get_x() > paddle.get_x() && ball.get_x() < paddle.get_x() + PADDLE_WIDTH {
            // パドルにボールが当たった場合
            let dist = (ball.get_x() + BALL_RADIUS) - (paddle.get_x() + PADDLE_WIDTH / 2.0);
            let radian = (90.0 - (dist / (PADDLE_WIDTH / 2.0)) * 80.0).to_radians();
            let speed = (ball.get_dx().powf(2.0) + ball.get_dy().powf(2.0)).sqrt();
            ball.set_dx(radian.cos() * speed);
            ball.set_dy(-radian.sin() * speed);
            ball.add_speed();
        } else {
            ball.set_dy(-ball.get_dy());
            // ここのelse節は下に突き抜けた場合
            status.set_status(Status::Stop);
            status.set_lives(status.get_lives() - 1);
            if status.get_lives() == 0 {
                let _ = web_sys::window().unwrap().alert_with_message("GAME OVER");
                let _ = web_sys::window().unwrap().location().reload();
            } else {
                ball.set_x(width as f64 / 2.0);
                ball.set_y(height as f64 - BALL_RADIUS * 2.0);
                ball.set_dx(2.0 * SPEED);
                ball.set_dy(-2.0 * SPEED);
                ball.init_speed();
                paddle.set_x((width as f64 - PADDLE_WIDTH) / 2.0);
            }
        }
    }
    if paddle.get_right_pressed() && paddle.get_x() < width as f64 - PADDLE_WIDTH {
        paddle.set_x(paddle.get_x() + 7.0);
    } else if paddle.get_left_pressed() && paddle.get_x() > 0.0 {
        paddle.set_x(paddle.get_x() - 7.0);
    }
    ball.set_x(ball.get_x() + ball.get_dx());
    ball.set_y(ball.get_y() + ball.get_dy());
}
