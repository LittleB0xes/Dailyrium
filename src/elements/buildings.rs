use super::{Element, ElementType};
use tetra::graphics::Color;



pub fn create_floor(x: i32, y: i32) -> Element {
	let mut floor = Element::new(x, y, ElementType::Floor);
	floor.sprite.glyph = '.' as u16;
	floor.sprite.fg_color = Color::rgba8(125, 100, 125,0);
	floor
}
pub fn create_wall(x: i32, y: i32) -> Element {
	let mut wall = Element::new(x, y, ElementType::Wall);
	wall.sprite.glyph = '#' as u16;
	wall.sprite.fg_color = Color::rgba8(25, 10, 25, 0);
	wall.sprite.bg_color = Color::rgba8(125, 100, 125, 0);
	wall.crossable = false;
	wall.see_through = false;

	wall
}