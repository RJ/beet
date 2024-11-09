use crate::prelude::*;

/// Immediately end the run with the provided result
pub type EndOnRun = TriggerOnTrigger<OnRunResult, OnRun>;


impl EndOnRun {
	pub fn success() -> Self { Self::new(OnRunResult::success()) }
	pub fn failure() -> Self { Self::new(OnRunResult::failure()) }
}


#[cfg(test)]
mod test {
	use super::EndOnRun;
	use crate::prelude::*;
	use anyhow::Result;
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use sweet::*;

	#[test]
	fn works() -> Result<()> {
		let mut app = App::new();
		app.add_plugins(ActionPlugin::<EndOnRun>::default());
		let world = app.world_mut();
		let func = observe_run_results(world);

		expect(world.entities().len()).to_be(1)?;
		let entity = world.spawn(EndOnRun::failure()).id();
		expect(world.entities().len()).to_be(2)?;
		world.flush();
		expect(world.entities().len()).to_be(3)?;
		expect(&func).not().to_have_been_called()?;
		world.trigger_targets(OnRun, entity);
		world.flush();
		expect(&func).to_have_been_called()?;
		expect(&func).to_have_returned_nth_with(0, &RunResult::Failure)?;
		Ok(())
	}
	#[test]
	fn works_with_run_on_spawn() -> Result<()> {
		let mut app = App::new();
		app.add_plugins(ActionPlugin::<EndOnRun>::default());
		let world = app.world_mut();
		let func = observe_run_results(world);

		world.spawn((RunOnSpawn, EndOnRun::failure()));
		world.run_system_once(run_on_spawn)?;
		world.flush();
		expect(world.entities().len()).to_be(3)?;
		expect(&func).to_have_been_called()?;
		expect(&func).to_have_returned_nth_with(0, &RunResult::Failure)?;
		Ok(())
	}
}
