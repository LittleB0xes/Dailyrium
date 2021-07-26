use rand::prelude::*;
use crate::dailyrium::{Property, PropertyValue};
use crate::elements::{Element, ElementType};
use crate::elements::items::create_gold;
use crate::living_entity::{LivingEntity, EntityType};


pub struct Level {
	pub width: i32,
	pub height: i32,
	pub level_map: Vec<Element>,
	pub entities: Vec<LivingEntity>,
	pub items: Vec<Element>
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
		}
	}
}

pub fn random_test_world(w: i32, h: i32) -> Vec<Element> {
	let mut rng = rand::thread_rng();
	let mut wmap: Vec<Element> = Vec::new();
	for i in 0..(w*h) {
		let alea = rng.gen_range(0..100);
		let element: ElementType;
		if alea < 10 {
			element = ElementType::Wall;
		}
		else {
			element = ElementType::Floor;
		}

		wmap.push(Element::new( i % w, i / w, element));

	}

	wmap
}

pub fn random_items_spawn(wmap: &Vec<Element>, width: i32, height: i32) -> Vec<Element> {
	let mut rng = rand::thread_rng();
	let mut items: Vec<Element> = Vec::new();
	let max_of_items = 20;

	for _i in 0..max_of_items {
		let x = rng.gen_range(0..width);
		let y = rng.gen_range(0..height);
		let amount = rng.gen_range(0..100);
		if wmap[(x + y * width) as usize].have_property(Property::Crossable) == PropertyValue::Bool(true) {
			items.push(create_gold(x, y, amount));
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
		npc.push(LivingEntity::new(x, y, EntityType::Zombie));
	}

	npc
}