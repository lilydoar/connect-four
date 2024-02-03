use crate::game::{Action, Game, Player, State};
use crate::graphics::View;
use crate::randomizer::RaylibRandomizer;
use raylib::prelude::*;

pub struct Input {
    pub r_pressed: bool,
    pub mouse_clicked: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl Input {
    pub fn poll(rl: &mut RaylibHandle) -> Self {
        Self {
            r_pressed: rl.is_key_pressed(KeyboardKey::KEY_R),
            mouse_clicked: rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
            mouse_x: rl.get_mouse_x(),
            mouse_y: rl.get_mouse_y(),
        }
    }
}

pub fn construct_action(input: &Input, game: &Game, view: &View) -> Option<Action> {
    if input.r_pressed {
        let player = match game.state() {
            State::Turn(_) | State::Tie => Player::random(&mut RaylibRandomizer),
            State::Win(player) => *player,
        };
        return Some(Action::Restart(player));
    }

    if let State::Turn(_) = game.state() {
        if input.mouse_clicked {
            let col = view.column_from_x(input.mouse_x)?.into();
            if game.column_is_full(&col) {
                return None;
            }
            return Some(Action::PlaceTile(col));
        }
    }

    None
}
