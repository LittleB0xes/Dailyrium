use macroquad::prelude::*;

mod dailyrium;
use dailyrium::{Terminal};


#[macroquad::main(window_conf)]
async fn main() {
    let mut terminal = Terminal::new(48, 32, 16, 16, 3 );
    let texture = Texture2D::from_file_with_format(include_bytes!("../assets/16x16_yun.png"), None);

    terminal.fill_layer_area(1, '@' as u16,2,2,10,10);
    terminal.fill_layer_area(1, '0' as u16,5,3,10,15);

    loop {
        clear_background(BLACK);
        //cell.draw(texture);
        terminal.render(texture);


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
