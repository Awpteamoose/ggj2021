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
	clippy::similar_names,
	clippy::shadow_unrelated,
	clippy::clippy::too_many_arguments,
	clippy::must_use_candidate,
	dead_code
)]
#![feature(try_blocks, async_closure, iterator_fold_self, fn_traits, unboxed_closures)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use] pub mod prelude;
pub mod systems;
pub mod components;
pub mod resources;

use prelude::*;

#[derive(bevy_discovery::DiscoveryPlugin)]
struct DiscoveryPlugin;

#[bevy_main]
fn main() {
	use bevy::diagnostic::{PrintDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

	App::build()
		.add_startup_system(systems::setup.system())
		.add_resource(ClearColor(Color::rgb(0.251, 0.231, 0.278)))
		.add_plugins(DefaultPlugins)
		.add_system(bevy::input::system::exit_on_esc_system.system())
		// .add_plugin(PrintDiagnosticsPlugin::default())
		// .add_plugin(FrameTimeDiagnosticsPlugin::default())
		// .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.system())
		// .add_plugin(bevy_inspector_egui::InspectorPlugin::<systems::debug::Tweak>::new())
		.add_plugin(DiscoveryPlugin)
		.add_resource(res::CursorPos::default())
		.add_resource(res::SelectedDrawer::default())
		.add_resource(res::SelectedItem::default())
		.run();
}
