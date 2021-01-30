use crate::prelude::*;

#[system(stage::PRE_UPDATE)]
pub fn system(
	// events to get cursor position
	ev_cursor: Res<Events<CursorMoved>>,
	mut evr_cursor: Local<EventReader<CursorMoved>>,
	// need to get window dimensions
	wnds: Res<Windows>,
	// query to get camera transform
	q_camera: Query<&Transform, With<cmp::MainCamera>>,
	mut cursor_pos: ResMut<res::CursorPos>,
) {
	// assuming there is exactly one main camera entity, so this is OK
	let camera_transform = q_camera.iter().next().unwrap();

	for ev in evr_cursor.iter(&ev_cursor) {
		// get the size of the window that the event is for
		let wnd = wnds.get(ev.id).unwrap();
		let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

		// the default orthographic projection is in pixels from the center;
		// just undo the translation
		let p = ev.position - size / 2.0;

		// apply the camera transform
		let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
		cursor_pos.0 = pos_wld.xy();
		// eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);
	}
}
