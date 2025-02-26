use beet_examples::prelude::*;
use bevy::prelude::*;
use emby::prelude::*;

fn main() {
	App::new()
		.add_plugins((crate_test_beet_example_plugin, plugin_ml, EmbyPlugin))
		.add_systems(
			Startup,
			(
				setup,
				bevyhub::core::scenes::lighting_3d,
				bevyhub::core::scenes::ground_3d,
				bevyhub::core::scenes::ui_terminal_input,
				emby::scenes::spawn_barbarian,
				emby::scenes::phone_texture_camera_3d,
			),
		)
		.add_observer(emby::scenes::add_phone_render_texture_to_arm)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn((Camera3d::default(), Transform::from_xyz(0., 1.6, 5.)));
}
