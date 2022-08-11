use std::num::NonZeroI128;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::architect::{Stage, GenerationType};
use crate::termquad::Terminal;
use crate::dailyrium::utils::*;
use crate::dailyrium::pathfinder::path_finder;
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

        let current_stage = Stage::new(0, 80, 45, GenerationType::Room );

        let mut x_candidate = gen_range(0, current_stage.width - 1);
        let mut y_candidate = gen_range(0, current_stage.height - 1);
        while !current_stage.stage_map[(x_candidate + y_candidate * current_stage.width) as usize ].crossable {
            x_candidate = gen_range(0, current_stage.width - 1);
            y_candidate = gen_range(0, current_stage.height - 1);
        }
        let hero = Hero::new(x_candidate, y_candidate);
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

        // All game's update her 
        let mut visible_tile: Vec<(i32, i32)> = Vec::new();
        
        // Player turn
        if !self.manor_turn {
            self.manor_turn = self.hero.update(&self.current_stage);
            visible_tile = fov_raycast(self.hero.x, self.hero.y, 10, &mut self.current_stage.stage_map, self.current_stage.width, self.current_stage.height);
        }
        // Manor and monster turn
        else {
            self.turn += 1;
            for monster in self.living_entities.iter_mut() {
                monster.update(&mut self.current_stage)
            }

            for element in self.current_stage.stage_map.iter_mut() {
                if element.visited && !element.seen {
                    element.alzheimerize();
                }
            }
            self.manor_turn = false;

        }


        // All drawing stuff below

        // Clear all the scene
        self.terminal.bg_color(BLACK);
        self.terminal.layer(1);
        self.terminal.fill(' ' as u16);
        self.terminal.layer(0);
        self.terminal.fill(' ' as u16);


        // Manor, player, monster... on layer 0
        // Place display
        for element in self.current_stage.stage_map.iter_mut() {
            if element.seen {
                let dist = (((self.hero.x - element.x).pow(2) + (self.hero.y - element.y).pow(2)) as f32).sqrt();
                let factor = -(1.0 - 0.25) / 10.0 * dist + 1.0;
                
                let bg_color = mul_color(element.bg_color, factor);
                let fg_color = mul_color(element.fg_color, factor);
                self.terminal.put_ex(
                    element.x as u32,
                    element.y as u32,
                    element.glyph,
                    fg_color,
                    bg_color,
                );

            }
            else if element.visited {
                let bg_color = mul_color(element.bg_color, 0.25 * element.alois_factor);
                let fg_color = mul_color(element.fg_color, 0.25 * element.alois_factor);
                self.terminal.put_ex(
                    element.x as u32,
                    element.y as u32,
                    element.glyph,
                    fg_color,
                    bg_color,
                );
            }
        }

        for monster in self.living_entities.iter() {
            if visible_tile.contains(&(monster.x, monster.y)) {
                let dist = (((self.hero.x - monster.x).pow(2) + (self.hero.y - monster.y).pow(2)) as f32).sqrt();
                let factor = -(1.0 - 0.25) / 10.0 * dist + 1.0;
                
                let bg_color = mul_color(self.terminal.pick_bg(monster.x as u32, monster.y as u32), factor);
                let fg_color = mul_color(monster.fg_color, factor);
                self.terminal.put_ex(
                monster.x as u32,
                monster.y as u32,
                monster.glyph,
                fg_color,
                bg_color,
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
        let fps = format!("fps: {}", get_fps() as u32);
        //self.terminal.print(0, 0, fps);
        
        
        // Pathfinding
        let mouse_x = (mouse_position().0 / 16.0) as i32;
        let mouse_y = (mouse_position().1 / 16.0) as i32;

        let mut path_option: Option<Vec<(i32, i32)>> =  None;
        
        let width = self.current_stage.width;
        let height = self.current_stage.height;
        if inside_rect(mouse_x, mouse_y, 0, 0, width-1, height-1) && self.current_stage.stage_map[(mouse_x + mouse_y * width) as usize].crossable && self.current_stage.stage_map[(mouse_x + mouse_y * width) as usize].visited {
            path_option = path_finder(self.hero.x, self.hero.y, mouse_x, mouse_y, &self.current_stage.stage_map, width, height);
            
        }
        
        // HUD info on layer 1
        self.terminal.layer(1);

        // Path finder result (if ther has)
        match path_option {
            Some(path) => {
                for t in path.iter() {
                    self.terminal.put_ex(
                    t.0 as u32,
                    t.1 as u32,
                    ' ' as u16,
                    WHITE,
                    Color::new(0.9, 0.8, 0.0, 0.3),
                    );
                };

                if is_mouse_button_pressed(MouseButton::Left) {
                    self.hero.set_autorun(path);
                }
            },
            None =>{},
        }
        // Show the path
        
        self.terminal.layer(0);
        //self.terminal.print(0, 1, mouse);
        self.terminal.render(self.texture);
    }
}
