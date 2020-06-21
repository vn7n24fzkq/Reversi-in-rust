use crate::game::*;
use std::cmp;

pub struct ComPlayer;

static lazy_weight: [[i8; 8]; 8] = [
    [100, -20, 10, 5, 5, 10, -20, 100],
    [-20, -50, -2, -2, -2, -2, -50, -20],
    [10, -2, -1, -1, -1, -1, -2, 10],
    [5, -2, -1, -1, -1, -1, -2, 5],
    [5, -2, -1, -1, -1, -1, -2, 5],
    [10, -2, -1, -1, -1, -1, -2, 10],
    [-20, -50, -2, -2, -2, -2, -50, -20],
    [100, -20, 10, 5, 5, 10, -20, 100],
];
// static lazy_weight: [[i8; 8]; 8] = [
//     [100, 0, 10, 5, 5, 10, 0, 100],
//     [0, 0, 0, 0, 0, 0, 0, 0],
//     [10, 0, 0, 0, 0, 0, 0, 10],
//     [5, 0, 0, 0, 0, 0, 0, 5],
//     [5, 0, 0, 0, 0, 0, 0, 5],
//     [10, 0, 0, 0, 0, 0, 0, 10],
//     [0, 0, 0, 0, 0, 0, 0, 0],
//     [100, 0, 10, 5, 5, 10, 0, 100],
// ];

// static lazy_weight: [[i8; 8]; 8] = [
//     [100, -30, 15, 5, 5, 15, -30, 100],
//     [-30, -80, -2, -2, -2, -2, -80, -30],
//     [15, -2, -1, -1, -1, -1, -2, 15],
//     [5, -2, -1, 0, 0, -1, -2, 5],
//     [5, -2, -1, 0, 0, -1, -2, 5],
//     [15, -2, -1, -1, -1, -1, -2, 15],
//     [-30, -80, -2, -2, -2, -2, -80, -30],
//     [100, -30, 15, 5, 5, 15, -30, 100],
// ];

impl ComPlayer {
    pub fn new() -> ComPlayer {
        ComPlayer {}
    }
    pub fn find_the_best_way(
        &self,
        game: &mut Game,
        peice: Peice,
        max_level: i32,
    ) -> (usize, usize, [[i8; 8]; 8]) {
        let da_way = Game::the_point_if_you_can_place_and_effect(game.board, peice.clone());
        //black >0 white <0
        let mut way = da_way[0];

        match peice {
            Peice::BLACK => {
                let mut cost: i64 = i64::MAX;
                for w in da_way {
                    let c = ComPlayer::find_da_way(
                        game,
                        w.1,
                        w.0,
                        i64::MIN,
                        i64::MAX,
                        Peice::BLACK,
                        1,
                        max_level,
                    );
                    println!("{},{} - {}", w.1, w.0, c);
                    if c > cost {
                        cost = c;
                        way = w;
                    }
                }
                println!("{}", cost);
            }
            Peice::WHITE => {
                let mut cost: i64 = i64::MAX;
                for w in da_way {
                    let c = ComPlayer::find_da_way(
                        game,
                        w.1,
                        w.0,
                        i64::MAX,
                        i64::MIN,
                        Peice::WHITE,
                        -1,
                        max_level,
                    );
                    println!("{},{} - {}", w.1, w.0, c);
                    if c < cost {
                        cost = c;
                        way = w;
                    }
                }
                println!("{}", cost);
            }
            _ => (),
        };
        return way;
    }

    fn find_da_way(
        g: &mut Game,
        x: usize,
        y: usize,
        mut alpha: i64,
        mut beta: i64,
        peice: Peice,
        target_peice: i8,
        max_level: i32,
    ) -> i64 {
        let mut game = g.clone();
        game.put_peice(peice, x, y);
        if max_level <= 0 {
            // println!("{}", game);
            // println!("{}", ComPlayer::compute_score(&game.board));
            return ComPlayer::compute_score(&game.board);
        }
        match game.state {
            GameState::FINISHED => match game.compute_winner() {
                Peice::BLACK => return i64::MAX,
                Peice::WHITE => return i64::MIN,
                Peice::EMPTY => return 0,
            },
            GameState::BlackTurn => {
                let mut cost = i64::MIN;
                if target_peice == -1 {
                    cost = i64::MAX;
                }
                let _da_way = Game::the_point_if_you_can_place_and_effect(game.board, Peice::BLACK);
                for way in _da_way {
                    let cc = ComPlayer::find_da_way(
                        &mut game,
                        way.1,
                        way.0,
                        alpha,
                        beta,
                        Peice::BLACK,
                        target_peice,
                        max_level - 1,
                    );
                    if target_peice == 1 {
                        println!("{}\t{}\t{}\t{}", cost, cc, alpha, beta);
                        cost = cmp::max(cost, cc);
                        alpha = cmp::max(alpha, cost);
                        if alpha >= beta {
                            break;
                        }
                    } else {
                        cost = cmp::max(cost, cc);
                        beta = cmp::max(beta, cost);
                        if beta >= alpha {
                            break;
                        }
                    }
                }
                return cost;
            }
            GameState::WhiteTurn => {
                let mut cost = i64::MIN;
                if target_peice == -1 {
                    cost = i64::MAX;
                }
                let _da_way = Game::the_point_if_you_can_place_and_effect(game.board, Peice::WHITE);
                for way in _da_way {
                    let cc = ComPlayer::find_da_way(
                        &mut game,
                        way.1,
                        way.0,
                        alpha,
                        beta,
                        Peice::WHITE,
                        target_peice,
                        max_level - 1,
                    );
                    if target_peice == 1 {
                        println!("{}\t{}\t{}\t{}", cost, cc, alpha, beta);
                        cost = cmp::max(cost, cc);
                        beta = cmp::max(beta, cost);
                        if beta <= alpha {
                            break;
                        }
                    } else {
                        cost = cmp::min(cost, cc);
                        alpha = cmp::min(alpha, cost);
                        if alpha <= beta {
                            break;
                        }
                    }
                }
                return cost;
            }
        };
    }

    fn compute_score(board: &[[i8; 8]; 8]) -> i64 {
        let mut final_weight: i64 = 0;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                final_weight += lazy_weight[i][j] as i64 * board[i][j] as i64;
            }
        }
        return final_weight;
    }
}
