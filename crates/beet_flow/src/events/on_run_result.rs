use crate::prelude::*;
use bevy::prelude::*;

#[rustfmt::skip]
pub mod child_expect {
	pub const NO_CHILDREN: &str = 
		"OnChildResult triggered but no children found";
	pub const NOT_MY_CHILD: &str =
		"OnChildResult triggered but caller not in children";
}


/// Signifies a behavior has stopped running.
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, Event, Reflect)]
#[reflect(Default)]
pub struct OnRunResult(RunResult);
impl OnRunResult {
	pub fn new(result: RunResult) -> Self { Self(result) }
	/// Populate with [`RunResult::Success`]
	pub fn success() -> Self { Self(RunResult::Success) }
	/// Populate with [`RunResult::Failure`]
	pub fn failure() -> Self { Self(RunResult::Failure) }
	pub fn result(&self) -> RunResult { **self }
}

// impl Event for OnRunResult {
// 	type Traversal = &'static Parent;
// 	const AUTO_PROPAGATE: bool = false;
// }



pub type OnChildResult = OnChildValue<RunResult>;
impl OnChildResult {
	pub fn success(child: Entity) -> Self {
		Self::new(child, RunResult::Success)
	}
	pub fn failure(child: Entity) -> Self {
		Self::new(child, RunResult::Failure)
	}
}
