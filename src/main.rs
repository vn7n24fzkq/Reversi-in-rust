use std::fmt;
use std::io::{self, Read};

#[derive(PartialEq)]
enum Peice {
    EMPTY = 0,
    WHITE = -1,
    BLACK = 1,
}

impl Peice {
    fn from_i8(value: i8) -> Peice {
        match value {
            1 => Peice::BLACK,
            -1 => Peice::WHITE,
            0 => Peice::EMPTY,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(PartialEq)]
enum GameState {
    FINISHED,
    WhiteTurn,
    BlackTurn,
}

struct Game {
    board: [[i8; 8]; 8],
    turn_count: i8,
    state: GameState,
}

impl Game {
    fn new() -> Game {
        let mut game = Game {
            board: [[0; 8]; 8],
            turn_count: 0,
            state: GameState::BlackTurn,
        };
        game.put_peice(Peice::BLACK, 'd', 5);
        game.put_peice(Peice::WHITE, 'd', 4);
        game.put_peice(Peice::BLACK, 'e', 4);
        game.put_peice(Peice::WHITE, 'e', 5);
        game
    }

    fn peice_can_place_points(
        &mut self,
        peice_type: i8,
        row_index: char,
        col_index: i8,
    ) -> (char, i8) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {}
        }
        ('a', 3)
    }

    fn row_to_index(&self, row: &char) -> i8 {
        *row as i8 - 97
    }

    fn row_index_to_char(&self, row_index: u8) -> char {
        (row_index + 98) as char
    }

    fn put_peice(
        &mut self,
        peice: Peice,
        row_index: char,
        col_index: i8,
    ) -> std::result::Result<(), &str> {
        if peice == Peice::BLACK && self.state == GameState::BlackTurn {
            self.state = GameState::WhiteTurn;
        } else if peice == Peice::WHITE && self.state == GameState::WhiteTurn {
            self.state = GameState::BlackTurn;
        } else {
            return Err("It is not your turn now !!");
        }

        // board index start ad 1
        let col_index_on_board = (col_index - 1) as usize;
        let row_index_on_board = self.row_to_index(&row_index) as usize;
        if col_index_on_board >= 8 || row_index_on_board >= 8 {
            return Err("Wrong cocoordinate !!");
        }
        if self.board[col_index_on_board][row_index_on_board] != Peice::EMPTY as i8 {
            return Err("Coordinate is not empty!!");
        } else {
            self.turn_count += 1;
            self.board[col_index_on_board][row_index_on_board] = peice as i8;
            if self.turn_count == 64 {
                self.state = GameState::FINISHED;
            }
            return Ok(());
        }
    }

    fn the_point_you_can_place_and_result(&self, peice_type: i8) -> Vec<(i8, i8, [[i8; 8]; 8])> {
        let result = vec![];
        //up
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] != 0 {
                    //up
                    for k in i - 1..0 {}
                    //down
                    //left
                    //right
                }
            }
        }
        result
    }

    fn is_finished(&self) -> bool {
        self.turn_count >= 64
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "\x1B[30m\x1B[42m{}\x1B[0m", "   a b c d e f g h \n");
        for i in 0..self.board.len() {
            write!(f, "\x1B[30m\x1B[42m{} {} \x1B[0m", "", i + 1);
            for row in &self.board[i] {
                let peice = Peice::from_i8(*row);
                match peice {
                    Peice::BLACK => write!(f, "\x1B[30m\x1B[42m{} \x1B[0m", "●"),
                    Peice::WHITE => write!(f, "\x1B[37m\x1B[42m{} \x1B[0m", "●"),
                    _ => write!(f, "\x1B[42m{} \x1B[0m", "-"),
                };
            }
            write!(f, "{}", "\n");
        }
        std::result::Result::Ok(())
    }
}

fn put_on_board(game: &mut Game, peice: Peice) {
    let mut input = String::new();
    println!("Please enter place like - a2");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let input = input.as_bytes();
    let mut result = game.put_peice(peice, input[0] as char, input[1] as i8 - 48);
    println!("{}[2J", 27 as char);
    if result.is_err() {
        eprintln!("\x1B[37m\x1B[41m{}\x1B[0m", result.err().unwrap());
    }
}

fn main() {
    let mut game: Game = Game::new();
    let com_first = true;
    println!("{}", game);
    while game.state != GameState::FINISHED {
        match game.state {
            GameState::WhiteTurn => put_on_board(&mut game, Peice::WHITE),
            GameState::BlackTurn => put_on_board(&mut game, Peice::BLACK),
            GameState::FINISHED => println!("FINISHED!!!"),
        }
        println!("{}", game);
    }
}
