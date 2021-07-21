//use rand::prelude::*;
//use tetra::graphics::Color;
use tetra::input::{self, Key};
//use tetra::math::Vec2;
use tetra::{time, Context, ContextBuilder, Event, State};

mod rltetra;
mod dailyrium;
mod engine;
mod living_entity;

use living_entity::LivingEntity;
use dailyrium::Action;
use engine::action_manager;

use rltetra::Terminal;

const HEIGHT: i32 = 50;
const WIDTH: i32 = 80;
const CELL_SIZE: i32 = 16;

struct GameState {
	terminal: Terminal,
	player: LivingEntity,
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		Ok(GameState{
			terminal: Terminal::new(ctx, WIDTH, HEIGHT, CELL_SIZE, CELL_SIZE),
			player: LivingEntity::new(10, 10),
		})

	}
}

impl State for GameState {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		action_manager(&mut self.player);

		self.terminal.clear(ctx);
		self.terminal.put(self.player.x, self.player.y, self.player.sprite.glyph);
		self.terminal.refresh(ctx);
		Ok(())

	}

	fn update(&mut self, ctx: &mut Context) -> tetra::Result{

		if input::is_key_pressed(ctx, Key::Up) {
			self.player.action = Action::Move(0,-1);
		}
		else if input::is_key_pressed(ctx, Key::Down) {
			self.player.action = Action::Move(0,1);
		}
		else if input::is_key_pressed(ctx, Key::Left) {
			self.player.action = Action::Move(-1,0);
		}
		else if input::is_key_pressed(ctx, Key::Right) {
			self.player.action = Action::Move(1,0);
		}
		Ok(())
	}

}
fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
