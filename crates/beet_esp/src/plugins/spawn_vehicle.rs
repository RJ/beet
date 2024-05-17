use beet::prelude::*;
use bevy::prelude::*;


pub fn spawn_obstacle_avoider(world: &mut World) {
	let behavior = avoid_obstacle_behavior(world);

	world
		.spawn((
			DualMotorValue::splat(MotorValue::stop()),
			DepthValue::default(),
		))
		.add_child(behavior);
}
