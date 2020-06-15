use std::fmt;

#[derive(PartialEq)]
pub enum Peice {
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

impl Clone for Peice {
    fn clone(&self) -> Peice {
        match self {
            Peice::BLACK => Peice::BLACK,
            Peice::WHITE => Peice::WHITE,
            Peice::EMPTY => Peice::EMPTY,
        }
    }
}

#[derive(PartialEq)]
pub enum GameState {
    FINISHED,
    WhiteTurn,
    BlackTurn,
}

pub struct Game {
    pub board: [[i8; 8]; 8],
    pub turn_count: i8,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Game {
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

    pub fn comput_winner(&self) -> Peice {
        let mut black = 0;
        let mut white = 0;
        for i in self.board.iter() {
            for j in i.iter() {
                let peice = Peice::from_i8(*j);
                match peice {
                    Peice::BLACK => black += 1,
                    Peice::WHITE => white += 1,
                    _ => (),
                };
            }
        }
        if black > white {
            return Peice::BLACK;
        } else if white > black {
            return Peice::WHITE;
        } else {
            return Peice::EMPTY;
        }
    }

    pub fn row_to_index(&self, row: &char) -> i8 {
        *row as i8 - 97
    }

    pub fn row_index_to_char(&self, row_index: u8) -> char {
        (row_index + 98) as char
    }

    pub fn force_put_peice(&mut self, peice: Peice, row_index: char, col_index: i8) {
        let col_index_on_board = (col_index - 1) as usize;
        let row_index_on_board = self.row_to_index(&row_index) as usize;
        self.board[col_index_on_board][row_index_on_board] = peice as i8;
        self.turn_count += 1;
    }

    pub fn put_peice_with_humanread(
        &mut self,
        peice: Peice,
        row_index: char,
        col_index: i8,
    ) -> std::result::Result<(), &str> {
        let col_index_on_board = (col_index - 1) as usize;
        let row_index_on_board = self.row_to_index(&row_index) as usize;
        self.put_peice(peice, row_index_on_board, col_index_on_board)
    }

    pub fn put_peice(
        &mut self,
        peice: Peice,
        row_index_on_board: usize,
        col_index_on_board: usize,
    ) -> std::result::Result<(), &str> {
        // board index start ad 1
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
            let effect = Game::the_point_if_you_can_place_and_effect(self.board, peice.clone());
            let mut find: (usize, usize, [[i8; 8]; 8]) = (0, 0, [[0; 8]; 8]);
            let mut findout = false;

            for _point in &effect {
                if _point.0 == col_index_on_board && _point.1 == row_index_on_board {
                    findout = true;
                    find = *_point;
                }
            }

            if effect.len() == 0 {
                if self.state == GameState::BlackTurn {
                    self.state = GameState::WhiteTurn;
                } else if self.state == GameState::WhiteTurn {
                    self.state = GameState::BlackTurn;
                }
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
            let mut p = peice.clone();
            match peice {
                Peice::WHITE => p = Peice::BLACK,
                Peice::BLACK => p = Peice::WHITE,
                Peice::EMPTY => (),
            }

            let effect = Game::the_point_if_you_can_place_and_effect(find.2, p);

            if effect.len() == 0 {
                if self.state == GameState::BlackTurn {
                    self.state = GameState::WhiteTurn;
                } else if self.state == GameState::WhiteTurn {
                    self.state = GameState::BlackTurn;
                }
            }
            return Ok(());
        }
    }

    pub fn the_point_if_you_can_place_and_effect(
        board: [[i8; 8]; 8],
        peice: Peice,
    ) -> Vec<(usize, usize, [[i8; 8]; 8])> {
        let mut result = vec![];
        let peice_number = peice as i8;

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let mut result_board = board;
                if (board[i][j] != 0) {
                    continue;
                }
                let effect_points = Game::_find_board(&board, i, j, &peice_number);
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

    pub fn _find_board(
        _board: &[[i8; 8]; 8],
        x: usize,
        y: usize,
        peice_number: &i8,
    ) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let mut tmp_result = vec![];
        //right
        let mut i = y + 1;
        tmp_result.clear();
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
        tmp_result.clear();
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
        tmp_result.clear();
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
        tmp_result.clear();
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
        tmp_result.clear();
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
        tmp_result.clear();
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
        tmp_result.clear();
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
        tmp_result.clear();
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
