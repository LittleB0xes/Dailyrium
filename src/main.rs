use macroquad::prelude::*;

mod dailyrium;
use dailyrium::{Terminal, Cell};


#[macroquad::main(window_conf)]
async fn main() {
    let mut terminal = Terminal::new(48, 32, 16, 16, 1 );
    let texture = Texture2D::from_file_with_format(include_bytes!("../assets/16x16_yun.png"), None);
    let cell = Cell::new(4, 6, '.' as u16);

    loop {
        clear_background(BLACK);
        cell.draw(texture);

        /*
            Future "Great" Stuff here :-D
        
        */


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
