use crate::dailyrium::{Sprite, Property, PropertyValue};
use crate::engine::give_id;
use tetra::graphics::Color;

use std::collections::HashMap;

pub mod items;
pub mod buildings;


#[derive(Copy, Clone)]
pub enum ElementType {
	Floor,
	Wall,
	Gold,
}

pub struct Element {
	pub id: u32,
	pub x: i32,
	pub y: i32,
	pub sprite: Sprite,
	pub nature: ElementType,
	pub properties: HashMap<Property, PropertyValue>,
	pub name: String,
	pub description: String,
}

impl Element {
	pub fn new(x: i32, y: i32, t: ElementType) -> Element {
		let g = Sprite {
			glyph: '.' as u16,
			fg_color: Color::rgba8(255,255,255, 255),
			bg_color: Color::rgba8(0,0,0, 255),
		};
		let element = Element{
			id: give_id(),
			x,
			y,
			sprite: g,
			nature: t,
			properties: HashMap::new(),
			name: "".to_string(),
			description: "".to_string(),
		};

		element
	}



	pub fn have_property(&self, property: Property) -> PropertyValue {
		//self.properties.iter().any(|&c| c == property)
		match self.properties.get(&property) {
			Some(&property) => property,

			_ => PropertyValue::Bool(false)

		}

	}
}



