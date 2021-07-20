use bracket_lib::prelude::RGBA;

pub enum ElementType {
	Floor,
	Wall,
}

pub enum Action {
    MOVE(i32, i32),
    WAITING,
}

pub struct Sprite {
	pub glyph: u16,
	pub fg_color: RGBA,
	pub bg_color: RGBA
}