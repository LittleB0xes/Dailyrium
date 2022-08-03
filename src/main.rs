use macroquad::prelude::*;

mod dailyrium;
use dailyrium::{Terminal};

mod architect;
use architect::Stage;


#[macroquad::main(window_conf)]
async fn main() {
    let mut terminal = Terminal::new(80, 45, 16, 16, 1, 2 );
    let texture = Texture2D::from_file_with_format(include_bytes!("../assets/16x16_yun.png"), None);
    texture.set_filter(FilterMode::Nearest);

    let stage = Stage::new(0,80, 45);
    loop {
        for element in stage.stage_map.iter() {
            terminal.put(element.x, element.y, element.glyph)
        }
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
