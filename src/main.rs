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
	//player: LivingEntity,
	entities: Vec<LivingEntity>,
	level_map: Vec<Element>,
	player_turn: bool,
	turn_count: u32,
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let mut npc: Vec<LivingEntity> = Vec::new();
		npc.push(LivingEntity::new(10, 10, EntityType::Hero ));
		
		let mut rng = rand::thread_rng();
		for _i in 0..10 {
			let x = rng.gen_range(0..WIDTH);
			let y = rng.gen_range(0..HEIGHT);
			npc.push(LivingEntity::new(x, y, EntityType::Zombie));
		}
		Ok(GameState{

			terminal: Terminal::new(ctx, WIDTH, HEIGHT, CELL_SIZE, CELL_SIZE),
			//player: LivingEntity::new(10, 10, EntityType::Hero ),
			entities: npc,
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
			self.terminal.put_ext(
				element.x,
				element.y,
				element.sprite.glyph,
				element.sprite.fg_color,
				element.sprite.bg_color);
		}
		for entity in self.entities.iter() {
			self.terminal.put_ext(
				entity.x,
				entity.y,
				entity.sprite.glyph,
				entity.sprite.fg_color,
				entity.sprite.bg_color);
		}

		self.terminal.print(0, 0, format!("Turn: {}", self.turn_count));
		self.terminal.refresh(ctx);
		Ok(())

	}

	fn update(&mut self, ctx: &mut Context) -> tetra::Result{

		// Need to be clarified (event management)
		if !self.player_turn {
			// NPC turn
			for entity in self.entities.iter_mut() {
				if entity.nature != EntityType::Hero {
					brain(entity);
				}
			}

			// Resolve actions for all entities
			for entity in self.entities.iter_mut() {
				action_manager(entity, &mut self.level_map, WIDTH, HEIGHT);
			}
			self.player_turn = true;
			self.turn_count += 1;
		}


		Ok(())
	}

	fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
		//println!("Event: {:?}", event);
		match event {
			KeyPressed{key} => {
				if self.player_turn {
					match key {
						Key::Up =>    {self.entities[0].action = Action::Move(0,-1); self.player_turn = false;},
						Key::Down =>  {self.entities[0].action = Action::Move(0,1); self.player_turn = false;},
						Key::Left =>  {self.entities[0].action = Action::Move(-1,0); self.player_turn = false},
						Key::Right => {self.entities[0].action = Action::Move(1,0); self.player_turn = false},
						_ =>{}
					}
				}
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
