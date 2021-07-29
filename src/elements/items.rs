use crate::dailyrium::Sprite;
use crate::elements::{Element, ElementType};
use crate::living_entities::LivingEntity;
use crate::engine::give_id;
use tetra::graphics::Color;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ItemType {
	None,
	Gold,
}

#[derive(Clone)]
pub struct Item {
	pub id: u32,
	pub x: i32,
	pub y: i32,
	pub sprite: Sprite,
	pub nature: ItemType,
	pub name: String,
	pub description: String,

	// Some over used properties
	pub visited: bool,
	pub seen: bool,
	pub crossable: bool,
	pub see_through: bool,

	pub data: u64,
}

impl Item {
	pub fn new(xo: i32, yo: i32, item: ItemType) -> Item {
		let g = Sprite {
			glyph: '.' as u16,
			fg_color: Color::rgba8(255,255,255, 255),
			bg_color: Color::rgba8(0,0,0, 255),
		};

		Item {
			id: give_id(),
			x: xo,
			y: yo,
			sprite: g,
			nature: item,
			name: "".to_string(),
			description: "".to_string(),

			visited: false,
			seen: false,
			crossable: true,
			see_through: true,

			data: 0,
		}
	}
}
pub fn create_gold(x: i32, y: i32, amount: i32) -> Item {
	let mut gold = Item::new(x, y, ItemType::Gold);
	gold.sprite.glyph = '$' as u16;
	gold.sprite.fg_color = Color::rgba8(235, 230, 25, 255);
	gold.sprite.bg_color = Color::rgba8(0,0,0, 255);

	gold.name = "gold".to_string();
	gold.data = amount as u64;
	gold.description = format!("{} gold", amount);

	gold

}



