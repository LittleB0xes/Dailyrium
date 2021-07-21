//use rand::prelude::*;
//use tetra::graphics::Color;
use tetra::input::{self, Key};
use tetra::Event::KeyPressed;
//use tetra::math::Vec2;
use tetra::{time, Context, ContextBuilder, Event, State};

use rand::prelude::*;

mod rltetra;
mod dailyrium;
mod engine;
mod living_entity;
mod elements;
mod world_factory;

use living_entity::LivingEntity;
use dailyrium::{Action, EntityType};
use engine::{action_manager, brain};
use world_factory::random_test_world;
use rltetra::Terminal;
use elements::Element;

const HEIGHT: i32 = 50;
const WIDTH: i32 = 80;
const CELL_SIZE: i32 = 16;

struct GameState {
	terminal: Terminal,
	player: LivingEntity,
	npc_list: Vec<LivingEntity>,
	level_map: Vec<Element>,
	player_turn: bool,
	turn_count: u32,
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let mut npc: Vec<LivingEntity> = Vec::new();
		
		let mut rng = rand::thread_rng();
		for i in 0..10 {
			let x = rng.gen_range(0..WIDTH);
			let y = rng.gen_range(0..HEIGHT);
			npc.push(LivingEntity::new(x, y, EntityType::Zombie));
		}
		Ok(GameState{

			terminal: Terminal::new(ctx, WIDTH, HEIGHT, CELL_SIZE, CELL_SIZE),
			player: LivingEntity::new(10, 10, EntityType::Hero ),
			npc_list: npc,
			level_map: random_test_world(WIDTH, HEIGHT),
			player_turn: true,
			turn_count: 0,
		})

	}
}

impl State for GameState {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {

		self.terminal.clear(ctx);
		for element in self.level_map.iter() {
			self.terminal.fg_color(element.sprite.fg_color);
			self.terminal.bg_color(element.sprite.bg_color);
			self.terminal.put(element.x, element.y, element.sprite.glyph);
		}
		for npc in self.npc_list.iter() {
			self.terminal.fg_color(npc.sprite.fg_color);
			self.terminal.bg_color(npc.sprite.bg_color);
			self.terminal.put(npc.x, npc.y, npc.sprite.glyph);
		}
		self.terminal.fg_color(self.player.sprite.fg_color);
		self.terminal.bg_color(self.player.sprite.bg_color);
		self.terminal.put(self.player.x, self.player.y, self.player.sprite.glyph);
		self.terminal.print(0, 0, format!("Trun: {}", self.turn_count));
		self.terminal.refresh(ctx);
		Ok(())

	}

	fn update(&mut self, ctx: &mut Context) -> tetra::Result{
		if self.player_turn {
			let action_finish = action_manager(&mut self.player, &mut self.level_map, WIDTH, HEIGHT);

			if action_finish {
				self.turn_count += 1;
				self.player_turn = false;
			}
		}
		else {
			// NPC turn
			for npc in self.npc_list.iter_mut() {
				brain(npc);
			}
			for npc in self.npc_list.iter_mut() {
				action_manager(npc, &mut self.level_map, WIDTH, HEIGHT);
			}
			self.player_turn = true;
		}


		Ok(())
	}

	fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
		//println!("Event: {:?}", event);
		match event {
			KeyPressed{key} => {
				if self.player_turn {
					match key {
						Key::Up => {self.player.action = Action::Move(0,-1);},
						Key::Down => {self.player.action = Action::Move(0,1);},
						Key::Left => {self.player.action = Action::Move(-1,0);},
						Key::Right => {self.player.action = Action::Move(1,0);},
						_ =>{}
					}
				}
				//println!("Key: {:?}", key);
			},
			_ => {},
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
