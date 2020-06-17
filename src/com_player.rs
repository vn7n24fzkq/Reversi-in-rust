use crate::game::*;

extern crate time;
use time::PreciseTime;

pub struct ComPlayer;

impl ComPlayer {
    pub fn new() -> ComPlayer {
        ComPlayer {}
    }
    pub fn find_the_best_way(&self, game: &mut Game, peice: Peice) -> (usize, usize, [[i8; 8]; 8]) {
        let da_way = Game::the_point_if_you_can_place_and_effect(game.board, peice.clone());
        let mut cost = 0;
        //black >0 white <0
        let mut way = da_way[0];
        // return way;
        for w in da_way {
            let start = PreciseTime::now();

            match peice {
                Peice::BLACK => {
                    let c = ComPlayer::find_da_way(game, w.1, w.0, Peice::BLACK, 0);
                    if c > cost {
                        cost = c;
                        way = w;
                    }
                }
                Peice::WHITE => {
                    let c = ComPlayer::find_da_way(game, w.1, w.0, Peice::WHITE, 0);
                    if c < cost {
                        cost = c;
                        way = w;
                    }
                }
                _ => (),
            };
            let end = PreciseTime::now();
            println!("{}", start.to(end));
            println!("------------");
        }
        return way;
    }

    fn find_da_way(g: &mut Game, x: usize, y: usize, peice: Peice, mut level: i32) -> i32 {
        level += 1;
        let mut game = g.clone();
        if level >= 7 || game.state == GameState::FINISHED {
            match game.compute_winner() {
                Peice::BLACK => return 1,
                Peice::WHITE => return -1,
                Peice::EMPTY => return 0,
            };
        }
        game.put_peice(peice, x, y);
        match game.state {
            GameState::FINISHED => match game.compute_winner() {
                Peice::BLACK => return 1,
                Peice::WHITE => return -1,
                Peice::EMPTY => return 0,
            },
            GameState::BlackTurn => {
                let _da_way = Game::the_point_if_you_can_place_and_effect(game.board, Peice::BLACK);
                // level += (_da_way.len() as i32);
                let mut cost = 0;
                for way in _da_way {
                    let cc = ComPlayer::find_da_way(&mut game, way.1, way.0, Peice::BLACK, level);
                    cost += cc;
                }
                return cost;
            }
            GameState::WhiteTurn => {
                let _da_way = Game::the_point_if_you_can_place_and_effect(game.board, Peice::WHITE);
                let mut cost = 0;
                // level += (_da_way.len() as i32);
                for way in _da_way {
                    let cc = ComPlayer::find_da_way(&mut game, way.1, way.0, Peice::WHITE, level);
                    cost += cc;
                }
                return cost;
            }
        };
    }
}
