mod com_player;
mod game;
use com_player::*;
use game::*;
use std::time::{Duration, Instant};

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

fn main() {
    let now = Instant::now();
    let mut game: Game = Game::new();
    let com_first = true;
    println!("{}", game);
    let com = ComPlayer::new();
    while game.state != GameState::FINISHED {
        match game.state {
            GameState::WhiteTurn => {
                println!("\x1B[34m\x1B[47m{}\x1B[0m", "white's trun");
                // put_on_board(&mut game, Peice::WHITE);
                let da_way = com.find_the_best_way(&mut game, Peice::WHITE, 6);
                game.put_peice(Peice::WHITE, da_way.1, da_way.0);
            }
            GameState::BlackTurn => {
                println!("\x1B[34m\x1B[40m{}\x1B[0m", "black's trun");
                let da_way = com.find_the_best_way(&mut game, Peice::BLACK, 6);
                game.put_peice(Peice::BLACK, da_way.1, da_way.0);
                // put_on_board(&mut game, Peice::BLACK);
            }
            GameState::FINISHED => println!("FINISHED!!!"),
        }
        println!("{}", game);
    }
    let winner = game.compute_winner();
    match winner {
        Peice::BLACK => println!("Black WIN!!"),
        Peice::WHITE => println!("WHITE WIN!!"),
        Peice::EMPTY => println!("DRAW!"),
    }
    println!("{}", game);
    println!(
        "game spend {:?}",
        Instant::now().checked_duration_since(now)
    );
}
