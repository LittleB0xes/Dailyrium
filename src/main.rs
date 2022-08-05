use macroquad::prelude::*;

mod dailyrium;
mod architect;
mod hero;

mod game;
use game::Game;


#[macroquad::main(window_conf)]
async fn main() {

    let mut game = Game::new();
    loop {

        game.tick();

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dailyrium".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}
