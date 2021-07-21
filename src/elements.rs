use crate::dailyrium::{Sprite, ElementType, Propertie};
use crate::engine::give_id;
use tetra::graphics::Color;

pub struct Element {
	pub id: u32,
	pub x: i32,
	pub y: i32,
	pub sprite: Sprite,
	pub nature: ElementType,
	pub properties: Vec<Propertie>,
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
			properties: Vec::new(),
		};

		match t {
			ElementType::Floor 	=> element.to_floor(),
			ElementType::Wall 	=> element.to_wall(),
		}

		element
	}

	fn to_floor(&mut self) {
		self.sprite.glyph = '.' as u16;
		self.sprite.fg_color = Color::rgba8(125, 100, 125, 255);
		self.properties.push(Propertie::Crossable);
		self.properties.push(Propertie::SeeThrought);
	}
	fn to_wall(&mut self) {
		self.sprite.glyph = '#' as u16;
		self.sprite.fg_color = Color::rgba8(25, 10, 25, 255);
		self.sprite.bg_color = Color::rgba8(125, 100, 125, 255);
	}
}

