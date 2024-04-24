use beet::prelude::*;
use bevy::prelude::*;
use forky_bevy::systems::close_on_esc;

#[derive(Component)]
pub struct FollowCursor;

/// Boilerplate for examples
pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
	fn build(&self, app: &mut App) {
		app
		.add_plugins(DefaultPlugins)
		.add_plugins(DefaultBeetPlugins::<CoreModule>::default())
		.add_systems(Startup, space_setup)
		.add_systems(Update, follow_cursor)
.add_systems(Update, close_on_esc)
		/*-*/;
	}
}


fn space_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// camera
	commands.spawn(Camera2dBundle::default());

	// background
	commands.spawn((
		SpriteBundle {
			texture: asset_server.load("space_background/Space_Stars2.png"),
			transform: Transform::from_translation(Vec3::new(0., 0., -1.))
				.with_scale(Vec3::splat(100.)),
			..default()
		},
		ImageScaleMode::Tiled {
			tile_x: true,
			tile_y: true,
			stretch_value: 0.01,
		},
	));
}

fn follow_cursor(
	camera_query: Query<(&Camera, &GlobalTransform)>,
	mut cursor_query: Query<&mut Transform, With<FollowCursor>>,
	windows: Query<&Window>,
) {
	let (camera, camera_transform) = camera_query.single();

	let Some(cursor_position) = windows.single().cursor_position() else {
		return;
	};

	let Some(point) =
		camera.viewport_to_world_2d(camera_transform, cursor_position)
	else {
		return;
	};

	for mut transform in cursor_query.iter_mut() {
		transform.translation = point.extend(0.);
	}
}
