use crate::prelude::*;

#[derive(Default)]
pub struct Drawer {
	pub material: Handle<ColorMaterial>,
	pub size: (u8, u8),
	pub items: HashMap<(u8, u8), Entity>,
}

impl Drawer {
	#[must_use]
	pub fn can_fill(&self, item_query: &Query<&cmp::Item>, size: (u8, u8)) -> Option<(u8, u8)> {
		let mut fill_map = vec![vec![false; self.size.1 as usize]; self.size.0 as usize];
		for ((x, y), item) in &self.items {
			let item = item_query.get(*item).unwrap();

			for w in 0..item.size.0 {
				for h in 0..item.size.1 {
					fill_map[(x + w) as usize][(y + h) as usize] = true;
				}
			}
		}

		let test_fill_map = |x, y| -> bool {
			for w in 0..size.0 {
				for h in 0..size.1 {
					if fill_map.get((x + w) as usize).cloned().unwrap_or_default().get((y + h) as usize).cloned().unwrap_or(true) { return false; }
				}
			}

			true
		};

		for y in 0..self.size.1 {
			for x in 0..self.size.0 {
				if test_fill_map(x, y) { return Some((x, y)) }
			}
		}

		None
	}
}
