use beet_examples::prelude::*;
use bevy::prelude::*;

pub fn main() {
	App::new()
		.add_plugins(running_beet_example_plugin)
		.add_systems(
			Startup,
			(
				beetmash::core::scenes::ui_terminal,
				beetmash::core::scenes::lighting_3d,
				beetmash::core::scenes::ground_3d,
				beet_examples::scenes::flow::beet_debug,
				beet_examples::scenes::spatial::seek_3d,
			),
		)
		.run();
}
