use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_utils::Duration;
use serde::Deserialize;
use serde::Serialize;


#[action(system=empty_action)]
#[derive(Default)]
pub struct EmptyAction;
pub fn empty_action() {}

// intentionally dont deref to avoid bugs.
#[action(system=set_run_result)]
#[derive(Default)]
pub struct SetRunResult(pub RunResult);

impl SetRunResult {
	pub fn new(result: RunResult) -> Self { Self(result) }
	pub fn success() -> Self { Self(RunResult::Success) }
	pub fn failure() -> Self { Self(RunResult::Failure) }
}

pub fn set_run_result(
	mut commands: Commands,
	mut query: Query<(Entity, &SetRunResult), With<Running>>,
) {
	for (entity, result) in query.iter_mut() {
		commands.entity(entity).insert(result.0);
	}
}

#[action(system=succeed_in_duration)]
pub struct SucceedInDuration {
	pub duration: Duration,
}

impl Default for SucceedInDuration {
	fn default() -> Self {
		Self {
			duration: Duration::from_secs(1),
		}
	}
}

pub fn succeed_in_duration(
	mut commands: Commands,
	mut query: Query<(Entity, &RunTimer, &SucceedInDuration), With<Running>>,
) {
	for (entity, timer, succeed_in_duration) in query.iter_mut() {
		if timer.last_started.elapsed() >= succeed_in_duration.duration {
			commands.entity(entity).insert(RunResult::Success);
		}
	}
}
