use crate::game::{Board, Game, Piece, Player, State, BOARD_HEIGHT, BOARD_WIDTH};
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
    text: Color,
}

pub struct BoardView {
    pub tile_size: f32,
    pub tile_padding: f32,
    pub board_padding: f32,
}

impl Graphics {
    pub fn new() -> Self {
        Self {
            pallete: Pallete::default(),
            view: BoardView::default(),
        }
    }

    pub fn draw(&self, rl: &mut RaylibHandle, thread: &RaylibThread, game: &Game) {
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
            text: Color::from_hex("E3B505").unwrap(),
        }
    }
}

impl BoardView {
    fn default() -> Self {
        Self {
            tile_size: 128.0,
            tile_padding: 8.0,
            board_padding: 48.0,
        }
    }

    fn text_size(&self) -> f32 {
        self.board_padding * 0.9
    }

    fn text_padding(&self) -> f32 {
        self.board_padding * 0.1 / 2.0
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
                self.draw_tie(d);
            }
        }
    }

    fn draw_board(&self, d: &mut RaylibDrawHandle, board: &Board) {
        // Draw board
        d.draw_rectangle(
            self.view.board_padding.round() as i32,
            self.view.board_padding.round() as i32,
            (self.view.tile_size * BOARD_WIDTH as f32).round() as i32,
            (self.view.tile_size * BOARD_HEIGHT as f32).round() as i32,
            self.pallete.board,
        );

        // Draw pieces
        for row in 0..6 {
            for column in 0..7 {
                let (x, y) = self.view.piece_position(row, column);
                let radius = self.view.piece_radius();
                let color = match board.get(row, column) {
                    Piece::Empty => self.pallete.background,
                    Piece::Full(player) => match player {
                        Player::Player1 => self.pallete.piece_player_1,
                        Player::Player2 => self.pallete.piece_player_2,
                    },
                };

                d.draw_circle(x.round() as i32, y.round() as i32, radius, color);
            }
        }
    }

    fn draw_player_turn(&self, d: &mut RaylibDrawHandle, player: Player) {
        let text = match player {
            Player::Player1 => "Player 1",
            Player::Player2 => "Player 2",
        };
        d.draw_text(
            format!("Turn: {}", text).as_str(),
            self.view.board_padding.round() as i32,
            self.view.text_padding().round() as i32,
            self.view.text_size() as i32,
            self.pallete.text,
        );
    }

    fn draw_winner(&self, d: &mut RaylibDrawHandle, player: Player) {
        let text = match player {
            Player::Player1 => "Player 1",
            Player::Player2 => "Player 2",
        };
        d.draw_text(
            format!("{} wins! Press space to restart", text).as_str(),
            self.view.board_padding.round() as i32,
            self.view.text_padding().round() as i32,
            self.view.text_size() as i32,
            self.pallete.text,
        );
    }

    fn draw_tie(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Tie! Press space to restart",
            self.view.board_padding.round() as i32,
            self.view.text_padding().round() as i32,
            self.view.text_size() as i32,
            self.pallete.text,
        );
    }

    pub fn window_size(&self) -> (f32, f32) {
        let width = self.view.tile_size * BOARD_WIDTH as f32 + self.view.board_padding * 2.0;
        let height = self.view.tile_size * BOARD_HEIGHT as f32 + self.view.board_padding * 2.0;
        (width, height)
    }

    pub fn board_view(&self) -> &BoardView {
        &self.view
    }
}

impl BoardView {
    fn piece_position(&self, row: usize, column: usize) -> (f32, f32) {
        let x = self.board_padding + self.tile_size * column as f32 + self.tile_size / 2.0;
        let y = self.board_padding
            + self.tile_size * (BOARD_HEIGHT - 1 - row) as f32
            + self.tile_size / 2.0;
        (x, y)
    }

    fn piece_radius(&self) -> f32 {
        self.tile_size / 2.0 - self.tile_padding
    }
}
