use crate::prelude::*;

#[derive(Inspectable, Default)]
pub struct Tweak {
	tweak: bool,
	size: f32,
	#[inspectable(min = vec2(-1., -1.), max = vec2(1., 1.))]
	pos: Vec2,
}

#[system]
pub fn debug(
	mut query: Query<&mut Text>,
	time: Res<Time>,
	mut things: Query<(&cmp::Drawer, &mut Transform, &GlobalTransform, &Sprite, &mut Handle<ColorMaterial>)>,
	items: Query<&cmp::Item>,
	tweak: Res<Tweak>,
	cursor_pos: Res<res::CursorPos>,
	materials: Res<res::Materials>,
	btns: Res<Input<MouseButton>>,
) {
	for mut text in query.iter_mut() {
		text.value = time.delta_seconds().to_string();
	}

	if tweak.tweak {
		let mut one_clicked = false;
		for (drawer, mut transform, global_transform, sprite, mut material) in things.iter_mut() {
			// transform.translation = Vec3::new(tweak.pos.x, tweak.pos.y, transform.translation.z);
			// transform.scale = Vec3::new(tweak.size, tweak.size, tweak.size);
			// dbg!(bevy::sprite::collide_aabb::collide(Vec3::new(cursor_pos.x, cursor_pos.y, transform.translation.z), Vec2::zero(), transform.translation, sprite.size));
			// dbg!(collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size));
			if btns.just_pressed(MouseButton::Left){
				if collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size) && !one_clicked {
					*material = materials.black.clone();
					one_clicked = true;
				} else {
					*material = drawer.material.clone();
				}
			}

			if btns.just_pressed(MouseButton::Right){
				if collide(*cursor_pos.as_ref(), global_transform.translation.xy(), sprite.size) && !one_clicked {
					dbg!(drawer.can_fill(&items, (2, 2)));
					one_clicked = true;
				}
			}
		}
	}
}
