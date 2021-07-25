use tetra::input::Key;
use tetra::Event::KeyPressed;
//use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, Event, State};

mod dailyrium;
mod elements;
mod engine;
mod living_entity;
mod rltetra;
mod world_factory;

use dailyrium::Action;
use engine::{action_manager, brain};
use living_entity::EntityType;
use rltetra::Terminal;

use world_factory::Level;

const HEIGHT: i32 = 60;
const WIDTH: i32 = 100;
const UI_WIDTH: i32 = 20;
const CELL_SIZE: i32 = 16;

const UI_XOFFSET: i32 = WIDTH - UI_WIDTH;



struct GameState {
    terminal: Terminal,
    level: Level,
    play_log: Vec<String>,
    player_turn: bool,
    turn_count: u32,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut log = Vec::new();
        log.push("Welcome in Dailyrium !".to_string());
        Ok(GameState {
            terminal: Terminal::new(ctx, WIDTH, HEIGHT, CELL_SIZE, CELL_SIZE),
            level: Level::new(WIDTH - UI_WIDTH, HEIGHT),
            play_log: log,
            player_turn: true,
            turn_count: 0,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        self.terminal.clear(ctx);
        for element in self.level.level_map.iter() {
            self.terminal.put_ext(
                element.x,
                element.y,
                element.sprite.glyph,
                element.sprite.fg_color,
                element.sprite.bg_color,
            );
        }
        for item in self.level.items.iter() {
            self.terminal.put_ext(
                item.x,
                item.y,
                item.sprite.glyph,
                item.sprite.fg_color,
                item.sprite.bg_color,
            );
        }
        for entity in self.level.entities.iter() {
            self.terminal.put_ext(
                entity.x,
                entity.y,
                entity.sprite.glyph,
                entity.sprite.fg_color,
                entity.sprite.bg_color,
            );
        }

        self.terminal.print(UI_XOFFSET, 0, format!("Turn: {}", self.turn_count));
        self.terminal.print(20, 0, format!("{}", self.play_log[0]));
        self.terminal.refresh(ctx);
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        // Need to be clarified (event management)
        if !self.player_turn {
            // NPC turn
            for entity in self.level.entities.iter_mut() {
                if entity.nature != EntityType::Hero {
                    brain(entity);
                }
            }

            // Resolve actions for all entities
            for entity in self.level.entities.iter_mut() {
                action_manager(
                    entity,
                    &mut self.level.items,
                    &mut self.play_log,
                    &mut self.level.level_map,
                    self.level.width,
                    self.level.height);
            }
            self.player_turn = true;
            self.turn_count += 1;
        }

        Ok(())
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
        //println!("Event: {:?}", event);
        match event {
            KeyPressed { key } => {
                if self.player_turn {
                    match key {
                        Key::Up => {
                            self.level.entities[0].action = Action::Move(0, -1);
                            self.player_turn = false;
                        }
                        Key::Down => {
                            self.level.entities[0].action = Action::Move(0, 1);
                            self.player_turn = false;
                        }
                        Key::Left => {
                            self.level.entities[0].action = Action::Move(-1, 0);
                            self.player_turn = false
                        }
                        Key::Right => {
                            self.level.entities[0].action = Action::Move(1, 0);
                            self.player_turn = false
                        }
                        Key::P => {
                            self.level.entities[0].action = Action::Pick;
                            self.player_turn = false
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
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
