use rand::prelude::*;
use crate::elements::Element;
use crate::elements::items::Item;
use crate::elements::items;
use crate::elements::buildings::*;
use crate::living_entities::{LivingEntity, EntityType, Behavior};


pub struct Level {
	pub width: i32,
	pub height: i32,
	pub level_map: Vec<Element>,
	pub entities: Vec<LivingEntity>,
	pub items: Vec<Item>, 
	pub in_fov: Vec<(i32, i32)>
}

impl Level {
	pub fn new(w: i32, h: i32) -> Level {
		let wmap = random_test_world(w, h);
		let mut npc = random_monsters_spawn(&wmap, w, h);
		npc.insert(0, LivingEntity::new(10,10,EntityType::Hero));

		let items_list = random_items_spawn(&wmap, w, h);

		Level {
			width: w,
			height: h,
			level_map: wmap,
			entities: npc,
			items: items_list,
			in_fov: Vec::new()	
		}
	}
}

pub fn random_test_world(w: i32, h: i32) -> Vec<Element> {
	let mut rng = rand::thread_rng();
	let mut wmap: Vec<Element> = Vec::new();
	for i in 0..(w*h) {
		let alea = rng.gen_range(0..100);
		if alea < 10 {
			wmap.push(create_wall( i % w, i / w));
		}
		else {
			wmap.push(create_floor( i % w, i / w));
		}


	}

	wmap
}

pub fn random_items_spawn(wmap: &Vec<Element>, width: i32, height: i32) -> Vec<Item> {
	let mut rng = rand::thread_rng();
	let mut items: Vec<Item> = Vec::new();
	let max_of_items = 20;

	for _i in 0..max_of_items {
		let x = rng.gen_range(0..width);
		let y = rng.gen_range(0..height);
		let amount = rng.gen_range(0..100);
		if wmap[(x + y * width) as usize].crossable {
			let  gold = items::create_gold(x, y, amount);
			items.push(gold);
		}
	}
	items
}

pub fn random_monsters_spawn(_wmap: &Vec<Element>, width: i32, height: i32) -> Vec<LivingEntity> {
    let mut npc: Vec<LivingEntity> = Vec::new();
	let mut rng = rand::thread_rng();
	for _i in 0..10 {
		let x = rng.gen_range(0..width );
		let y = rng.gen_range(0..height);
		let state = rng.gen_range(0..100);
		let mut monster = LivingEntity::new(x, y, EntityType::Zombie);
		if state > 20 {
			monster.behavior = Behavior::Drunk;
		}
		else {
			monster.behavior = Behavior::Sleep;
		}
		npc.push(monster);
	}

	npc
}