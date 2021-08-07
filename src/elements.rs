use crate::dailyrium::Sprite;
use crate::engine::give_id;
use tetra::graphics::Color;

pub mod items;
pub mod buildings;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ElementType {
	Floor,
	Wall,
}

#[derive(Clone)]
pub struct Element {
	pub id: u32,
	pub x: i32,
	pub y: i32,
	pub sprite: Sprite,
	pub nature: ElementType,
	pub name: String,
	pub description: String,

	// Some over used properties
	pub visited: bool,
	pub seen: bool,
	pub crossable: bool,
	pub see_through: bool,

	pub data: u64,

}

impl Element {
	pub fn new(x: i32, y: i32, t: ElementType) -> Element {
		let g = Sprite {
			glyph: '.' as u16,
			fg_color: Color::rgba8(255,255,255, 255),
			bg_color: Color::rgba8(0,0,0, 255),
			fade: 0.0
		};
		let element = Element{
			id: give_id(),
			x,
			y,
			sprite: g,
			nature: t,
			name: "".to_string(),
			description: "".to_string(),

			visited: false,
			seen: false,
			crossable: true,
			see_through: true,

			data: 0,
		};

		element
	}
	//fn get_flag(f: u8, flag: Flags) -> bool {
	//	let val = f & (1 << flag as u8);
	//	match val >> flag as u8 {
	//		1 => true,
	//		_ => false,
	//	}
	//}

	//fn set_flag(f: u8, flag: Flags) -> u8 {
	//	f | (1 << flag as u8)
	//}

	//fn clear_flag(f: u8, flag: Flags) -> u8 {
	//	f & !(1 << flag as u8)
	//}

}



