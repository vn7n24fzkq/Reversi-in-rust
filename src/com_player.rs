use crate::game::*;
use std::cmp;
use std::time::{Duration, Instant};

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
static middle_weight: [[i8; 8]; 8] = [
    [100, -25, 10, 5, 5, 10, -25, 100],
    [-25, -50, -2, -2, -2, -2, -50, -25],
    [10, -2, -1, -1, -1, -1, -2, 10],
    [5, -2, -1, -1, -1, -1, -2, 5],
    [5, -2, -1, -1, -1, -1, -2, 5],
    [10, -2, -1, -1, -1, -1, -2, 10],
    [-25, -50, -2, -2, -2, -2, -50, -25],
    [100, -25, 10, 5, 5, 10, -25, 100],
];
static end_weight: [[i8; 8]; 8] = [
    [127, 20, 10, 5, 5, 10, 20, 127],
    [20, -50, -2, -2, -2, -2, -50, 20],
    [10, -2, -1, -1, -1, -1, -2, 10],
    [5, -2, -1, -1, -1, -1, -2, 5],
    [5, -2, -1, -1, -1, -1, -2, 5],
    [10, -2, -1, -1, -1, -1, -2, 10],
    [20, -50, -2, -2, -2, -2, -50, 20],
    [127, 20, 10, 5, 5, 10, 20, 127],
];

// static lazy_weight: [[i8; 8]; 8] = [
//     [100, -20, 10, 5, 5, 10, -20, 100],
//     [-20, -50, -2, -2, -2, -2, -50, -20],
//     [10, -2, -1, -1, -1, -1, -2, 10],
//     [5, -2, -1, -1, -1, -1, -2, 5],
//     [5, -2, -1, -1, -1, -1, -2, 5],
//     [10, -2, -1, -1, -1, -1, -2, 10],
//     [-20, -50, -2, -2, -2, -2, -50, -20],
//     [100, -20, 10, 5, 5, 10, -20, 100],
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
        let now = Instant::now();
        let da_way = Game::the_point_if_you_can_place_and_effect(game.board, peice.clone());
        //black >0 white <0
        let mut way = da_way[0];
        let p = peice.to_i8();
        let mut cost: i64 = i64::MAX;
        for w in da_way {
            let mut state = 1;
            if game.state == GameState::WhiteTurn {
                state = -1;
            }
            let c = ComPlayer::do_u_know_da_way(
                game,
                w.1,
                w.0,
                i64::MIN + 1,
                i64::MAX - 1,
                state,
                state,
                max_level,
                true,
                now,
                20.0,
            );
            println!("{},{} - {}", w.1, w.0, c);
            if c < cost {
                cost = c;
                way = w;
            }
        }
        println!("{}", cost);
        println!("spend {:?}", Instant::now().checked_duration_since(now));
        return way;
    }

    fn do_u_know_da_way(
        g: &mut Game,
        x: usize,
        y: usize,
        mut alpha: i64,
        mut beta: i64,
        peice: i8,
        target_peice: i8,
        max_level: i32,
        max: bool,
        time_instant: Instant,
        limit_sec: f64,
    ) -> i64 {
        let mut game = g.clone();
        game.put_peice(Peice::from_i8(peice), x, y);

        let mut state = 1;
        if (game.state == GameState::WhiteTurn) {
            state = -1;
        }
        if Instant::now()
            .checked_duration_since(time_instant)
            .unwrap()
            .as_secs_f64()
            > limit_sec
        {
            println!("Spend too much time!!");
            match game.compute_winner().to_i8() {
                target_peice => return i64::MAX - 1,
                _ => return i64::MIN + 1,
            }
        }
        if game.state == GameState::FINISHED || game.turn_count >= 54 {
            match game.compute_winner().to_i8() {
                target_peice => return i64::MAX - 1,
                _ => return i64::MIN + 1,
            }
        }

        if max_level <= 0 {
            let cc = ComPlayer::compute_score(&game) as i64 * state as i64;
            return cc;
        }
        let mut cost = i64::MIN + 1;
        for way in Game::the_point_if_you_can_place_and_effect(game.board, Peice::from_i8(state)) {
            cost = cmp::max(
                cost,
                -ComPlayer::do_u_know_da_way(
                    &mut game,
                    way.1,
                    way.0,
                    -beta,
                    -alpha,
                    state,
                    target_peice,
                    max_level - 1,
                    target_peice == state,
                    time_instant,
                    limit_sec,
                ),
            );
            alpha = cmp::max(cost, alpha);
            if alpha >= beta {
                break;
            }
        }
        return cost;
    }

    fn find_da_way(
        g: &mut Game,
        x: usize,
        y: usize,
        mut alpha: i64,
        mut beta: i64,
        peice: i8,
        target_peice: i8,
        max_level: i32,
        max: bool,
    ) -> i64 {
        let mut game = g.clone();
        game.put_peice(Peice::from_i8(peice), x, y);

        //
        if game.state == GameState::FINISHED || game.turn_count >= 54 {
            match game.compute_winner().to_i8() {
                target_peice => return i64::MAX,
                _ => return i64::MIN,
            }
        }

        if max_level <= 0 {
            // println!("{}", game);
            // println!("{}", ComPlayer::compute_score(&game.board));
            let cc = ComPlayer::compute_score(&game) as i64 * target_peice as i64;
            // if cc == 54 {
            //     println!("{}", game);
            // }
            return cc;

            // + Game::the_point_if_you_can_place_and_effect(
            //     game.board,
            //     Peice::from_i8(target_peice),
            // )
            // .len() as i64
            //     * 10;
        }
        let mut state = 1;
        if (game.state == GameState::WhiteTurn) {
            state = -1;
        }
        if max {
            // println!("{} max", max_level);
            let mut cost = i64::MIN;
            for way in
                Game::the_point_if_you_can_place_and_effect(game.board, Peice::from_i8(state))
            {
                cost = cmp::max(
                    cost,
                    ComPlayer::find_da_way(
                        &mut game,
                        way.1,
                        way.0,
                        alpha,
                        beta,
                        state,
                        target_peice,
                        max_level - 1,
                        target_peice == state,
                    ),
                );
                alpha = cmp::max(cost, alpha);
                if alpha >= beta {
                    break;
                }
            }
            return cost;
        } else {
            // println!("{} min", max_level);
            let mut cost = i64::MAX;
            for way in
                Game::the_point_if_you_can_place_and_effect(game.board, Peice::from_i8(state))
            {
                cost = cmp::min(
                    cost,
                    ComPlayer::find_da_way(
                        &mut game,
                        way.1,
                        way.0,
                        alpha,
                        beta,
                        state,
                        target_peice,
                        max_level - 1,
                        target_peice == state,
                    ),
                );
                beta = cmp::min(cost, beta);
                if beta <= alpha {
                    break;
                }
            }
            return cost;
        }
    }

    fn compute_score(game: &Game) -> i64 {
        let mut final_weight: i64 = 0;
        let mut weight = lazy_weight;
        if game.turn_count > 20 && game.turn_count < 50 {
            weight = middle_weight;
        } else if game.turn_count >= 50 {
            weight = end_weight;
        }

        let board = game.board;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                final_weight += (weight[i][j] as i64 * board[i][j] as i64);
            }
        }
        return final_weight;
    }
}
