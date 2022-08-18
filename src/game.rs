use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::architect::{Stage, GenerationType, Manor};
use crate::termquad::Terminal;
use crate::dailyrium::utils::*;
use crate::dailyrium::pathfinder::path_finder;
use crate::hero::Hero;
use crate::nursery::*;

pub struct Game {
    terminal: Terminal,
    hero: Hero,
    manor: Manor,
    living_entities: Vec<Vec<LivingEntity>>,
    turn: u32,
    manor_turn: bool,
    current_stage_id: usize,
}

impl Game {
    pub fn new() -> Self {

        rand::srand(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64);

        // Terminal creation
        let texture =
            Texture2D::from_file_with_format(include_bytes!("../assets/16x16_sm.png"), None);
        texture.set_filter(FilterMode::Nearest);
        let terminal = Terminal::new(80, 45, 16, 16, 1.0, 2, texture);


        let manor: Manor = Manor::new(80, 45, 2);


        // Search a free place to spawn the hero
        let mut x_candidate = gen_range(0, manor.stages[0].width - 1);
        let mut y_candidate = gen_range(0, manor.stages[0].height - 1);
        while !manor.stages[0].stage_map[(x_candidate + y_candidate * manor.stages[0].width) as usize ].crossable {
            x_candidate = gen_range(0, manor.stages[0].width - 1);
            y_candidate = gen_range(0, manor.stages[0].height - 1);
        }

        // Create the hero ...
        let hero = Hero::new(x_candidate, y_candidate);

        //... and spawn some test monster
        // each element of living_entities works with a manor stage (index of living_entities is an ID's level)
        let mut living_entities: Vec<Vec<LivingEntity>> = Vec::new();
        for i in 0..manor.stages_number {
            let entities = spawn_monsters(20, &manor.stages[i as usize]);
            living_entities.push(entities);
        }

        Self {
            terminal,
            hero,
            turn: 0,
            manor_turn: false,
            living_entities,
            manor: Manor::new(80, 45, 2),
            current_stage_id: 0,
        }
    }

    pub fn tick(&mut self) {
        let stage_id = self.current_stage_id;
        if is_key_pressed(KeyCode::A) {
            self.current_stage_id += 1;
        }
        // All game's update her 

        // all tile in the player fov
        let mut visible_tile: Vec<(i32, i32)> = Vec::new();
        
        // Player turn
        if !self.manor_turn {
            let width = self.manor.stages[stage_id].width;
            let height = self.manor.stages[stage_id].height;
            self.manor_turn = self.hero.update(&self.manor.stages[stage_id]);
            visible_tile = fov_raycast(self.hero.x, self.hero.y, 10, &mut self.manor.stages[stage_id].stage_map, width, height);
            }
        // Manor and monster turn
        else {
            self.turn += 1;
            for list in self.living_entities.iter_mut() {
                for monster in list.iter_mut() {

                    monster.update(&mut self.manor.stages[stage_id]);
                }
            }

            for element in self.manor.stages[stage_id].stage_map.iter_mut() {
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
        for element in self.manor.stages[stage_id].stage_map.iter_mut() {
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

        for monster in self.living_entities[self.current_stage_id].iter() {
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
        //let fps = format!("fps: {}", get_fps() as u32);
        //self.terminal.print(0, 0, fps);
        
        
        // Pathfinding
        let mouse_x = (mouse_position().0 / 16.0) as i32;
        let mouse_y = (mouse_position().1 / 16.0) as i32;

        let mut path_option: Option<Vec<(i32, i32)>> =  None;

        let width =  self.manor.stages[stage_id].width;
        let height = self.manor.stages[stage_id].height;
        if inside_rect(mouse_x, mouse_y, 0, 0, width-1, height-1) && self.manor.stages[stage_id].stage_map[(mouse_x + mouse_y * width) as usize].crossable && self.manor.stages[stage_id].stage_map[(mouse_x + mouse_y * width) as usize].visited {
            path_option = path_finder(self.hero.x, self.hero.y, mouse_x, mouse_y, &self.manor.stages[stage_id].stage_map, width, height);
            
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
        self.terminal.render();
    }
}
