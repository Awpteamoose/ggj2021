use crate::prelude::*;

#[system]
pub fn system(
	mut drawers: Query<(&cmp::Drawer, &GlobalTransform, &Sprite, &mut Handle<ColorMaterial>)>,
	// items: Query<&cmp::Item>,
	mut visibles: Query<&mut Visible>,
	cursor_pos: Res<res::CursorPos>,
	materials: Res<res::Materials>,
	btns: Res<Input<MouseButton>>,
) {
	if btns.just_pressed(MouseButton::Left) {
		let mut one_clicked = false;
		for (drawer, global_transform, sprite, mut material) in drawers.iter_mut() {
			if collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size) && !one_clicked {
				*material = materials.black.clone();
				one_clicked = true;
				for item in drawer.items.values() {
					visibles.get_mut(*item).unwrap().is_visible = true;
				}
			} else {
				*material = drawer.material.clone();
				for item in drawer.items.values() {
					visibles.get_mut(*item).unwrap().is_visible = false;
				}
			}
		}
	}
}
