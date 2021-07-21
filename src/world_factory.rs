use rand::prelude::*;
use crate::elements::Element;
use crate::dailyrium::ElementType;

pub fn random_test_world(w: i32, h: i32) -> Vec<Element> {
	let mut rng = rand::thread_rng();
	let mut wmap: Vec<Element> = Vec::new();
	for i in 0..(w*h) {
		let alea = rng.gen_range(0..100);
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