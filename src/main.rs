use std::fmt;

struct Board {
    board: [[i8; 8]; 8],
    finished: bool,
    black: i8,
    white: i8,
}

impl Board {
    fn new() -> Board {
        let mut board = Board {
            board: [[0; 8]; 8],
            finished: false,
            black: -1,
            white: 1,
        };
        board.board[3][3] = board.white;
        board.board[3][4] = board.black;
        board.board[4][3] = board.black;
        board.board[4][4] = board.white;
        board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "\x1B[30m\x1B[42m{}\x1B[0m", "   a b c d e f g h \n");
        for i in 0..self.board.len() {
            write!(f, "\x1B[30m\x1B[42m{} {} \x1B[0m", "", i + 1);
            for row in &self.board[i] {
                match row {
                    //TODO change -1 to black
                    &-1 => write!(f, "\x1B[30m\x1B[42m{} \x1B[0m", "●"),
                    &1 => write!(f, "\x1B[37m\x1B[42m{} \x1B[0m", "●"),
                    _ => write!(f, "\x1B[42m{} \x1B[0m", "-"),
                };
            }
            write!(f, "{}", "\n");
        }
        std::result::Result::Ok(())
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board);
}
