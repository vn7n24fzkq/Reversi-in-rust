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
        game.force_put_peice(Peice::BLACK, 'd', 5);
        game.force_put_peice(Peice::WHITE, 'd', 4);
        game.force_put_peice(Peice::BLACK, 'e', 4);
        game.force_put_peice(Peice::WHITE, 'e', 5);
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

    fn force_put_peice(&mut self, peice: Peice, row_index: char, col_index: i8) {
        let col_index_on_board = (col_index - 1) as usize;
        let row_index_on_board = self.row_to_index(&row_index) as usize;
        self.board[col_index_on_board][row_index_on_board] = peice as i8;
        self.turn_count += 1;
    }
    fn put_peice(
        &mut self,
        peice: Peice,
        row_index: char,
        col_index: i8,
    ) -> std::result::Result<(), &str> {
        // board index start ad 1
        let col_index_on_board = (col_index - 1) as usize;
        let row_index_on_board = self.row_to_index(&row_index) as usize;
        if col_index_on_board >= 8 || row_index_on_board >= 8 {
            return Err("Wrong cocoordinate!!");
        }
        if self.board[col_index_on_board][row_index_on_board] != Peice::EMPTY as i8 {
            return Err("Coordinate is not empty!!");
        } else {
            if !(peice == Peice::BLACK && self.state == GameState::BlackTurn)
                && !(peice == Peice::WHITE && self.state == GameState::WhiteTurn)
            {
                return Err("It is not your turn now!!");
            }
            let effect = Game::the_point_if_you_can_place_and_effect(self.board, peice);
            let mut find: (usize, usize, [[i8; 8]; 8]) = (0, 0, [[0; 8]; 8]);
            let mut findout = false;

            for _point in &effect {
                if _point.0 == col_index_on_board && _point.1 == row_index_on_board {
                    findout = true;
                    find = *_point;
                }
            }

            if effect.len() == 0 {
                return Err("You don't have any cocoordinate allowed to place!!");
            }

            if !findout {
                return Err("You can't place here!!");
            }

            if findout || self.turn_count < 4 {
                self.turn_count += 1;
                self.board = find.2;
                if self.state == GameState::BlackTurn {
                    self.state = GameState::WhiteTurn;
                } else if self.state == GameState::WhiteTurn {
                    self.state = GameState::BlackTurn;
                }
                if self.turn_count == 64 {
                    self.state = GameState::FINISHED;
                }
            }
            return Ok(());
        }
    }

    fn the_point_if_you_can_place_and_effect(
        board: [[i8; 8]; 8],
        peice: Peice,
    ) -> Vec<(usize, usize, [[i8; 8]; 8])> {
        let mut result = vec![];
        let peice_number = peice as i8;

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let mut result_board = board;
                let mut effect_points = Game::_find_board(&board, i, j, &peice_number);
                if effect_points.len() == 0 {
                    continue;
                }
                for v in effect_points {
                    result_board[v.0][v.1] *= -1;
                }
                result_board[i][j] = peice_number;
                result.push((i, j, result_board));
            }
        }
        result
    }

    fn _find_board(
        _board: &[[i8; 8]; 8],
        x: usize,
        y: usize,
        peice_number: &i8,
    ) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let mut tmp_result = vec![];
        //right
        let mut i = y + 1;
        while i < _board[x].len() {
            if _board[x][i] == Peice::EMPTY as i8 {
                tmp_result.clear();
                break;
            } else if _board[x][i] == *peice_number {
                result.append(&mut tmp_result);
                break;
            } else {
                tmp_result.push((x, i));
            }
            i += 1;
        }
        //left
        let mut i = y;
        while i > 0 {
            i -= 1;
            if _board[x][i] == Peice::EMPTY as i8 {
                tmp_result.clear();
                break;
            } else if _board[x][i] == *peice_number {
                result.append(&mut tmp_result);
                break;
            } else {
                tmp_result.push((x, i));
            }
        }
        //up
        let mut i = x;
        while i > 0 {
            i -= 1;
            if _board[i][y] == Peice::EMPTY as i8 {
                tmp_result.clear();
                break;
            } else if _board[i][y] == *peice_number {
                result.append(&mut tmp_result);
                break;
            } else {
                tmp_result.push((i, y));
            }
        }
        //down
        let mut i = x + 1;
        while i < _board[x].len() {
            if _board[i][y] == Peice::EMPTY as i8 {
                tmp_result.clear();
                break;
            } else if _board[i][y] == *peice_number {
                result.append(&mut tmp_result);
                break;
            } else {
                tmp_result.push((i, y));
            }
            i += 1;
        }
        //right-up
        let mut i = y + 1;
        let mut j = x;
        while i < _board[x].len() && j > 0 {
            j -= 1;
            if _board[j][i] == Peice::EMPTY as i8 {
                tmp_result.clear();
                break;
            } else if _board[j][i] == *peice_number {
                result.append(&mut tmp_result);
                break;
            } else {
                tmp_result.push((j, i));
            }
            i += 1;
        }
        //right-down
        let mut i = y + 1;
        let mut j = x + 1;
        while i < _board[x].len() && j < _board.len() {
            if _board[j][i] == Peice::EMPTY as i8 {
                tmp_result.clear();
                break;
            } else if _board[j][i] == *peice_number {
                result.append(&mut tmp_result);
                break;
            } else {
                tmp_result.push((j, i));
            }
            i += 1;
            j += 1;
        }
        //left-up
        let mut i = y;
        let mut j = x;
        while i > 0 && j > 0 {
            j -= 1;
            i -= 1;
            if _board[j][i] == Peice::EMPTY as i8 {
                tmp_result.clear();
                break;
            } else if _board[j][i] == *peice_number {
                result.append(&mut tmp_result);
                break;
            } else {
                tmp_result.push((j, i));
            }
        }
        //left-down
        let mut i = y;
        let mut j = x + 1;
        while i > 0 && j < _board[x].len() {
            i -= 1;
            if _board[j][i] == Peice::EMPTY as i8 {
                tmp_result.clear();
                break;
            } else if _board[j][i] == *peice_number {
                result.append(&mut tmp_result);
                break;
            } else {
                tmp_result.push((j, i));
            }
            j += 1;
        }
        result
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
            GameState::WhiteTurn => {
                println!("\x1B[34m\x1B[47m{}\x1B[0m", "white's trun");
                put_on_board(&mut game, Peice::WHITE);
            }
            GameState::BlackTurn => {
                println!("\x1B[34m\x1B[40m{}\x1B[0m", "black's trun");
                put_on_board(&mut game, Peice::BLACK);
            }
            GameState::FINISHED => println!("FINISHED!!!"),
        }
        println!("{}", game);
    }
}
