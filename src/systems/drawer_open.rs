use crate::prelude::*;

#[system]
pub fn system(
	mut drawers: Query<(Entity, &cmp::Drawer, &GlobalTransform, &Sprite, &mut Handle<ColorMaterial>)>,
	mut visibles: Query<&mut Visible>,
	cursor_pos: Res<res::CursorPos>,
	materials: Res<res::Materials>,
	btns: Res<Input<MouseButton>>,
	mut selected_drawer: ResMut<res::SelectedDrawer>,
) {
	if btns.just_pressed(MouseButton::Left) {
		let mut one_clicked = false;

		for (entity, drawer, global_transform, sprite, mut material) in drawers.iter_mut() {
			if collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size) && !one_clicked {
				*material = materials.black.clone();
				one_clicked = true;
				for item in drawer.items.values() {
					visibles.get_mut(*item).unwrap().is_visible = true;
				}
				selected_drawer.replace(entity);
			} else {
				*material = materials.drawers[&drawer.size].clone();
				for item in drawer.items.values() {
					visibles.get_mut(*item).unwrap().is_visible = false;
				}
			}
		}

		if !one_clicked {
			let _ = selected_drawer.take();
		}
	}
}
