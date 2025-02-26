use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
/// Runs before [`TickSet`] and In this set you can do things that need to happen before the tick.
pub struct PreTickSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
/// The set in which actions are run.
pub struct TickSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
/// Runs after [`TickSet`] and [`apply_deferred`].
pub struct PostTickSet;

#[derive(Debug, Clone, Default)]
/// Helpers that clean up run state, this is included in the [`LifecyclePlugin`]
pub struct LifecycleSystemsPlugin;

impl LifecycleSystemsPlugin {
	/// a brittle hack, keeps track of the number of observers added by the LifecycleSystemsPlugin
	/// for validating scene exports
	pub const NUM_OBSERVERS: u32 = 3;
}

impl Plugin for LifecycleSystemsPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<BeetConfig>();
		let config = app.world().resource::<BeetConfig>();
		let schedule = config.schedule.clone();

		app /*-*/
			.configure_sets(schedule, PreTickSet)
			.configure_sets(schedule, TickSet.after(PreTickSet))
			.configure_sets(schedule, PostTickSet.after(TickSet))
			.add_systems(
				schedule,
				set_root_as_target_entity.in_set(PreTickSet),
			)
			/*-*/;

		let world = app.world_mut();
		world.add_observer(bubble_run_result);
		world.add_observer(interrupt_on_run);
		world.add_observer(interrupt_on_run_result);
	}
}
