use macroquad::prelude::*;

mod dailyrium;
use dailyrium::{Terminal};


#[macroquad::main(window_conf)]
async fn main() {
    let mut terminal = Terminal::new(80, 45, 16, 16, 2 );
    let texture = Texture2D::from_file_with_format(include_bytes!("../assets/16x16_yun.png"), None);

    terminal.layer(0);
    terminal.fill('.' as u16);
    terminal.fg_color(GREEN);
    terminal.bg_color(BLUE);
    terminal.fill_layer_area(0, '@' as u16,2,2,10,10);

    terminal.layer(1);
    terminal.fg_color(BEIGE);
    terminal.bg_color(Color::new(0.0,0.0,0.0,0.0)); 
    terminal.fill_layer_area(1, '0' as u16,5,3,10,15);

    terminal.layer(0);
    terminal.put_ex(20, 20, 'A' as u16, BLACK, RED);

    loop {
        clear_background(BLACK);
        terminal.print(0, 1, format!("FPS: {}", get_fps() as i32));
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
        //high_dpi: true,
        ..Default::default()
    }
}
