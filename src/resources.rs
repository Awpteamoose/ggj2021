use crate::prelude::*;

#[derive(Default, Reflect, Shrinkwrap)]
pub struct CursorPos(pub Vec2);

pub struct Materials {
	pub black: Handle<ColorMaterial>,
	pub book: Handle<ColorMaterial>,
	pub hat: Handle<ColorMaterial>,
	pub ring: Handle<ColorMaterial>,
	pub umbrella: Handle<ColorMaterial>,

	pub drawers: HashMap<(u8, u8), Handle<ColorMaterial>>,

	// pub drawer1x1: Handle<ColorMaterial>,
	// pub drawer1x2: Handle<ColorMaterial>,
	// pub drawer1x3: Handle<ColorMaterial>,
	// pub drawer1x4: Handle<ColorMaterial>,
	// pub drawer2x1: Handle<ColorMaterial>,
	// pub drawer2x2: Handle<ColorMaterial>,
	// pub drawer2x3: Handle<ColorMaterial>,
	// pub drawer3x1: Handle<ColorMaterial>,
	// pub drawer3x2: Handle<ColorMaterial>,
	// pub drawer3x3: Handle<ColorMaterial>,
	// pub drawer4x1: Handle<ColorMaterial>,
}
