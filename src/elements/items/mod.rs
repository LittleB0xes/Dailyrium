use crate::elements::{Element, ElementType};
use tetra::graphics::Color;
use crate::dailyrium::{Property, PropertyValue};


pub fn create_gold(x: i32, y: i32, amount: i32) -> Element {
	let mut gold = Element::new(x, y, ElementType::Gold);
	gold.sprite.glyph = '$' as u16;
	gold.sprite.fg_color = Color::rgba8(235, 230, 25, 255);
	gold.sprite.bg_color = Color::rgba8(0,0,0, 255);

	gold.properties.insert(Property::Amount, PropertyValue::Int(amount));

	gold.description = format!("{} gold", amount);

	gold

}
