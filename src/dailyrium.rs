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

pub struct Level {
	width: i32,
	height: i32,
	
}

#[derive(Eq,PartialEq, Hash, Copy, Debug, Clone)]
pub enum Property {
	Crossable,
	SeeThrought,
}


#[derive(Eq,PartialEq, Hash, Copy, Debug, Clone)]
pub enum PropertyValue {
	Bool(bool),
	Int(i32),
}