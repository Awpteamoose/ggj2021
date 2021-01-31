use crate::prelude::*;

#[system]
pub fn system(
	mut drawers: Query<(Entity, &mut cmp::Drawer)>,
	cursor_pos: Res<res::CursorPos>,
	btns: Res<Input<MouseButton>>,
	selected_drawer: Res<res::SelectedDrawer>,
	sprites: Query<(Entity, &GlobalTransform, &Sprite)>,
	mut visibles: Query<&mut Visible>,
	mut selected_item: ResMut<res::SelectedItem>,
	items: Query<&cmp::Item>,
	mut transforms: Query<&mut Transform>,
	commands: &mut Commands,
	mut lost_items: ResMut<res::LostItems>,
) {
	if selected_item.0.is_none() && btns.just_pressed(MouseButton::Left) {
		if let Some(&last_lost_item) = lost_items.1.last() {
			let (_, global_transform, sprite) = sprites.get(last_lost_item).unwrap();
			if collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size) {
				lost_items.1.pop();
				selected_item.0.replace(last_lost_item);
				let _ = selected_item.1.take();
				commands.remove_one::<Parent>(last_lost_item);
			}
		}

		if let Some(selected_drawer_entity) = selected_drawer.0 {
			let (_, mut selected_drawer) = drawers.get_mut(selected_drawer_entity).unwrap();
			let (_, global_transform, sprite) = sprites.get(selected_drawer_entity).unwrap();
			if collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size) {
				for (&coords, &item) in &selected_drawer.items {
					let (entity, global_transform, sprite) = sprites.get(item).unwrap();
					if collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size) {
						selected_item.0.replace(entity);
						selected_item.1.replace((selected_drawer_entity, coords));
						selected_drawer.items.remove(&coords);
						commands.remove_one::<Parent>(entity);
						break;
					}
				}
			}
		}
	}

	if let Some(selected_item_entity) = selected_item.0 {
		if btns.pressed(MouseButton::Left) {
			let mut transform = transforms.get_mut(selected_item_entity).unwrap();
			transform.translation = vec3(cursor_pos.x, cursor_pos.y, 0.);
		} else if btns.just_released(MouseButton::Left) {
			let item = items.get(selected_item_entity).unwrap();

			let mut transferred = false;
			for (entity, mut drawer) in drawers.iter_mut() {
				if let Some(selected_drawer_entity) = selected_drawer.0 {
					if selected_drawer_entity == entity { continue; }
				}

				let (_, global_transform, sprite) = sprites.get(entity).unwrap();
				if collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size) {
					if let Some(coords) = drawer.can_fill(&items, item.size) {
						drawer.items.insert(coords, selected_item_entity);
						let mut transform = transforms.get_mut(selected_item_entity).unwrap();
						*transform = item.transform(coords);
						commands.insert_one(selected_item_entity, Parent(drawer.root));
						let _ = selected_item.0.take();
						let _ = selected_item.1.take();
						transferred = true;
						visibles.get_mut(selected_item_entity).unwrap().is_visible = false;
						break;
					}
				}
			}

			if collide(*cursor_pos.as_ref(), vec2(0., -0.150625), vec2(0.18, 0.135)) {
				println!("TRY GIVE");
				commands.despawn_recursive(selected_item_entity);
				transferred = true;
			}

			if !transferred {
				if let Some((old_parent, coords)) = selected_item.1 {
					let (_, mut drawer) = drawers.get_mut(old_parent).unwrap();
					drawer.items.insert(coords, selected_item_entity);
					let mut transform = transforms.get_mut(selected_item_entity).unwrap();
					*transform = item.transform(coords);
					commands.insert_one(selected_item_entity, Parent(drawer.root));
				} else {
					lost_items.1.push(selected_item_entity);
					commands.insert_one(selected_item_entity, Parent(lost_items.0));
					let mut transform = transforms.get_mut(selected_item_entity).unwrap();
					*transform = item.transform((0, 0));
				}

			}

			let _ = selected_item.0.take();
			let _ = selected_item.1.take();
		}
	}
}
