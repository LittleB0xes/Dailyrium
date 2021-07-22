use tetra::graphics::Color;

#[derive(Copy, Clone)]
pub enum ElementType {
	Floor,
	Wall,
}

#[derive(Eq, PartialEq)]
pub enum EntityType {
	Hero,
	Zombie,
}

pub enum Action {
    Move(i32, i32),
    Waiting,
}

pub struct Sprite {
	pub glyph: u16,
	pub fg_color: Color,
	pub bg_color: Color
}

#[derive(Eq, PartialEq, Copy, Debug, Clone)]
pub enum Property {
	Crossable,
	SeeThrought
}