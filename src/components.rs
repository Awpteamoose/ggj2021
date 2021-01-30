mod drawer;

pub use drawer::Drawer;

use crate::prelude::*;

#[derive(Reflect)]
pub struct MainCamera;

#[derive(Reflect)]
pub struct Name(String);

pub struct Item {
	pub material: Handle<ColorMaterial>,
	pub size: (u8, u8),
}

