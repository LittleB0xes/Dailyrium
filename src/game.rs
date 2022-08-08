use macroquad::prelude::*;

use crate::architect::Stage;
use crate::dailyrium::Terminal;
use crate::dailyrium::utils::*;
use crate::hero::Hero;
use crate::nursery::*;

pub struct Game {
    terminal: Terminal,
    texture: Texture2D,
    hero: Hero,
    current_stage: Stage,
    living_entities: Vec<LivingEntity>,
    turn: u32,
    manor_turn: bool,
}

impl Game {
    pub fn new() -> Self {
        let terminal = Terminal::new(80, 45, 16, 16, 1.0, 2);
        let texture =
            Texture2D::from_file_with_format(include_bytes!("../assets/16x16_rounded.png"), None);
        texture.set_filter(FilterMode::Nearest);

        let current_stage = Stage::new(0, 80, 45);
        let hero = Hero::new(10, 10);
        let living_entities = spawn_monsters(20, &current_stage);

        Self {
            terminal,
            texture,
            hero,
            current_stage,
            turn: 0,
            manor_turn: false,
            living_entities,
        }
    }

    pub fn tick(&mut self) {
        let mut visible_tile: Vec<(i32, i32)> = Vec::new();
        if !self.manor_turn {
            self.manor_turn = self.hero.update(&self.current_stage);
            visible_tile = fov_raycast(self.hero.x, self.hero.y, 10, &mut self.current_stage.stage_map, self.current_stage.width, self.current_stage.height);
        } else {
            self.turn += 1;
            for monster in self.living_entities.iter_mut() {
                monster.update(&mut self.current_stage)
            }
            self.manor_turn = false;
        }

        // Clear all the scene
        self.terminal.bg_color(BLACK);
        self.terminal.fill(' ' as u16);
        // Place display
        for element in self.current_stage.stage_map.iter() {
            if element.seen {
                self.terminal.put_ex(
                    element.x as u32,
                    element.y as u32,
                    element.glyph,
                    element.fg_color,
                    element.bg_color,
                );

            }
        }

        for monster in self.living_entities.iter() {
            if visible_tile.contains(&(monster.x, monster.y)) {
                self.terminal.put_ex(
                monster.x as u32,
                monster.y as u32,
                monster.glyph,
                monster.fg_color,
                self.terminal.pick_bg(monster.x as u32, monster.y as u32),
            );
            }
            

        }
        self.terminal.put_ex(
            self.hero.x as u32,
            self.hero.y as u32,
            self.hero.glyph,
            self.hero.fg_color,
            self.terminal.pick_bg(self.hero.x as u32, self.hero.y as u32),
        );
        clear_background(BLACK);
        //let fps = format!("fps: {}", get_fps() as u32);
        //self.terminal.print(0, 0, fps);
        let turn = format!("Turn: {}", self.turn);
        self.terminal.print(0, 1, turn);
        self.terminal.render(self.texture);
    }
}
