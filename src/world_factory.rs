use bracket_lib::prelude::RandomNumberGenerator;

use crate::elements::Element;
use crate::dailyrium::ElementType;

pub fn random_test_world(w: i32, h: i32) -> Vec<Element> {
	let mut rng = RandomNumberGenerator::new();
	let mut wmap: Vec<Element> = Vec::new();
	for i in 0..(w*h) {
		let alea = rng.range::<u8>(0, 100);
		let element: ElementType;
		if alea < 10 {
			element = ElementType::Wall;
		}
		else {
			element = ElementType::Floor;
		}

		wmap.push(Element::new( i % w, i / w, element));

	}

	wmap
}