use crate::dailyrium::{Sprite, Property, PropertyValue};
use crate::engine::give_id;
use tetra::graphics::Color;

use std::collections::HashMap;


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
	//pub properties: Vec<Property>,
}

impl Element {
	pub fn new(x: i32, y: i32, t: ElementType) -> Element {
		let g = Sprite {
			glyph: '.' as u16,
			fg_color: Color::rgba8(255,255,255, 255),
			bg_color: Color::rgba8(0,0,0, 255),
		};
		let mut element = Element{
			id: give_id(),
			x,
			y,
			sprite: g,
			nature: t,
			properties: HashMap::new(),
			name: "".to_string(),
			description: "".to_string(),
		};

		match t {
			ElementType::Floor 	=> element.to_floor(),
			ElementType::Wall 	=> element.to_wall(),
			ElementType::Gold	=> element.to_gold(),
			
		}

		element
	}

	fn to_floor(&mut self) {
		self.sprite.glyph = '.' as u16;
		self.sprite.fg_color = Color::rgba8(125, 100, 125, 255);
		self.properties.insert(Property::Crossable, PropertyValue::Bool(true));
		self.properties.insert(Property::SeeThrought, PropertyValue::Bool(true));
	}
	fn to_wall(&mut self) {
		self.sprite.glyph = '#' as u16;
		self.sprite.fg_color = Color::rgba8(25, 10, 25, 255);
		self.sprite.bg_color = Color::rgba8(125, 100, 125, 255);
	}

	fn to_gold(&mut self) {
		self.sprite.glyph = '$' as u16;
		self.sprite.fg_color = Color::rgba8(235, 230, 25, 255);
		self.sprite.bg_color = Color::rgba8(0,0,0, 255);
		self.description = "some gold".to_string();
	}

	pub fn have_property(&self, property: Property) -> PropertyValue {
		//self.properties.iter().any(|&c| c == property)
		match self.properties.get(&property) {
			Some(&property) => property,

			_ => PropertyValue::Bool(false)

		}

	}
}



