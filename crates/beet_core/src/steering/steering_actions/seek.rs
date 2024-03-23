use crate::prelude::*;
use beet_ecs::prelude::*;
use bevy::prelude::*;
use forky_core::ResultTEExt;

#[derive_action]
#[action(graph_role=GraphRole::Agent)]
pub struct Seek;

// TODO if target has Velocity, pursue
fn seek(
	transforms: Query<&Transform>,
	mut targets: Query<(
		&Transform,
		&Velocity,
		&SteerTarget,
		&MaxSpeed,
		&MaxForce,
		&mut Impulse,
		Option<&ArriveRadius>,
	)>,
	query: Query<(&TargetAgent, &Seek), With<Running>>,
) {
	for (target, _) in query.iter() {
		if let Ok((
			transform,
			velocity,
			steer_target,
			max_speed,
			max_force,
			mut impulse,
			arrive_radius,
		)) = targets.get_mut(**target)
		// if agent has no steer_target thats ok
		{
			if let Some(target_position) = steer_target
				.position(&transforms)
				.ok_or(|e| log::warn!("{e}"))
			{
				*impulse = seek_impulse(
					&transform.translation,
					&velocity,
					&target_position,
					*max_speed,
					*max_force,
					arrive_radius.copied(),
				);
			}
		}
	}
}
