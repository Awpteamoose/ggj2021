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
	pub drawers: HashMap<(u8, u8), Handle<ColorMaterial>>,
}

#[derive(Clone)]
pub struct ItemTemplate {
	pub size: (u8, u8),
	pub material: Handle<ColorMaterial>,
	pub description: &'static str,
}

pub struct ItemTemplates(pub Vec<ItemTemplate>);

impl ItemTemplates {
	pub fn get_random(&self) -> &ItemTemplate {
		&self.0[thread_rng().gen_range(0..self.0.len())]
	}
}

pub struct ItemRequest(pub &'static str, pub bool, pub usize);
