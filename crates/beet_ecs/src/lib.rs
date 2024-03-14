pub mod action;
pub mod ecs_nodes;
pub mod edge;
pub mod extensions;
pub mod graph;
pub mod node;
pub mod reflect;
pub mod ui;

// currently required for action_list! macro to work
extern crate self as beet;
extern crate self as beet_ecs;

pub mod prelude {

	pub use crate::action::*;
	pub use crate::ecs_nodes::actions::*;
	pub use crate::ecs_nodes::selectors::*;
	pub use crate::ecs_nodes::*;
	pub use crate::edge::*;
	pub use crate::extensions::*;
	pub use crate::graph::*;
	pub use crate::node::*;
	pub use crate::reflect::*;
	pub use crate::ui::*;
	pub use beet_ecs_macros::*;
}


pub mod exports {
	pub use bevy_app::prelude::App;
	pub use bevy_ecs;
	pub use bevy_ecs::prelude::*;
	pub use bevy_ecs::schedule::ScheduleLabel;
	pub use bevy_ecs::system::EntityCommands;
	pub use bevy_reflect;
	pub use bevy_reflect::FromReflect;
	pub use bevy_reflect::GetTypeRegistration;
	pub use bevy_reflect::Reflect;
	pub use bevy_reflect::TypePath;
	pub use bevy_reflect::TypeRegistry;
	pub use strum::IntoEnumIterator;
	pub use strum_macros::Display;
	pub use strum_macros::EnumIter;
}
