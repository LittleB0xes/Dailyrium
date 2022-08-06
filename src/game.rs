use macroquad::prelude::*;

use crate::dailyrium::Terminal;
use crate::hero::Hero;
use crate::architect::Stage;

pub struct Game {
    terminal: Terminal,
    texture: Texture2D,
    hero: Hero,
    current_stage: Stage,
    turn: u32,
    manor_turn: bool,
}

impl Game {
    pub fn new() -> Self {
        let terminal = Terminal::new(80, 45, 16,16, 1.0, 2 );
        let texture = Texture2D::from_file_with_format(include_bytes!("../assets/16x16_rounded.png"), None);
        texture.set_filter(FilterMode::Nearest);

        let current_stage = Stage::new(0, 80, 45);
        let hero = Hero::new(10,10);


        Self {
            terminal,
            texture,
            hero,
            current_stage,
            turn: 0,
            manor_turn: false,
        }
    }

    pub fn tick(&mut self) {
        if !self.manor_turn {
            self.manor_turn = self.hero.update(&self.current_stage);

        }
        else {
            self.turn += 1;
            println!("{} - Manor Turn !", self.turn);
            self.manor_turn = false;

        }

        for element in self.current_stage.stage_map.iter() {
            self.terminal.put_ex(element.x, element.y, element.glyph, element.fg_color, element.bg_color);
        }
        self.terminal.put_ex(self.hero.x, self.hero.y, self.hero.glyph, self.hero.fg_color, BLACK);
        clear_background(BLACK);
        let fps = format!("FPS: {}", get_fps() as i32);
        self.terminal.print(0, 1, fps);
        self.terminal.render(self.texture);
    }
}