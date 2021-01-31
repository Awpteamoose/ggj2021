mod drawer;

pub use drawer::Drawer;

use crate::prelude::*;

#[derive(Reflect)]
pub struct MainCamera;

#[derive(Reflect)]
pub struct Name(String);

pub struct Item {
	pub size: (u8, u8),
}

impl Item {
	pub fn transform(&self, coords: (u8, u8)) -> Transform {
		let size = vec2(f32::from(self.size.0) * 0.05, f32::from(self.size.1) * 0.05);
		Transform::from_translation(vec3(size.x * 0.5 + f32::from(coords.0) / 20., -size.y * 0.5 - ((f32::from(coords.1) / 11.25) * 9./16.), 1.))
	}
}
