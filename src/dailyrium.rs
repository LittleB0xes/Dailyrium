use tetra::graphics::Color;


#[derive(Copy, Clone)]
pub enum Action {
    Move(i32, i32),
    Waiting,
	Pick,
}

#[derive(Copy, Clone)]
pub struct Sprite {
	pub glyph: u16,
	pub fg_color: Color,
	pub bg_color: Color
}


#[derive(Eq,PartialEq, Hash, Copy, Debug, Clone)]
pub enum Property {
	Crossable,
	SeeThrought,
	Amount,
}


#[derive(Eq,PartialEq, Hash, Copy, Debug, Clone)]
pub enum PropertyValue {
	Bool(bool),
	Int(i32),
}