
use tetra::input::{self, Key};
use tetra::{time, Context, ContextBuilder, State};
use tetra::graphics::Color;

mod dailyrium;
mod elements;
mod engine;
mod living_entities;
mod rl_tetra;
mod world_factory;

use dailyrium::Action;
use engine::puppet_master;
use rl_tetra::Terminal;
use world_factory::Level;

const HEIGHT: i32 = 45;
const WIDTH: i32 = 80;
const CELL_SIZE: i32 = 16;


struct GameState {
    terminal: Terminal,
    level: Level,
    play_log: Vec<String>,
    interactive_panel: bool,
    player_turn: bool,
    turn_count: u32,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut log = Vec::new();
        log.push("Welcome to Dailyrium !".to_string());
        Ok(GameState {
            terminal: Terminal::new(ctx, WIDTH, HEIGHT, CELL_SIZE, CELL_SIZE),
            level: Level::new(WIDTH, HEIGHT),
            play_log: log,
            interactive_panel: false,
            player_turn: false,
            turn_count: 0,
        })
    }
}

impl State for GameState {

    /* Need to refactor fading effect with mul Color function */


    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        self.terminal.clear(ctx);
        for element in self.level.level_map.iter_mut() {
            if element.seen {

                // fade in effect
                if element.sprite.fg_color.a < 1.0 {
                    element.sprite.fg_color.a += 0.05;
                    element.sprite.bg_color.a += 0.05;

                }
                else {
                    element.sprite.fg_color.a = 1.0;
                    element.sprite.bg_color.a = 1.0;
                }
                self.terminal.put_ext(
                    element.x,
                    element.y,
                    element.sprite.glyph,
                    element.sprite.fg_color,
                    element.sprite.bg_color,
                );

            } else if element.visited {

                // fade out effect
                if element.sprite.fg_color.a > 0.4 {
                    element.sprite.fg_color.a -= 0.02;
                    element.sprite.bg_color.a -= 0.02;
                } else if !self.player_turn {
                    element.sprite.fg_color.a -= 0.02;
                    element.sprite.bg_color.a -= 0.02;

                }

                if element.sprite.fg_color.a < 0.05 {
                    element.visited = false;
                }
                self.terminal.put_ext(
                    element.x,
                    element.y,
                    element.sprite.glyph,
                    element.sprite.fg_color,
                    element.sprite.bg_color,
                );

            }
        }

        // Need to be improved... Perhaps i will put it in the engine puppet master function
        for seen in self.level.in_fov.iter() {
            for item in self.level.items.iter() {
                if item.x == seen.0 && item.y == seen.1 {
                    self.terminal.put_ext(
                        item.x,
                        item.y,
                        item.sprite.glyph,
                        item.sprite.fg_color,
                        item.sprite.bg_color,
                    );
                }
            }
        }

        for entity in self.level.entities.iter_mut() {
            if entity.seen {
                if entity.sprite.fg_color.a < 1.0 {
                    entity.sprite.fg_color.a += 0.05;
                    entity.sprite.bg_color.a += 0.05;

                }
                else {
                    entity.sprite.fg_color.a = 1.0;
                    entity.sprite.bg_color.a = 1.0;
                }
                self.terminal.put_ext(
                    entity.x,
                    entity.y,
                    entity.sprite.glyph,
                    entity.sprite.fg_color,
                    entity.sprite.bg_color,
                );
            }
            else {
                if entity.sprite.fg_color.a > 0.0 {
                    entity.sprite.fg_color.a -= 0.05;
                    entity.sprite.bg_color.a -= 0.05;
                    self.terminal.put_ext(
                        entity.x,
                        entity.y,
                        entity.sprite.glyph,
                        entity.sprite.fg_color,
                        entity.sprite.bg_color,
                    );
                }
                else {
                    entity.sprite.fg_color.a = 0.0; 
                    entity.sprite.bg_color.a = 0.0; 
                }
            }

        }

        self.terminal.print(0, 0, format!("Turn: {}", self.turn_count));
        self.terminal.print(0, 1, format!("FPS: {}", time::get_fps(ctx) as i32));


        // UI display
        if self.interactive_panel {
            self.terminal.bg_color(Color::rgb8(200,0,100));
            self.terminal.print(40, 22, "                    ".to_string());
            self.terminal.print(40, 23, "                    ".to_string());
            self.terminal.print(40, 24, "  Interactive Panel ".to_string());
            self.terminal.print(40, 25, "                    ".to_string());
            self.terminal.print(40, 26, "                    ".to_string());
            self.terminal.bg_color(Color::rgba8(0,0,0, 255));
        }



        self.terminal.refresh(ctx);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Need to be clarified (event management)
        if self.player_turn && !self.interactive_panel {
            if input::is_key_pressed(ctx, Key::Up) {

                self.level.entities[0].action = Action::Move(0, -1);
                self.player_turn = false;
            }
            else if input::is_key_pressed(ctx, Key::Down) {
                self.level.entities[0].action = Action::Move(0, 1);
                self.player_turn = false;

            }
            else if input::is_key_pressed(ctx, Key::Left) {
                self.level.entities[0].action = Action::Move(-1, 0);
                self.player_turn = false;

            }
            else if input::is_key_pressed(ctx, Key::Right) {
                self.level.entities[0].action = Action::Move(1, 0);
                self.player_turn = false;

            }
            else if input::is_key_pressed(ctx, Key::P) {
                self.level.entities[0].action = Action::Pick;
                self.player_turn = false
            }
            else if input::is_key_pressed(ctx, Key::I) {
                self.interactive_panel = true;
            }
        }
        else if self.interactive_panel {
            if input::is_key_pressed(ctx, Key::I) {
                self.interactive_panel = false;
            }

        }
        else if !self.player_turn {
            puppet_master(&mut self.level, &mut self.play_log);
            self.player_turn = true;
            self.turn_count += 1;
        }

        Ok(())
    }

}
fn main() -> tetra::Result {
    ContextBuilder::new("Dailyrium", WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
