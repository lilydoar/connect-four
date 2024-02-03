use game::{Game, Player};
use graphics::Graphics;
use input::{construct_action, Input};

mod game;
mod graphics;
mod input;
mod randomizer;

fn main() {
    let mut game = Game::new(&Player::Player1);
    let graphics = Graphics::default();

    let (width, height) = graphics.view.window_size();
    let (mut rl, thread) = raylib::init().size(width, height).build();

    graphics.draw(&mut rl.begin_drawing(&thread), &game);

    while !rl.window_should_close() {
        let input = Input::poll(&mut rl);
        if let Some(action) = construct_action(&input, &game, &graphics.view) {
            game.handle_action(&action);
        }
        graphics.draw(&mut rl.begin_drawing(&thread), &game);
    }
}
