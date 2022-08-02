use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {

    loop {
        clear_background(BLACK);

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
