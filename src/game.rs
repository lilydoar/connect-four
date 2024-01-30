use crate::input::UserInput;
use log::warn;
use raylib::prelude::*;

pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;

pub struct Game {
    pub board: Board,
    pub state: State,
}

pub struct Board {
    pieces: [Piece; BOARD_WIDTH * BOARD_HEIGHT],
}

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    Empty,
    Full(Player),
}

pub enum State {
    Turn(Player),
    Win(Player),
    Tie,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    Player1,
    Player2,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::empty(),
            state: State::Turn(Player::Player1),
        }
    }

    pub fn update(&mut self, input: &UserInput) {
        // Restart game
        if input.r_pressed {
            let first_player = match self.state {
                State::Turn(_) => random_player(),
                State::Win(player) => player,
                State::Tie => random_player(),
            };

            self.board = Board::empty();
            self.state = State::Turn(first_player);
        }

        // Handle turn
        if let State::Turn(player) = self.state {
            self.handle_turn(player, input);
            return;
        }
    }

    fn handle_turn(&mut self, player: Player, input: &UserInput) {
        if !input.mouse_clicked {
            return;
        }

        let (m_col, _) = match (input.mouse_column, input.mouse_row) {
            (Some(m_col), Some(m_row)) => (m_col, m_row),
            _ => return,
        };

        if self.board.column_full(m_col) {
            return;
        } else {
            self.board.place_piece(m_col, player);
        }

        if self.board.check_win(player) {
            self.state = State::Win(player);
            return;
        }

        if self.board.check_tie() {
            self.state = State::Tie;
            return;
        }

        self.state = match player {
            Player::Player1 => State::Turn(Player::Player2),
            Player::Player2 => State::Turn(Player::Player1),
        };
    }
}

impl Board {
    fn empty() -> Self {
        Self {
            pieces: [Piece::Empty; BOARD_WIDTH * BOARD_HEIGHT],
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Piece {
        if row >= BOARD_HEIGHT || column >= BOARD_WIDTH {
            warn!(
                "Board::get: row or column out of bounds: row={}, column={}",
                row, column
            );
            return Piece::Empty;
        }
        self.pieces[row * BOARD_WIDTH + column]
    }

    pub fn column_full(&self, column: usize) -> bool {
        self.get(BOARD_HEIGHT - 1, column) != Piece::Empty
    }

    pub fn place_piece(&mut self, column: usize, player: Player) {
        for row in 0..BOARD_HEIGHT {
            if self.get(row, column) == Piece::Empty {
                self.pieces[row * BOARD_WIDTH + column] = Piece::Full(player);
                return;
            }
        }
    }

    pub fn check_win(&self, player: Player) -> bool {
        // Horizontal
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH - 3 {
                let i = row * BOARD_WIDTH + col;
                if convolution_op(i, &WIN_HORIZONTAL, &self.pieces, player) {
                    return true;
                }
            }
        }

        // Vertical
        for row in 0..(BOARD_HEIGHT - 3) {
            for col in 0..BOARD_WIDTH {
                let i = row * BOARD_WIDTH + col;
                if convolution_op(i, &WIN_VERTICAL, &self.pieces, player) {
                    return true;
                }
            }
        }

        // Diagonal bottom left
        for row in 0..(BOARD_HEIGHT - 3) {
            for col in 0..(BOARD_WIDTH - 3) {
                let i = row * BOARD_WIDTH + col;
                if convolution_op(i, &WIN_DIAGONAL_BOTTOM_LEFT, &self.pieces, player) {
                    return true;
                }
            }
        }

        // Diagonal bottom right
        for row in 0..(BOARD_HEIGHT - 3) {
            for col in 3..BOARD_WIDTH {
                let i = row * BOARD_WIDTH + col;
                if convolution_op(i, &WIN_DIAGONAL_BOTTOM_RIGHT, &self.pieces, player) {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_tie(&self) -> bool {
        for col in 0..BOARD_WIDTH {
            if !self.column_full(col) {
                return false;
            }
        }
        true
    }
}

fn random_player() -> Player {
    if get_random_value::<i32>(0, 1) == 0 {
        Player::Player1
    } else {
        Player::Player2
    }
}

// Describe the 4 win orientations as offsets from the current index
const WIN_HORIZONTAL: [usize; 4] = [0, 1, 2, 3];
const WIN_VERTICAL: [usize; 4] = [0, BOARD_WIDTH, BOARD_WIDTH * 2, BOARD_WIDTH * 3];
const WIN_DIAGONAL_BOTTOM_LEFT: [usize; 4] =
    [0, BOARD_WIDTH + 1, BOARD_WIDTH * 2 + 2, BOARD_WIDTH * 3 + 3];
const WIN_DIAGONAL_BOTTOM_RIGHT: [usize; 4] =
    [0, BOARD_WIDTH - 1, BOARD_WIDTH * 2 - 2, BOARD_WIDTH * 3 - 3];

fn convolution_op(index: usize, win_arr: &[usize], pieces: &[Piece], player: Player) -> bool {
    // Check a win orientation against a single index in the board
    win_arr
        .iter()
        .filter(|i| pieces[index + *i] == Piece::Full(player))
        .count()
        == 4
}
