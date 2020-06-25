mod com_player;
mod game;
use com_player::*;
use game::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
struct Othello {
    game: Game,
    com: ComPlayer,
    context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Othello {
    pub fn finish(&self) -> bool {
        self.game.state == GameState::FINISHED
    }
    pub fn new() -> Othello {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("othello-canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Othello {
            game: Game::new(),
            com: ComPlayer::new(),
            context: context,
        }
    }
    pub fn start(&mut self, white_is_com: bool, black_is_com: bool) {
        match self.game.state {
            GameState::WhiteTurn => {
                if white_is_com {
                    let da_way = self.com.find_the_best_way(&mut self.game, Peice::WHITE, 3);
                    self.game.put_peice(Peice::WHITE, da_way.1, da_way.0);
                } else {
                    // put_on_board(&mut game, Peice::WHITE);
                }
            }
            GameState::BlackTurn => {
                if black_is_com {
                    let da_way = self.com.find_the_best_way(&mut self.game, Peice::BLACK, 3);
                    self.game.put_peice(Peice::BLACK, da_way.1, da_way.0);
                } else {
                    // put_on_board(&mut game, Peice::BLACK);
                }
            }
            _ => (),
        }
    }

    pub fn get_winner(&self) -> i32 {
        let winner = self.game.compute_winner();
        match winner {
            Peice::BLACK => 1,
            Peice::WHITE => -1,
            Peice::EMPTY => 0,
        }
    }

    pub fn draw(&self) {
        let ctx = &self.context;
        let game = &self.game;
        //draw background
        // ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("#009933"));
        ctx.fill_rect(0.0, 0.0, 600.0, 600.0);
        // //draw table
        ctx.set_fill_style(&JsValue::from_str("#000000"));
        ctx.begin_path();
        ctx.set_line_width(3.0);
        for i in 0..9 {
            ctx.move_to(i as f64 * 75.0, 0.0);
            ctx.line_to(i as f64 * 75.0, 600.0);
            ctx.move_to(0.0, i as f64 * 75.0);
            ctx.line_to(600.0, i as f64 * 75.0);
        }

        ctx.stroke();
        ctx.set_line_width(0.0);
        for i in 0..8 {
            for j in 0..8 {
                ctx.begin_path();
                if game.board[i][j] == -1 {
                    ctx.set_fill_style(&JsValue::from_str("#000000"));
                    ctx.arc(
                        i as f64 * 75.0 + 37.0,
                        j as f64 * 75.0 + 37.5,
                        30.0,
                        0.0,
                        7.18,
                    );
                    ctx.fill();
                } else if game.board[i][j] == 1 {
                    ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
                    ctx.arc(
                        i as f64 * 75.0 + 37.5,
                        j as f64 * 75.0 + 37.5,
                        30.0,
                        0.0,
                        7.18,
                    );
                    ctx.fill();
                }
                ctx.stroke();
            }
        }
    }
}
fn put_on_board(game: &mut Game, peice: game::Peice) {
    let mut input = String::new();
    println!("Please enter place like - a2");
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let input = input.as_bytes();
    if input.len() != 3 {
        eprintln!("\x1B[37m\x1B[41m{}\x1B[0m", "Wrong input!");
        return;
    }
    let result = game.put_peice_with_humanread(peice, input[0] as char, input[1] as i8 - 48);
    // println!("{}[2J", 27 as char);
    if result.is_err() {
        eprintln!("\x1B[37m\x1B[41m{}\x1B[0m", result.err().unwrap());
    }
}
