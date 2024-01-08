use raylib::prelude::*;
use log::warn;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

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

#[derive(Copy, Clone, PartialEq)]
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

    pub fn update(&mut self, rl: &RaylibHandle) {}
}

impl Board {
    fn empty() -> Self {
        Self {
            pieces: [Piece::Empty; BOARD_WIDTH * BOARD_HEIGHT],
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Piece {
        if row >= BOARD_HEIGHT || column >= BOARD_WIDTH {
            warn!("Board::get: row or column out of bounds: row={}, column={}", row, column);
            return Piece::Empty;
        }
        self.pieces[row * BOARD_WIDTH + column]
    }
}
