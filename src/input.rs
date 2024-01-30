use crate::game::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::graphics::BoardView;
use raylib::prelude::*;

#[derive(Debug)]
pub struct UserInput {
    pub space_pressed: bool,
    pub r_pressed: bool,
    pub mouse_clicked: bool,
    pub mouse_column: Option<usize>,
    pub mouse_row: Option<usize>,
}

impl UserInput {
    pub fn new() -> Self {
        Self {
            space_pressed: false,
            r_pressed: false,
            mouse_clicked: false,
            mouse_column: None,
            mouse_row: None,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, board_view: &BoardView) {
        self.space_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        self.r_pressed = rl.is_key_pressed(KeyboardKey::KEY_R);
        self.mouse_clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);
        self.mouse_column = None;
        self.mouse_row = None;

        let m_col = (rl.get_mouse_x() as f32 - board_view.board_padding) / board_view.tile_size;
        if m_col >= 0.0 && m_col < BOARD_WIDTH as f32 {
            self.mouse_column = Some(m_col.floor() as usize);
        }

        let m_row = (rl.get_mouse_y() as f32 - board_view.board_padding) / board_view.tile_size;
        if m_row >= 0.0 && m_row < BOARD_HEIGHT as f32 {
            self.mouse_row = Some(m_row.floor() as usize);
        }
    }
}
