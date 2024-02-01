use crate::game::{Board, Game, Player, State, Tile, BOARD_HEIGHT, BOARD_WIDTH};
use raylib::prelude::*;

#[derive(Default)]
pub struct Graphics {
    pub view: View,
    pallete: Pallete,
}

pub struct View {
    tile_size: i32,
    _tile_padding: i32,
    tile_radius: f32,
    text_size: i32,
    text_padding: i32,
    board_padding: i32,
}

struct Pallete {
    background: Color,
    board: Color,
    player_1: Color,
    player_2: Color,
    text: Color,
}

impl Graphics {
    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &Game) {
        d.clear_background(self.pallete.background);

        self.draw_board(d, game.board());
        self.draw_text(d, game);
    }

    fn draw_board(&self, d: &mut RaylibDrawHandle, board: &Board) {
        let (b_width, b_height) = self.view.board_size();
        let (b_x, b_y) = self.view.board_position();

        d.draw_rectangle(b_x, b_y, b_width, b_height, self.pallete.board);

        for row in 0..BOARD_HEIGHT {
            for column in 0..BOARD_WIDTH {
                let tile = board[row * BOARD_WIDTH + column];
                self.draw_tile(d, &tile, row, column);
            }
        }
    }

    fn draw_tile(&self, d: &mut RaylibDrawHandle, tile: &Tile, row: usize, column: usize) {
        let (t_x, t_y) = self.view.tile_position(row, column);
        let color = match tile {
            Tile::Empty => self.pallete.background,
            Tile::Full(player) => match player {
                Player::Player1 => self.pallete.player_1,
                Player::Player2 => self.pallete.player_2,
            },
        };

        d.draw_circle(t_x, t_y, self.view.tile_radius, color);
    }

    fn draw_text(&self, d: &mut RaylibDrawHandle, game: &Game) {
        let text = match game.state() {
            State::Turn(player) => format!("Turn: {}", player.as_str()),
            State::Win(player) => format!("{} wins! Press r to restart", player.as_str()),
            State::Tie => "Tie! Press r to restart".to_string(),
        };
        let (t_x, t_y) = self.view.text_position();

        d.draw_text(
            text.as_str(),
            t_x,
            t_y,
            self.view.text_size,
            self.pallete.text,
        );
    }
}

impl Default for View {
    fn default() -> Self {
        let tile_size = 100;
        let tile_padding = 10;

        Self {
            tile_size,
            _tile_padding: tile_padding,
            tile_radius: tile_size as f32 / 2.0 - tile_padding as f32,
            text_size: 40,
            text_padding: 10,
            board_padding: 15,
        }
    }
}

impl View {
    pub fn window_size(&self) -> (i32, i32) {
        let (b_width, b_height) = self.board_size();
        (
            b_width + self.board_padding * 2,
            b_height + self.text_size + self.text_padding * 2 + self.board_padding,
        )
    }

    fn board_size(&self) -> (i32, i32) {
        (
            self.tile_size * BOARD_WIDTH as i32,
            self.tile_size * BOARD_HEIGHT as i32,
        )
    }

    fn board_position(&self) -> (i32, i32) {
        (self.board_padding, self.text_size + self.text_padding * 2)
    }

    fn tile_position(&self, row: usize, column: usize) -> (i32, i32) {
        let (b_x, b_y) = self.board_position();

        (
            b_x + column as i32 * self.tile_size + self.tile_size / 2,
            b_y + (BOARD_HEIGHT - 1 - row) as i32 * self.tile_size + self.tile_size / 2,
        )
    }

    fn text_position(&self) -> (i32, i32) {
        (self.board_padding, self.text_padding)
    }

    pub fn column_from_x(&self, x: i32) -> Option<usize> {
        let x = x - self.board_padding;
        match x {
            x if x < 0 || x >= self.tile_size * BOARD_WIDTH as i32 => None,
            _ => Some((x / self.tile_size) as usize),
        }
    }
}

impl Default for Pallete {
    fn default() -> Self {
        Self {
            background: Color::from_hex("B4C292").unwrap(),
            board: Color::from_hex("736F4E").unwrap(),
            player_1: Color::from_hex("D17A22").unwrap(),
            player_2: Color::from_hex("4C061D").unwrap(),
            text: Color::from_hex("3B3923").unwrap(),
        }
    }
}
