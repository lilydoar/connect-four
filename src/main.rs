mod game;
mod graphics;
mod input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user_input = input::UserInput::new();
    let mut game = game::Game::new();
    let graphics = graphics::Graphics::new();

    let (width, height) = graphics.window_size();
    let (mut rl, thread) = raylib::init().size(width as i32, height as i32).build();

    user_input.update(&mut rl, graphics.board_view());
    graphics.draw(&mut rl, &thread, &game);

    while !rl.window_should_close() {
        user_input.update(&mut rl, graphics.board_view());
        game.update(&user_input);
        graphics.draw(&mut rl, &thread, &game);
    }

    Ok(())
}
