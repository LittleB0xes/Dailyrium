use macroquad::prelude::*;

mod dailyrium;
use dailyrium::{Terminal};

mod architect;
use architect::Stage;

mod hero;
use hero::Hero;


#[macroquad::main(window_conf)]
async fn main() {
    let mut terminal = Terminal::new(60, 44, 20,20, 1, 2 );
    let texture = Texture2D::from_file_with_format(include_bytes!("../assets/20x20_oreslam.png"), None);
    texture.set_filter(FilterMode::Nearest);

    let stage = Stage::new(0,60, 34);
    let mut hero = Hero::new(10, 10);
    loop {

        hero.update();

        for element in stage.stage_map.iter() {
            terminal.put(element.x, element.y, element.glyph)
        }
        terminal.put(hero.x, hero.y, hero.glyph);
        clear_background(BLACK);
        let fps = format!("FPS: {}", get_fps() as i32);
        terminal.print(0, 1, fps);
        terminal.render(texture);

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dailyrium".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        //high_dpi: true,
        ..Default::default()
    }
}
