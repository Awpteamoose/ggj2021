use crate::prelude::*;

#[derive(Default, Reflect, Shrinkwrap)]
pub struct CursorPos(pub Vec2);

#[derive(Default, Reflect, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct SelectedDrawer(pub Option<Entity>);

#[derive(Default)]
pub struct SelectedItem(pub Option<Entity>, pub Option<(Entity, (u8, u8))>);

pub struct LostItems(pub Entity, pub Vec<Entity>);

pub struct Materials {
	pub black: Handle<ColorMaterial>,
	pub book: Handle<ColorMaterial>,
	pub hat: Handle<ColorMaterial>,
	pub ring: Handle<ColorMaterial>,
	pub umbrella: Handle<ColorMaterial>,

	pub drawers: HashMap<(u8, u8), Handle<ColorMaterial>>,
}
