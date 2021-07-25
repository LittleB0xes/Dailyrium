use tetra::graphics::Color;


pub enum Action {
    Move(i32, i32),
    Waiting,
	Pick,
}

pub struct Sprite {
	pub glyph: u16,
	pub fg_color: Color,
	pub bg_color: Color
}


#[derive(Eq,PartialEq, Hash, Copy, Debug, Clone)]
pub enum Property {
	Crossable,
	SeeThrought,
}


#[derive(Eq,PartialEq, Hash, Copy, Debug, Clone)]
pub enum PropertyValue {
	Bool(bool),
}