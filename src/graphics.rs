use crate::game::{Board, Game, Player, State, Piece};
use raylib::prelude::*;

pub struct Graphics {
    pallete: Pallete,
    view: BoardView,
}

struct Pallete {
    background: Color,
    board: Color,
    piece_player_1: Color,
    piece_player_2: Color,
}

struct BoardView {
    tile_size: f32,
    tile_padding: f32,
    board_padding: f32,
}

impl Graphics {
    pub fn new() -> Self {
        Self {
            pallete: Pallete::default(),
            view: BoardView::default(),
        }
    }

    pub fn draw(&self, rl: &RaylibHandle, thread: &RaylibThread, game: &Game) {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(self.pallete.background);
        self.draw_game(&mut d, game);
    }
}

impl Pallete {
    fn default() -> Self {
        Self {
            background: Color::from_hex("31314e").unwrap(),
            board: Color::from_hex("685b67").unwrap(),
            piece_player_1: Color::from_hex("72afa7").unwrap(),
            piece_player_2: Color::from_hex("aabc6f").unwrap(),
        }
    }
}

impl BoardView {
    fn default() -> Self {
        Self {
            tile_size: 64.0,
            tile_padding: 4.0,
            board_padding: 16.0,
        }
    }
}

impl Graphics {
    fn draw_game(&self, d: &mut RaylibDrawHandle, game: &Game) {
        match game.state {
            State::Turn(player) => {
                self.draw_board(d, &game.board);
                self.draw_player_turn(d, player);
            }
            State::Win(player) => {
                self.draw_board(d, &game.board);
                self.draw_winner(d, player);
            }
            State::Tie => {
                self.draw_board(d, &game.board);
                self.draw_tie_(d);
            }
        }
    }

    fn draw_board(&self, d: &mut RaylibDrawHandle, board: &Board) {
        // Draw board
        d.draw_rectangle(
            self.view.board_padding,
            self.view.board_padding,
            self.view.tile_size * 7.0,
            self.view.tile_size * 6.0,
            self.pallete.board,
        );

        // Draw pieces
        for row in 0..6 {
            for column in 0..7 {
                let piece = board.get(row, column);
                let (x, y) = self.view.piece_position(row, column);  
                let radius = self.view.piece_radius();

                match piece {
                    Piece::Empty => {}
                    Piece::Full(Player::Player1) => {
                        d.draw_circle(x, y, radius, self.pallete.piece_player_1);
                    }
                    Piece::Full(Player::Player2) => {
                        d.draw_circle(x, y, radius, self.pallete.piece_player_2);
                    }
                }
            }
        }
    }

    fn draw_player_turn(&self, d: &mut RaylibDrawHandle, player: Player) {}

    fn draw_winner(&self, d: &mut RaylibDrawHandle, player: Player) {}

    fn draw_tie_(&self, d: &mut RaylibDrawHandle) {}
}

impl BoardView {
    fn piece_position(&self, row: usize, column: usize) -> (f32, f32) {
        todo!()
    }

    fn piece_radius(&self) -> f32 {
        todo!()
    }
}
