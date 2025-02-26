use crate::prelude::*;
use bevy::prelude::*;


/// An action that runs all of its children in order until one succeeds.
/// - If a child succeeds it succeed.
/// - If a child fails it will run the next child.
/// - If there are no more children to run it will succeed.
#[derive(Default, Component, Action, Reflect)]
#[reflect(Default, Component)]
#[category(ActionCategory::ChildBehaviors)]
#[observers(on_start, on_next)]
pub struct FallbackFlow;

fn on_start(
	trigger: Trigger<OnRun>,
	mut commands: Commands,
	query: Query<&Children>,
) {
	let children = query
		.get(trigger.entity())
		.expect(child_expect::NO_CHILDREN);
	if let Some(first_child) = children.iter().next() {
		commands.entity(*first_child).trigger(OnRun);
	} else {
		commands
			.entity(trigger.entity())
			.trigger(OnRunResult::success());
	}
}

fn on_next(
	trigger: Trigger<OnChildResult>,
	mut commands: Commands,
	query: Query<&Children>,
) {
	if *trigger.event().value() == RunResult::Success {
		commands
			.entity(trigger.entity())
			.trigger(OnRunResult::success());
		return;
	}
	let children = query
		.get(trigger.entity())
		.expect(child_expect::NO_CHILDREN);
	{
		let index = children
			.iter()
			.position(|&x| x == trigger.event().child())
			.expect(child_expect::NOT_MY_CHILD);
		if index == children.len() - 1 {
			commands
				.entity(trigger.entity())
				.trigger(OnRunResult::success());
		} else {
			commands.entity(children[index + 1]).trigger(OnRun);
		}
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use bevy::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		let mut app = App::new();
		app.add_plugins(ActionPlugin::<(FallbackFlow, EndOnRun)>::default());
		let world = app.world_mut();
		world.add_observer(bubble_run_result);

		let on_result = observe_trigger_names::<OnRunResult>(world);
		let on_run = observe_triggers::<OnRun>(world);

		world
			.spawn((Name::new("root"), FallbackFlow))
			.with_children(|parent| {
				parent.spawn((Name::new("child1"), EndOnRun::failure()));
				parent.spawn((Name::new("child2"), EndOnRun::success()));
			})
			.flush_trigger(OnRun);

		expect(&on_run).to_have_been_called_times(3);
		expect(&on_result).to_have_been_called_times(3);
		expect(&on_result).to_have_returned_nth_with(0, &"child1".to_string());
		expect(&on_result).to_have_returned_nth_with(1, &"child2".to_string());
		expect(&on_result).to_have_returned_nth_with(2, &"root".to_string());
	}
}
