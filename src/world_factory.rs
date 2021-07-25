use rand::prelude::*;
use crate::dailyrium::{Property, PropertyValue};
use crate::elements::{Element, ElementType};


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
		if wmap[(x + y * width) as usize].have_property(Property::Crossable) == PropertyValue::Bool(true) {
			items.push(Element::new(x, y, ElementType::Gold));
		}
	}
	items
}