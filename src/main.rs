#![warn(
	clippy::pedantic,
	clippy::clone_on_ref_ptr,
	clippy::decimal_literal_representation,
	clippy::integer_division,
	clippy::todo,
	clippy::wrong_pub_self_convention
)]
#![cfg_attr(not(debug_assertions), warn(
	clippy::dbg_macro,
	clippy::use_debug,
	clippy::print_stdout,
	clippy::unimplemented,
))]
#![allow(
	clippy::cast_precision_loss,
	clippy::module_name_repetitions,
	clippy::default_trait_access,
	clippy::new_without_default,
	clippy::non_ascii_literal,
	clippy::too_many_lines,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::missing_errors_doc,
	clippy::wildcard_imports,
	clippy::doc_markdown,
	clippy::needless_pass_by_value,
	dead_code
)]
#![feature(try_blocks, async_closure, iterator_fold_self, fn_traits, unboxed_closures)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use] mod prelude;

use prelude::*;

struct Name(String);

struct Camera2d;

#[derive(Inspectable, Default)]
struct CameraTweak {
	tweak: bool,
	size: f32,
}

fn setup(
	commands: &mut Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut windows: ResMut<Windows>,
) {
	let window = windows.get_primary_mut().unwrap();
	window.set_resizable(false);
	window.set_mode(bevy::window::WindowMode::BorderlessFullscreen);

	let scale = 1./window.requested_width();

	commands
		.spawn(Camera2dBundle {
			transform: Transform::from_scale(Vec3::new(scale, scale, 1.)),
			..default()
		})
		.with(Camera2d)
		.spawn(CameraUiBundle::default())

		.spawn(TextBundle {
			text: Text {
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				value: "Score:".to_string(),
				style: TextStyle {
					color: Color::rgb(0.5, 0.5, 1.0),
					font_size: 40.0,
					..default()
				},
			},
			style: Style {
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px(5.0),
					left: Val::Px(5.0),
					..default()
				},
				..default()
			},
			..default()
		})

		.spawn(SpriteBundle {
			material: materials.add(Color::rgb(0.5, 0.5, 1.).into()),
			transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
			sprite: Sprite::new(Vec2::new(1., 1.)),
			..default()
		})

		// .spawn(SpriteBundle {
		//     material: materials.add(Color::rgb(1., 0.5, 0.5).into()),
		//     transform: Transform::from_translation(Vec3::new(1., 0., 0.)),
		//     sprite: Sprite::new(Vec2::new(100., 100.)),
		//     ..default()
		// })

		// .spawn(SpriteBundle {
		//     material: materials.add(Color::rgb(0.5, 1., 0.5).into()),
		//     transform: Transform::from_translation(Vec3::new(0., 1., 0.)),
		//     sprite: Sprite::new(Vec2::new(10., 10.)),
		//     ..default()
		// })
	;
}

#[system]
fn test_system(
	mut query: Query<&mut Text>,
	time: Res<Time>,
	mut camera: Query<(&Camera2d, &mut Transform)>,
	camera_tweak: Res<CameraTweak>,
) {
	for mut text in query.iter_mut() {
		text.value = time.delta_seconds().to_string();
	}

	if camera_tweak.tweak {
		for (_, mut transform) in camera.iter_mut() {
			transform.scale = Vec3::new(camera_tweak.size, camera_tweak.size, camera_tweak.size);
		}
	}
}

#[derive(bevy_discovery::DiscoveryPlugin)]
struct DiscoveryPlugin;

#[bevy_main]
fn main() {
	App::build()
		.add_startup_system(setup.system())
		.add_plugins(DefaultPlugins)
		.add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
		.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
		.add_system(bevy::diagnostic::PrintDiagnosticsPlugin::print_diagnostics_system.system())
		.add_plugin(bevy_inspector_egui::InspectorPlugin::<CameraTweak>::new())
		.add_plugin(DiscoveryPlugin)
		.run();
}
